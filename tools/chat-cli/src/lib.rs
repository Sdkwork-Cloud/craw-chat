mod command;
mod realtime;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Method, Request};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use serde_json::{Value, json};
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

use command::{AuthInput, CommandContext, CommandOperation, sanitize_identifier};
pub use command::{CliCommand, CliError, CommandOutput, is_interactive_command, parse_cli_args};
use realtime::{
    CcpSocketCodec, RealtimeSocket, RealtimeSocketReader, RealtimeSocketWriter,
    connect_realtime_socket,
};

const CHAT_CONVERSATIONS_PATH: &str = "/im/v3/api/chat/conversations";
const LOCAL_TOKEN_HEADER: &str = r#"{"alg":"none","typ":"JWT"}"#;

pub async fn execute_command(command: CliCommand) -> Result<CommandOutput, CliError> {
    match command.operation {
        CommandOperation::Health => {
            let value =
                http_request_json(&command.context, Method::GET, "/healthz", None, false).await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Token {
            authorization_header,
        } => {
            let token = resolve_access_token(&command.context.auth)?;
            let authorization = format!("Bearer {token}");
            Ok(CommandOutput::Json(json!({
                "source": if command.context.auth.bearer_token.is_some() { "providedBearerToken" } else { "localDualToken" },
                "authorization": if authorization_header {
                    authorization
                } else {
                    token.clone()
                },
                "token": token,
                "claims": Value::Null
            })))
        }
        CommandOperation::CreateConversation {
            conversation_id,
            conversation_type,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                CHAT_CONVERSATIONS_PATH,
                Some(json!({
                    "conversationId": conversation_id,
                    "conversationType": conversation_type,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::AddMember {
            conversation_id,
            principal_id,
            principal_kind,
            role,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                format!("{CHAT_CONVERSATIONS_PATH}/{conversation_id}/members/add").as_str(),
                Some(json!({
                    "principalId": principal_id,
                    "principalKind": principal_kind,
                    "role": role,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Members { conversation_id } => {
            let value = http_request_json(
                &command.context,
                Method::GET,
                format!("{CHAT_CONVERSATIONS_PATH}/{conversation_id}/members").as_str(),
                None,
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::SendMessage {
            conversation_id,
            summary,
            text,
            client_msg_id,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                format!("{CHAT_CONVERSATIONS_PATH}/{conversation_id}/messages").as_str(),
                Some(json!({
                    "clientMsgId": client_msg_id,
                    "summary": summary,
                    "text": text,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(json!({
                "conversationId": conversation_id,
                "clientMsgId": client_msg_id,
                "summary": summary,
                "text": text,
                "result": value,
            })))
        }
        CommandOperation::Timeline { conversation_id } => {
            let value = http_request_json(
                &command.context,
                Method::GET,
                format!("{CHAT_CONVERSATIONS_PATH}/{conversation_id}/messages").as_str(),
                None,
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Watch {
            conversation_id,
            event_types,
            exit_after_events,
            idle_timeout,
        } => {
            let frames = watch_realtime_conversation(
                &command.context,
                conversation_id.as_str(),
                event_types.as_slice(),
                exit_after_events,
                idle_timeout,
            )
            .await?;
            Ok(CommandOutput::Frames(frames))
        }
        CommandOperation::ChatSession { .. } => Err(CliError::usage(
            "chat-session is interactive; execute it through the CLI binary or execute_interactive_command",
        )),
    }
}

pub async fn execute_interactive_command(command: CliCommand) -> Result<(), CliError> {
    execute_interactive_command_with_io(command, io::stdin(), io::stdout()).await
}

pub async fn execute_interactive_command_with_io<R, W>(
    command: CliCommand,
    input: R,
    output: W,
) -> Result<(), CliError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    match command.operation {
        CommandOperation::ChatSession {
            conversation_id,
            label,
            message_prefix,
            event_types,
            idle_timeout,
        } => {
            run_chat_session(
                &command.context,
                conversation_id.as_str(),
                label.as_deref(),
                message_prefix.as_deref(),
                event_types.as_slice(),
                idle_timeout,
                input,
                output,
            )
            .await
        }
        operation => {
            let output_value = execute_command(CliCommand {
                context: command.context,
                operation,
            })
            .await?;
            let rendered = render_output(&output_value)?;
            let mut writer = output;
            if !rendered.is_empty() {
                writer
                    .write_all(rendered.as_bytes())
                    .await
                    .map_err(|error| {
                        CliError::runtime(format!("failed to write rendered output: {error}"))
                    })?;
                writer.write_all(b"\n").await.map_err(|error| {
                    CliError::runtime(format!("failed to finalize rendered output: {error}"))
                })?;
                writer.flush().await.map_err(|error| {
                    CliError::runtime(format!("failed to flush rendered output: {error}"))
                })?;
            }
            Ok(())
        }
    }
}

pub fn render_output(output: &CommandOutput) -> Result<String, CliError> {
    match output {
        CommandOutput::Json(value) => serde_json::to_string_pretty(value)
            .map_err(|error| CliError::runtime(format!("failed to render json output: {error}"))),
        CommandOutput::Frames(values) => {
            let mut lines = Vec::with_capacity(values.len());
            for value in values {
                lines.push(serde_json::to_string(value).map_err(|error| {
                    CliError::runtime(format!("failed to render websocket frame: {error}"))
                })?);
            }
            Ok(lines.join("\n"))
        }
    }
}

async fn http_request_json(
    context: &CommandContext,
    method: Method,
    path: &str,
    body: Option<Value>,
    require_auth: bool,
) -> Result<Value, CliError> {
    let client = http_client();
    let uri = build_http_url(context.base_url.as_str(), path);
    let payload = if let Some(value) = body {
        serde_json::to_vec(&value)
            .map(Bytes::from)
            .map_err(|error| CliError::runtime(format!("failed to encode json body: {error}")))?
    } else {
        Bytes::new()
    };

    let mut request_builder = Request::builder()
        .method(method.clone())
        .uri(uri.as_str())
        .header(CONTENT_TYPE, "application/json");
    if require_auth {
        let access_token = resolve_access_token(&context.auth)?;
        let authorization = format!("Bearer {access_token}");
        request_builder = request_builder
            .header(AUTHORIZATION, authorization.as_str())
            .header("access-token", access_token.as_str());
    }

    let request = request_builder.body(Full::new(payload)).map_err(|error| {
        CliError::runtime(format!("failed to build request for {uri}: {error}"))
    })?;
    let response = client
        .request(request)
        .await
        .map_err(|error| {
            if error.is_connect() {
                return CliError::runtime(format!(
                    "unable to connect to sdkwork-im service at {} while calling {} {}; verify the service is running and the --base-url is correct: {}",
                    context.base_url,
                    method.as_str(),
                    path,
                    error
                ));
            }
            CliError::runtime(format!("request to {uri} failed: {error}"))
        })?;
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .map_err(|error| CliError::runtime(format!("failed to read response body: {error}")))?
        .to_bytes();

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(bytes.as_ref()).into_owned();
        return Err(CliError::runtime(format!(
            "{} {} failed with status {}: {}",
            method.as_str(),
            path,
            status.as_u16(),
            body_text
        )));
    }

    if bytes.is_empty() {
        return Ok(json!({}));
    }

    serde_json::from_slice(bytes.as_ref()).map_err(|error| {
        CliError::runtime(format!("response from {path} was not valid json: {error}"))
    })
}

async fn watch_realtime_conversation(
    context: &CommandContext,
    conversation_id: &str,
    event_types: &[String],
    exit_after_events: Option<usize>,
    idle_timeout: Option<Duration>,
) -> Result<Vec<Value>, CliError> {
    let mut socket = connect_realtime_socket(context).await?;

    let mut frames = Vec::new();
    frames.push(
        read_next_json_frame(&mut socket, idle_timeout)
            .await
            .map_err(|error| {
                CliError::runtime(format!(
                    "failed while waiting for realtime.connected: {error}"
                ))
            })?,
    );

    send_subscription_sync(&mut socket, conversation_id, event_types, "chat_cli_sync_1").await?;

    frames.push(
        read_next_json_frame(&mut socket, idle_timeout)
            .await
            .map_err(|error| {
                CliError::runtime(format!(
                    "failed while waiting for subscriptions.synced: {error}"
                ))
            })?,
    );

    let mut observed_event_windows = 0usize;
    loop {
        let frame = read_next_json_frame(&mut socket, idle_timeout)
            .await
            .map_err(|error| {
                CliError::runtime(format!(
                    "failed while waiting for event.window or events.acked: {error}"
                ))
            })?;
        let acked_seq = acked_seq_for_window(&frame);
        let is_event_window = frame["type"] == "event.window";
        frames.push(frame);

        if let Some(acked_seq) = acked_seq {
            send_events_ack(&mut socket, acked_seq).await?;
            frames.push(
                read_next_json_frame(&mut socket, idle_timeout)
                    .await
                    .map_err(|error| {
                        CliError::runtime(format!("failed while waiting for events.acked: {error}"))
                    })?,
            );
        }

        if is_event_window {
            observed_event_windows += 1;
            if let Some(limit) = exit_after_events
                && observed_event_windows >= limit
            {
                break;
            }
        }
    }

    socket.close().await;
    Ok(frames)
}

// This helper wires parsed CLI options and explicit IO handles into one chat
// session so tests and non-interactive entrypoints can reuse the same flow.
#[allow(clippy::too_many_arguments)]
async fn run_chat_session<R, W>(
    context: &CommandContext,
    conversation_id: &str,
    label: Option<&str>,
    message_prefix: Option<&str>,
    event_types: &[String],
    idle_timeout: Option<Duration>,
    input: R,
    mut output: W,
) -> Result<(), CliError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let socket = connect_realtime_socket(context).await?;
    let (mut ws_write, mut ws_read) = socket.split();
    let _ = read_next_json_frame_from_read(&mut ws_read, idle_timeout).await?;
    send_subscription_sync_write(
        &mut ws_write,
        conversation_id,
        event_types,
        "chat_cli_session_sync_1",
    )
    .await?;
    let _ = read_next_json_frame_from_read(&mut ws_read, idle_timeout).await?;

    let prompt_label = label.unwrap_or(context.auth.user_id.as_str()).to_owned();
    let mut lines = BufReader::new(input).lines();
    let mut pending_outgoing = Vec::<String>::new();

    write_chat_banner(&mut output, prompt_label.as_str(), conversation_id).await?;
    write_chat_prompt(&mut output, prompt_label.as_str()).await?;

    loop {
        let websocket_frame = read_next_json_frame_from_read(&mut ws_read, idle_timeout);
        tokio::pin!(websocket_frame);

        tokio::select! {
            line = lines.next_line() => {
                let Some(line) = line.map_err(|error| {
                    CliError::runtime(format!("failed to read chat-session input: {error}"))
                })? else {
                    break;
                };

                let trimmed = line.trim();
                if trimmed.is_empty() {
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                    continue;
                }

                if trimmed.eq_ignore_ascii_case("/quit") || trimmed.eq_ignore_ascii_case("/exit") {
                    write_chat_status_line(&mut output, "closing chat session").await?;
                    break;
                }

                if trimmed.eq_ignore_ascii_case("/help") {
                    write_chat_status_line(
                        &mut output,
                        "commands: /help, /quit, /exit",
                    ).await?;
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                    continue;
                }

                let sent_summary = apply_message_prefix(trimmed, message_prefix);
                let client_msg_id = next_chat_client_message_id(context.auth.user_id.as_str());
                send_message_request(
                    context,
                    conversation_id,
                    sent_summary.as_str(),
                    sent_summary.as_str(),
                    client_msg_id.as_str(),
                )
                .await?;
                pending_outgoing.push(sent_summary);
                write_chat_message_line(&mut output, prompt_label.as_str(), trimmed).await?;
                write_chat_prompt(&mut output, prompt_label.as_str()).await?;
            }
            frame = &mut websocket_frame => {
                let frame = frame?;
                if let Some(acked_seq) = acked_seq_for_window(&frame) {
                    send_events_ack_write(&mut ws_write, acked_seq).await?;
                }

                let mut printed_any = false;
                for summary in extract_window_summaries(&frame) {
                    if remove_pending_summary(&mut pending_outgoing, summary.as_str()) {
                        continue;
                    }

                    write_chat_status_line(&mut output, summary.as_str()).await?;
                    printed_any = true;
                }

                if printed_any {
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                }
            }
        }
    }

    ws_write.close().await;
    Ok(())
}

fn http_client() -> Client<HttpConnector, Full<Bytes>> {
    let connector = HttpConnector::new();
    Client::builder(TokioExecutor::new()).build(connector)
}

fn build_http_url(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), path)
}

fn build_websocket_url(base_url: &str, path: &str) -> Result<String, CliError> {
    let base = base_url.trim_end_matches('/');
    if let Some(rest) = base.strip_prefix("http://") {
        return Ok(format!("ws://{rest}{path}"));
    }
    if let Some(rest) = base.strip_prefix("https://") {
        return Ok(format!("wss://{rest}{path}"));
    }
    Err(CliError::usage(format!(
        "base url must start with http:// or https://: {base_url}"
    )))
}

pub(crate) fn resolve_access_token(auth: &AuthInput) -> Result<String, CliError> {
    if let Some(token) = auth
        .bearer_token
        .as_deref()
        .and_then(strip_bearer_prefix)
        .or(auth.bearer_token.as_deref())
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        return Ok(token.to_owned());
    }

    let payload = serde_json::to_vec(&json!({
        "tenant_id": auth.tenant_id,
        "organization_id": Value::Null,
        "login_scope": "TENANT",
        "user_id": auth.user_id,
        "actor_id": auth.user_id,
        "actor_kind": auth.actor_kind,
        "session_id": auth.session_id,
        "device_id": auth.device_id,
        "app_id": "sdkwork-im",
        "environment": "dev",
        "deployment_mode": "saas",
        "auth_level": "password",
        "permission_scope": auth.permissions,
        "data_scope": ["tenant"],
    }))
    .map_err(|error| CliError::runtime(format!("failed to encode local token payload: {error}")))?;
    Ok(format!(
        "{}.{}.local",
        URL_SAFE_NO_PAD.encode(LOCAL_TOKEN_HEADER.as_bytes()),
        URL_SAFE_NO_PAD.encode(payload)
    ))
}

pub(crate) fn resolve_authorization_header(auth: &AuthInput) -> Result<String, CliError> {
    resolve_access_token(auth).map(|token| format!("Bearer {token}"))
}

fn strip_bearer_prefix(value: &str) -> Option<&str> {
    let prefix = value.get(..7)?;
    if prefix.eq_ignore_ascii_case("Bearer ") {
        value.get(7..)
    } else {
        None
    }
}

async fn send_subscription_sync(
    socket: &mut RealtimeSocket,
    conversation_id: &str,
    event_types: &[String],
    request_id: &str,
) -> Result<(), CliError> {
    let payload = json!({
        "type": "subscriptions.sync",
        "requestId": request_id,
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": conversation_id,
                "eventTypes": event_types,
            }
        ]
    });
    let message = socket.ccp.encode_business_frame(
        "cc.realtime.subscriptions.sync.v1",
        "cmd",
        &payload,
    )?;
    socket
        .inner
        .send(message)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send subscription sync: {error}")))
}

async fn send_events_ack(socket: &mut RealtimeSocket, acked_seq: u64) -> Result<(), CliError> {
    let payload = json!({
        "type": "events.ack",
        "requestId": format!("chat_cli_ack_{acked_seq}"),
        "ackedSeq": acked_seq,
    });
    let message = socket
        .ccp
        .encode_business_frame("cc.realtime.events.ack.v1", "ack", &payload)?;
    socket
        .inner
        .send(message)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send events ack: {error}")))
}

async fn send_subscription_sync_write(
    writer: &mut RealtimeSocketWriter,
    conversation_id: &str,
    event_types: &[String],
    request_id: &str,
) -> Result<(), CliError> {
    let payload = json!({
        "type": "subscriptions.sync",
        "requestId": request_id,
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": conversation_id,
                "eventTypes": event_types,
            }
        ]
    });
    let message = writer.ccp.encode_business_frame(
        "cc.realtime.subscriptions.sync.v1",
        "cmd",
        &payload,
    )?;
    writer
        .inner
        .send(message)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send subscription sync: {error}")))
}

async fn send_events_ack_write(
    writer: &mut RealtimeSocketWriter,
    acked_seq: u64,
) -> Result<(), CliError> {
    let payload = json!({
        "type": "events.ack",
        "requestId": format!("chat_cli_ack_{acked_seq}"),
        "ackedSeq": acked_seq,
    });
    let message = writer
        .ccp
        .encode_business_frame("cc.realtime.events.ack.v1", "ack", &payload)?;
    writer
        .inner
        .send(message)
        .await
        .map_err(|error| CliError::runtime(format!("failed to send events ack: {error}")))
}

async fn send_message_request(
    context: &CommandContext,
    conversation_id: &str,
    summary: &str,
    text: &str,
    client_msg_id: &str,
) -> Result<Value, CliError> {
    http_request_json(
        context,
        Method::POST,
        format!("{CHAT_CONVERSATIONS_PATH}/{conversation_id}/messages").as_str(),
        Some(json!({
            "clientMsgId": client_msg_id,
            "summary": summary,
            "text": text,
        })),
        true,
    )
    .await
}

async fn read_next_json_frame(
    socket: &mut RealtimeSocket,
    idle_timeout: Option<Duration>,
) -> Result<Value, CliError> {
    loop {
        let next = if let Some(duration) = idle_timeout {
            timeout(duration, socket.inner.next())
                .await
                .map_err(|_| CliError::runtime("timed out waiting for realtime frame"))?
        } else {
            socket.inner.next().await
        };

        let message = next
            .ok_or_else(|| CliError::runtime("websocket closed before expected frame arrived"))?
            .map_err(|error| CliError::runtime(format!("websocket receive failed: {error}")))?;

        match message {
            Message::Text(_) | Message::Binary(_) => {
                return decode_realtime_business_message(&socket.ccp, message);
            }
            Message::Ping(payload) => {
                socket
                    .inner
                    .send(Message::Pong(payload))
                    .await
                    .map_err(|error| {
                        CliError::runtime(format!("failed to reply to websocket ping: {error}"))
                    })?;
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                let reason = frame
                    .map(|frame| format!("code={} reason={}", u16::from(frame.code), frame.reason))
                    .unwrap_or_else(|| "without close frame".to_owned());
                return Err(CliError::runtime(format!(
                    "websocket closed before expected frame arrived: {reason}"
                )));
            }
            Message::Frame(_) => {}
        }
    }
}

async fn read_next_json_frame_from_read(
    reader: &mut RealtimeSocketReader,
    idle_timeout: Option<Duration>,
) -> Result<Value, CliError> {
    loop {
        let next = if let Some(duration) = idle_timeout {
            timeout(duration, reader.inner.next())
                .await
                .map_err(|_| CliError::runtime("timed out waiting for realtime frame"))?
        } else {
            reader.inner.next().await
        };

        let message = next
            .ok_or_else(|| CliError::runtime("websocket closed before expected frame arrived"))?
            .map_err(|error| CliError::runtime(format!("websocket receive failed: {error}")))?;

        match message {
            Message::Text(_) | Message::Binary(_) => {
                return decode_realtime_business_message(&reader.ccp, message);
            }
            Message::Ping(_) | Message::Pong(_) => {}
            Message::Close(frame) => {
                let reason = frame
                    .map(|frame| format!("code={} reason={}", u16::from(frame.code), frame.reason))
                    .unwrap_or_else(|| "without close frame".to_owned());
                return Err(CliError::runtime(format!(
                    "websocket closed before expected frame arrived: {reason}"
                )));
            }
            Message::Frame(_) => {}
        }
    }
}

fn decode_realtime_business_message(
    ccp: &CcpSocketCodec,
    message: Message,
) -> Result<Value, CliError> {
    ccp.decode_business_json(message)
}

fn acked_seq_for_window(frame: &Value) -> Option<u64> {
    if frame.get("type") != Some(&Value::String("event.window".to_owned())) {
        return None;
    }

    if frame["window"]["items"]
        .as_array()
        .is_some_and(|items| items.is_empty())
    {
        return None;
    }

    frame["window"]["nextAfterSeq"]
        .as_u64()
        .and_then(|value| value.checked_sub(1))
}

fn extract_window_summaries(frame: &Value) -> Vec<String> {
    let mut summaries = Vec::new();

    let Some(items) = frame["window"]["items"].as_array() else {
        return summaries;
    };

    for item in items {
        let Some(payload_text) = item["payload"].as_str() else {
            continue;
        };

        let Ok(payload) = serde_json::from_str::<Value>(payload_text) else {
            continue;
        };

        if let Some(summary) = payload["summary"].as_str() {
            summaries.push(summary.to_owned());
        }
    }

    summaries
}

fn apply_message_prefix(message: &str, message_prefix: Option<&str>) -> String {
    match message_prefix {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}{message}"),
        _ => message.to_owned(),
    }
}

fn next_chat_client_message_id(user_id: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis();
    format!("chat_cli_{}_{}", sanitize_identifier(user_id), millis)
}

fn remove_pending_summary(pending: &mut Vec<String>, summary: &str) -> bool {
    if let Some(index) = pending.iter().position(|candidate| candidate == summary) {
        pending.remove(index);
        true
    } else {
        false
    }
}

async fn write_chat_banner<W>(
    output: &mut W,
    prompt_label: &str,
    conversation_id: &str,
) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    write_chat_status_line(
        output,
        format!("chat-session ready for {prompt_label} in {conversation_id}; type /quit to exit")
            .as_str(),
    )
    .await
}

async fn write_chat_prompt<W>(output: &mut W, prompt_label: &str) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    output
        .write_all(format!("{prompt_label}> ").as_bytes())
        .await
        .map_err(|error| CliError::runtime(format!("failed to write chat prompt: {error}")))?;
    output
        .flush()
        .await
        .map_err(|error| CliError::runtime(format!("failed to flush chat prompt: {error}")))
}

async fn write_chat_status_line<W>(output: &mut W, line: &str) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    output
        .write_all(format!("{line}\n").as_bytes())
        .await
        .map_err(|error| CliError::runtime(format!("failed to write chat line: {error}")))?;
    output
        .flush()
        .await
        .map_err(|error| CliError::runtime(format!("failed to flush chat line: {error}")))
}

async fn write_chat_message_line<W>(
    output: &mut W,
    prompt_label: &str,
    message: &str,
) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    write_chat_status_line(output, format!("[{prompt_label}] {message}").as_str()).await
}
