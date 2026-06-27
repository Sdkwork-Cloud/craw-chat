use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:18079";
const DEFAULT_TENANT_ID: &str = "100001";
const DEFAULT_USER_ID: &str = "1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    message: String,
    exit_code: i32,
}

impl CliError {
    pub(crate) fn help(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 0,
        }
    }

    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    pub fn runtime(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandOutput {
    Json(Value),
    Frames(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommand {
    pub(crate) context: CommandContext,
    pub(crate) operation: CommandOperation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandContext {
    pub(crate) base_url: String,
    pub(crate) auth: AuthInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AuthInput {
    pub(crate) tenant_id: String,
    pub(crate) user_id: String,
    pub(crate) actor_kind: String,
    pub(crate) session_id: String,
    pub(crate) device_id: String,
    pub(crate) permissions: Vec<String>,
    pub(crate) bearer_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CommandOperation {
    Health,
    Token {
        authorization_header: bool,
    },
    CreateConversation {
        conversation_id: String,
        conversation_type: String,
    },
    AddMember {
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: String,
    },
    Members {
        conversation_id: String,
    },
    SendMessage {
        conversation_id: String,
        summary: Option<String>,
        text: Option<String>,
        client_msg_id: Option<String>,
    },
    Timeline {
        conversation_id: String,
    },
    Watch {
        conversation_id: String,
        event_types: Vec<String>,
        exit_after_events: Option<usize>,
        idle_timeout: Option<Duration>,
    },
    ChatSession {
        conversation_id: String,
        label: Option<String>,
        message_prefix: Option<String>,
        event_types: Vec<String>,
        idle_timeout: Option<Duration>,
    },
}

#[derive(Debug, Default, Clone)]
struct GlobalOptions {
    base_url: Option<String>,
    tenant_id: Option<String>,
    user_id: Option<String>,
    actor_kind: Option<String>,
    session_id: Option<String>,
    device_id: Option<String>,
    permissions: Option<Vec<String>>,
    bearer_token: Option<String>,
}

pub fn parse_cli_args<I, S>(args: I) -> Result<CliCommand, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let values = args.into_iter().map(Into::into).collect::<Vec<_>>();
    if values.is_empty() {
        return Err(CliError::help(cli_usage()));
    }

    let mut cursor = ArgCursor::new(values.into_iter().skip(1).collect::<Vec<_>>());
    if cursor.is_empty() {
        return Err(CliError::help(cli_usage()));
    }

    let mut global = GlobalOptions::default();

    while let Some(next) = cursor.peek() {
        if is_command_name(next) {
            break;
        }

        match next {
            "-h" | "--help" => return Err(CliError::help(cli_usage())),
            "--base-url" => {
                cursor.next();
                global.base_url = Some(cursor.required_value("--base-url")?);
            }
            "--tenant-id" => {
                cursor.next();
                global.tenant_id = Some(cursor.required_value("--tenant-id")?);
            }
            "--user-id" => {
                cursor.next();
                global.user_id = Some(cursor.required_value("--user-id")?);
            }
            "--actor-kind" => {
                cursor.next();
                global.actor_kind = Some(cursor.required_value("--actor-kind")?);
            }
            "--session-id" => {
                cursor.next();
                global.session_id = Some(cursor.required_value("--session-id")?);
            }
            "--device-id" => {
                cursor.next();
                global.device_id = Some(cursor.required_value("--device-id")?);
            }
            "--permissions" => {
                cursor.next();
                global.permissions =
                    Some(parse_permissions(cursor.required_value("--permissions")?));
            }
            "--bearer-token" => {
                cursor.next();
                global.bearer_token = Some(cursor.required_value("--bearer-token")?);
            }
            other if other.starts_with('-') => {
                return Err(CliError::usage(format!(
                    "unknown global flag: {other}\n\n{}",
                    cli_usage()
                )));
            }
            other => {
                return Err(CliError::usage(format!(
                    "unknown command: {other}\n\n{}",
                    cli_usage()
                )));
            }
        }
    }

    let Some(command_name) = cursor.next() else {
        return Err(CliError::help(cli_usage()));
    };

    let context = build_command_context(global);
    let operation = parse_command_operation(command_name.as_str(), &mut cursor)?;

    if !cursor.is_empty() {
        return Err(CliError::usage(format!(
            "unexpected trailing arguments: {}\n\n{}",
            cursor.remaining().join(" "),
            cli_usage()
        )));
    }

    Ok(CliCommand { context, operation })
}

fn parse_command_operation(
    command_name: &str,
    cursor: &mut ArgCursor,
) -> Result<CommandOperation, CliError> {
    match command_name {
        "health" => Ok(CommandOperation::Health),
        "token" => {
            let mut authorization_header = true;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(token_usage())),
                    "--token-only" => {
                        cursor.next();
                        authorization_header = false;
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown token flag: {other}\n\n{}",
                            token_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Token {
                authorization_header,
            })
        }
        "create-conversation" => {
            let mut conversation_id = None;
            let mut conversation_type = Some("group".to_owned());
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(create_conversation_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--conversation-type" => {
                        cursor.next();
                        conversation_type = Some(cursor.required_value("--conversation-type")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown create-conversation flag: {other}\n\n{}",
                            create_conversation_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::CreateConversation {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for create-conversation",
                )?,
                conversation_type: conversation_type.unwrap_or_else(|| "group".to_owned()),
            })
        }
        "add-member" => {
            let mut conversation_id = None;
            let mut principal_id = None;
            let mut principal_kind = Some("user".to_owned());
            let mut role = Some("member".to_owned());
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(add_member_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--principal-id" => {
                        cursor.next();
                        principal_id = Some(cursor.required_value("--principal-id")?);
                    }
                    "--principal-kind" => {
                        cursor.next();
                        principal_kind = Some(cursor.required_value("--principal-kind")?);
                    }
                    "--role" => {
                        cursor.next();
                        role = Some(cursor.required_value("--role")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown add-member flag: {other}\n\n{}",
                            add_member_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::AddMember {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for add-member",
                )?,
                principal_id: required_field(
                    principal_id,
                    "--principal-id is required for add-member",
                )?,
                principal_kind: principal_kind.unwrap_or_else(|| "user".to_owned()),
                role: role.unwrap_or_else(|| "member".to_owned()),
            })
        }
        "members" => {
            let mut conversation_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(members_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown members flag: {other}\n\n{}",
                            members_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Members {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for members",
                )?,
            })
        }
        "send-message" => {
            let mut conversation_id = None;
            let mut summary = None;
            let mut text = None;
            let mut client_msg_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(send_message_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--summary" => {
                        cursor.next();
                        summary = Some(cursor.required_value("--summary")?);
                    }
                    "--text" => {
                        cursor.next();
                        text = Some(cursor.required_value("--text")?);
                    }
                    "--client-msg-id" => {
                        cursor.next();
                        client_msg_id = Some(cursor.required_value("--client-msg-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown send-message flag: {other}\n\n{}",
                            send_message_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if summary.is_none() && text.is_none() {
                return Err(CliError::usage(format!(
                    "send-message requires --summary, --text, or both\n\n{}",
                    send_message_usage()
                )));
            }

            Ok(CommandOperation::SendMessage {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for send-message",
                )?,
                summary,
                text,
                client_msg_id,
            })
        }
        "timeline" => {
            let mut conversation_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(timeline_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown timeline flag: {other}\n\n{}",
                            timeline_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Timeline {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for timeline",
                )?,
            })
        }
        "watch" => {
            let mut conversation_id = None;
            let mut event_types = Vec::new();
            let mut exit_after_events = None;
            let mut idle_timeout = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(watch_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--event-type" => {
                        cursor.next();
                        event_types.push(cursor.required_value("--event-type")?);
                    }
                    "--exit-after-events" => {
                        cursor.next();
                        exit_after_events = Some(parse_usize(
                            cursor.required_value("--exit-after-events")?.as_str(),
                            "--exit-after-events",
                        )?);
                    }
                    "--idle-timeout-seconds" => {
                        cursor.next();
                        let seconds = parse_u64(
                            cursor.required_value("--idle-timeout-seconds")?.as_str(),
                            "--idle-timeout-seconds",
                        )?;
                        idle_timeout = Some(Duration::from_secs(seconds));
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown watch flag: {other}\n\n{}",
                            watch_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if event_types.is_empty() {
                event_types.push("message.posted".to_owned());
            }

            Ok(CommandOperation::Watch {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for watch",
                )?,
                event_types,
                exit_after_events,
                idle_timeout,
            })
        }
        "chat-session" => {
            let mut conversation_id = None;
            let mut label = None;
            let mut message_prefix = None;
            let mut event_types = Vec::new();
            let mut idle_timeout = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(chat_session_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--label" => {
                        cursor.next();
                        label = Some(cursor.required_value("--label")?);
                    }
                    "--message-prefix" => {
                        cursor.next();
                        message_prefix = Some(cursor.required_value("--message-prefix")?);
                    }
                    "--event-type" => {
                        cursor.next();
                        event_types.push(cursor.required_value("--event-type")?);
                    }
                    "--idle-timeout-seconds" => {
                        cursor.next();
                        let seconds = parse_u64(
                            cursor.required_value("--idle-timeout-seconds")?.as_str(),
                            "--idle-timeout-seconds",
                        )?;
                        idle_timeout = Some(Duration::from_secs(seconds));
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown chat-session flag: {other}\n\n{}",
                            chat_session_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if event_types.is_empty() {
                event_types.push("message.posted".to_owned());
            }

            Ok(CommandOperation::ChatSession {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for chat-session",
                )?,
                label,
                message_prefix,
                event_types,
                idle_timeout,
            })
        }
        other => Err(CliError::usage(format!(
            "unknown command: {other}\n\n{}",
            cli_usage()
        ))),
    }
}

pub fn is_interactive_command(command: &CliCommand) -> bool {
    matches!(command.operation, CommandOperation::ChatSession { .. })
}

fn build_command_context(global: GlobalOptions) -> CommandContext {
    let user_id = global
        .user_id
        .or_else(|| std::env::var("SDKWORK_IM_CLI_USER_ID").ok())
        .unwrap_or_else(|| DEFAULT_USER_ID.to_owned());
    let sanitized_user_id = sanitize_identifier(user_id.as_str());
    let tenant_id = global
        .tenant_id
        .or_else(|| std::env::var("SDKWORK_IM_CLI_TENANT_ID").ok())
        .unwrap_or_else(|| DEFAULT_TENANT_ID.to_owned());
    let actor_kind = global
        .actor_kind
        .or_else(|| std::env::var("SDKWORK_IM_CLI_ACTOR_KIND").ok())
        .unwrap_or_else(|| "user".to_owned());
    let session_id = global
        .session_id
        .or_else(|| std::env::var("SDKWORK_IM_CLI_SESSION_ID").ok())
        .unwrap_or_else(|| format!("s_{sanitized_user_id}"));
    let device_id = global
        .device_id
        .or_else(|| std::env::var("SDKWORK_IM_CLI_DEVICE_ID").ok())
        .unwrap_or_else(|| format!("d_{sanitized_user_id}"));
    let permissions = global
        .permissions
        .or_else(|| {
            std::env::var("SDKWORK_IM_CLI_PERMISSIONS")
                .ok()
                .map(parse_permissions)
        })
        .unwrap_or_default();
    let base_url = global
        .base_url
        .or_else(|| std::env::var("SDKWORK_IM_BASE_URL").ok())
        .or_else(resolve_base_url_from_config)
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned());

    CommandContext {
        base_url,
        auth: AuthInput {
            tenant_id,
            user_id,
            actor_kind,
            session_id,
            device_id,
            permissions,
            bearer_token: global.bearer_token,
        },
    }
}

fn resolve_base_url_from_config() -> Option<String> {
    let config_path = find_local_env_file()?;
    if let Some(http_url) = read_env_file_value(
        config_path.as_path(),
        "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
    ) {
        return Some(http_url.trim_end_matches('/').to_owned());
    }
    let bind_address = read_env_file_value(
        config_path.as_path(),
        "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
    )?;
    Some(bind_address_to_base_url(bind_address.as_str()))
}

fn find_local_env_file() -> Option<PathBuf> {
    env_file_candidates()
        .into_iter()
        .find(|candidate| candidate.is_file())
}

fn env_file_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        for ancestor in current_dir.ancestors() {
            candidates.push(
                ancestor
                    .join("configs")
                    .join("topology")
                    .join("self-hosted.split-services.development.env"),
            );
        }
    }

    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(repo_root) = manifest_root.ancestors().nth(2) {
        candidates.push(
            repo_root
                .join("configs")
                .join("topology")
                .join("self-hosted.split-services.development.env"),
        );
    }

    candidates.sort();
    candidates.dedup();
    candidates
}

fn read_env_file_value(path: &Path, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((current_key, current_value)) = trimmed.split_once('=') else {
            continue;
        };
        if current_key.trim() == key {
            let value = current_value.trim();
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

pub(crate) fn sanitize_identifier(raw: &str) -> String {
    let mut sanitized = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    if sanitized.is_empty() {
        "user".to_owned()
    } else {
        sanitized
    }
}

fn bind_address_to_base_url(bind_address: &str) -> String {
    let mut parts = bind_address.rsplitn(2, ':');
    let port = parts.next().unwrap_or("18079").trim();
    let host = parts.next().unwrap_or("127.0.0.1").trim();
    let normalized_host = match host {
        "" | "0.0.0.0" | "::" | "[::]" => "127.0.0.1",
        other => other,
    };
    format!("http://{normalized_host}:{port}")
}

fn parse_permissions(raw: String) -> Vec<String> {
    raw.split(|ch: char| ch == ',' || ch.is_whitespace())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
}

fn parse_usize(raw: &str, flag: &str) -> Result<usize, CliError> {
    raw.parse::<usize>()
        .map_err(|_| CliError::usage(format!("{flag} must be a positive integer")))
}

fn parse_u64(raw: &str, flag: &str) -> Result<u64, CliError> {
    raw.parse::<u64>()
        .map_err(|_| CliError::usage(format!("{flag} must be a positive integer")))
}

fn required_field<T>(value: Option<T>, message: &str) -> Result<T, CliError> {
    value.ok_or_else(|| CliError::usage(message))
}

fn is_command_name(value: &str) -> bool {
    matches!(
        value,
        "health"
            | "token"
            | "create-conversation"
            | "add-member"
            | "members"
            | "send-message"
            | "timeline"
            | "watch"
            | "chat-session"
    )
}

fn cli_usage() -> String {
    format!(
        concat!(
            "Usage: sdkwork-im-cli [global options] <command> [command options]\n\n",
            "Global options:\n",
            "  --base-url <url>                 Service base url. Default: {base}\n",
            "  --tenant-id <id>                 Tenant id. Default: {tenant}\n",
            "  --user-id <id>                   User id. Default: {user}\n",
            "  --actor-kind <kind>              Actor kind. Default: user\n",
            "  --session-id <id>                Session id. Default: s_<user-id>\n",
            "  --device-id <id>                 Device id. Default: d_<user-id>\n",
            "  --permissions <csv>              Optional principal permissions header\n",
            "  --bearer-token <token>           Forward an upstream bearer token when available\n",
            "  -h, --help                       Show help\n\n",
            "Commands:\n",
            "  health\n",
            "  token [--token-only]\n",
            "  create-conversation --conversation-id <id> [--conversation-type <type>]\n",
            "  add-member --conversation-id <id> --principal-id <id> [--principal-kind user] [--role member]\n",
            "  members --conversation-id <id>\n",
            "  send-message --conversation-id <id> [--summary <text>] [--text <text>] [--client-msg-id <id>]\n",
            "  timeline --conversation-id <id>\n",
            "  watch --conversation-id <id> [--event-type <type>]... [--exit-after-events <n>] [--idle-timeout-seconds <n>]\n\n",
            "  chat-session --conversation-id <id> [--label <name>] [--message-prefix <prefix>] [--event-type <type>]... [--idle-timeout-seconds <n>]\n\n",
            "Example:\n",
            "  sdkwork-im-cli --user-id 1 --device-id d_owner --session-id s_owner create-conversation --conversation-id c_demo --conversation-type group\n"
        ),
        base = DEFAULT_BASE_URL,
        tenant = DEFAULT_TENANT_ID,
        user = DEFAULT_USER_ID
    )
}

fn token_usage() -> String {
    "Usage: sdkwork-im-cli [global options] token [--token-only]".to_owned()
}

fn create_conversation_usage() -> String {
    "Usage: sdkwork-im-cli [global options] create-conversation --conversation-id <id> [--conversation-type <type>]".to_owned()
}

fn add_member_usage() -> String {
    "Usage: sdkwork-im-cli [global options] add-member --conversation-id <id> --principal-id <id> [--principal-kind <kind>] [--role <role>]".to_owned()
}

fn members_usage() -> String {
    "Usage: sdkwork-im-cli [global options] members --conversation-id <id>".to_owned()
}

fn send_message_usage() -> String {
    "Usage: sdkwork-im-cli [global options] send-message --conversation-id <id> [--summary <text>] [--text <text>] [--client-msg-id <id>]".to_owned()
}

fn timeline_usage() -> String {
    "Usage: sdkwork-im-cli [global options] timeline --conversation-id <id>".to_owned()
}

fn watch_usage() -> String {
    "Usage: sdkwork-im-cli [global options] watch --conversation-id <id> [--event-type <type>]... [--exit-after-events <n>] [--idle-timeout-seconds <n>]".to_owned()
}

fn chat_session_usage() -> String {
    "Usage: sdkwork-im-cli [global options] chat-session --conversation-id <id> [--label <name>] [--message-prefix <prefix>] [--event-type <type>]... [--idle-timeout-seconds <n>]".to_owned()
}

#[derive(Debug, Clone)]
struct ArgCursor {
    items: Vec<String>,
    index: usize,
}

impl ArgCursor {
    fn new(items: Vec<String>) -> Self {
        Self { items, index: 0 }
    }

    fn is_empty(&self) -> bool {
        self.index >= self.items.len()
    }

    fn peek(&self) -> Option<&str> {
        self.items.get(self.index).map(String::as_str)
    }

    fn next(&mut self) -> Option<String> {
        if self.index >= self.items.len() {
            None
        } else {
            let value = self.items[self.index].clone();
            self.index += 1;
            Some(value)
        }
    }

    fn required_value(&mut self, flag: &str) -> Result<String, CliError> {
        match self.next() {
            Some(value) => Ok(value),
            None => Err(CliError::usage(format!("{flag} requires a value"))),
        }
    }

    fn remaining(&self) -> &[String] {
        &self.items[self.index..]
    }
}

#[cfg(test)]
mod tests {
    use super::read_env_file_value;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_env_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("sdkwork_im_cli_{name}_{unique}.env"))
    }

    #[test]
    fn test_read_env_file_value_skips_malformed_lines_before_valid_key() {
        let path = temp_env_path("malformed_env");
        fs::write(
            &path,
            "\
# comment
NOT_A_VALID_ENV_LINE
SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND=127.0.0.1:18124
",
        )
        .expect("temp env file should be written");

        let value =
            read_env_file_value(path.as_path(), "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");

        let _ = fs::remove_file(&path);
        assert_eq!(value.as_deref(), Some("127.0.0.1:18124"));
    }
}
