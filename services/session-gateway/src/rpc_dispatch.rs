//! gRPC runtime dispatch for session-gateway Presence and Realtime services.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::http::{HeaderMap, HeaderValue, header};
use im_app_context::AppContext;
use im_domain_core::presence::PresenceSnapshotView;
use im_domain_core::realtime::{RealtimeEvent, RealtimeSubscriptionSnapshot};
use prost::Message;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    AckRealtimeEventsRequest, AckRealtimeEventsResponse, CreatePresenceHeartbeatRequest,
    CreatePresenceHeartbeatResponse, ListRealtimeEventsRequest, ListRealtimeEventsResponse,
    PresenceView, RealtimeEventView, RealtimeSubscription, RetrieveMyPresenceRequest,
    RetrieveMyPresenceResponse, SyncRealtimeSubscriptionsRequest, SyncRealtimeSubscriptionsResponse,
    WatchPresenceRequest, WatchPresenceResponse, WatchRealtimeEventsRequest,
    WatchRealtimeEventsResponse,
};
use sdkwork_im_rpc_service_rust::{
    ImRpcBoxFuture, ImRpcBoxStream, ImRpcError, ImRpcRuntimeDispatcher, ImRpcStreamRequest,
    ImRpcStreamResponse, ImRpcUnaryRequest, ImRpcUnaryResponse, RpcMetadata,
    admit_app_unary_request, require_app_session_auth,
};
use tokio_stream::{StreamExt as _, wrappers::IntervalStream};

use crate::{
    ApiError, AppState, RealtimeSubscriptionItemInput, bootstrap_realtime_plane_from_env,
    resolve_iam_auth_pool_from_env, resolve_request_app_context, resolve_requested_device_id,
};

pub const SESSION_GATEWAY_RPC_SERVICE_KEYS: &[&str] = &[
    "sdkwork.communication.app.v3.PresenceService",
    "sdkwork.communication.app.v3.RealtimeService",
];

const PRESENCE_WATCH_POLL_INTERVAL: Duration = Duration::from_secs(2);
const REALTIME_WATCH_POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct SessionGatewayRpcDispatcher {
    state: AppState,
}

impl SessionGatewayRpcDispatcher {
    pub async fn bootstrap_from_env() -> Result<Self, String> {
        let mut bootstrap = bootstrap_realtime_plane_from_env().await?;
        if bootstrap.iam_auth_pool.is_none() {
            bootstrap.iam_auth_pool = resolve_iam_auth_pool_from_env().await;
        }
        Ok(Self {
            state: AppState::from_realtime_bootstrap(&bootstrap),
        })
    }

    pub fn from_app_state(state: AppState) -> Self {
        Self { state }
    }
}

impl ImRpcRuntimeDispatcher for SessionGatewayRpcDispatcher {
    fn dispatch_unary(
        &self,
        request: ImRpcUnaryRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>> {
        let state = self.state.clone();
        Box::pin(async move {
            admit_app_unary_request(request.binding, &request.metadata)?;
            let auth = resolve_auth(&state, &request.metadata).await?;
            match request.binding.operation_id {
                "presence.heartbeat.create" => {
                    let payload = CreatePresenceHeartbeatRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_presence_heartbeat(&state, &auth, payload).await
                }
                "presence.me.retrieve" => {
                    let payload = RetrieveMyPresenceRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_presence_me(&state, &auth, payload).await
                }
                "realtime.subscriptions.sync" => {
                    let payload =
                        SyncRealtimeSubscriptionsRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_sync_subscriptions(&state, &auth, payload).await
                }
                "realtime.events.list" => {
                    let payload = ListRealtimeEventsRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_list_events(&state, &auth, payload).await
                }
                "realtime.events.ack" => {
                    let payload = AckRealtimeEventsRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_ack_events(&state, &auth, payload).await
                }
                other => Err(ImRpcError::unimplemented(format!(
                    "session-gateway rpc host does not implement unary operation `{other}`"
                ))),
            }
        })
    }

    fn dispatch_server_stream(
        &self,
        request: ImRpcStreamRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcBoxStream<Result<ImRpcStreamResponse, ImRpcError>>, ImRpcError>>
    {
        let state = self.state.clone();
        Box::pin(async move {
            require_app_session_auth(request.binding, &request.metadata)?;
            let auth = resolve_auth(&state, &request.metadata).await?;
            match request.binding.operation_id {
                "presence.watch" => {
                    let payload = WatchPresenceRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_watch_presence(&state, &auth, payload).await
                }
                "realtime.events.watch" => {
                    let payload = WatchRealtimeEventsRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_watch_realtime_events(&state, &auth, payload).await
                }
                other => Err(ImRpcError::unimplemented(format!(
                    "session-gateway rpc host does not implement stream operation `{other}`"
                ))),
            }
        })
    }
}

async fn resolve_auth(state: &AppState, metadata: &RpcMetadata) -> Result<AppContext, ImRpcError> {
    let headers = metadata_to_axum_headers(metadata);
    resolve_request_app_context(None, &headers, state.rpc_auth_resolver())
        .await
        .map_err(map_api_error)
}

async fn dispatch_presence_heartbeat(
    state: &AppState,
    auth: &AppContext,
    request: CreatePresenceHeartbeatRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let device_id = resolve_requested_device_id(auth, optional_string(request.device_id))
        .map_err(map_runtime_error)?;
    state
        .rpc_prepare_active_client_route(auth, device_id.as_str(), "grpc")
        .map_err(map_runtime_error)?;
    let sync_state = state
        .rpc_client_route_state_snapshot(auth, Some(device_id.as_str()))
        .map_err(map_runtime_error)?;
    let snapshot = state
        .rpc_presence_runtime()
        .heartbeat(
            auth,
            device_id,
            sync_state.latest_sync_seq,
            sync_state.registered_route_keys,
        )
        .map_err(map_runtime_error)?;
    let response = CreatePresenceHeartbeatResponse {
        presence: Some(presence_view_from_snapshot(
            &snapshot,
            snapshot.current_device_id.as_deref(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_presence_me(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveMyPresenceRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let device_id = optional_string(request.device_id);
    let sync_state = state
        .rpc_client_route_state_snapshot(auth, device_id.as_deref())
        .map_err(map_runtime_error)?;
    let snapshot = state
        .rpc_presence_runtime()
        .presence_snapshot(auth, device_id, sync_state.registered_route_keys)
        .map_err(map_runtime_error)?;
    let response = RetrieveMyPresenceResponse {
        presence: Some(presence_view_from_snapshot(
            &snapshot,
            snapshot.current_device_id.as_deref(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_sync_subscriptions(
    state: &AppState,
    auth: &AppContext,
    request: SyncRealtimeSubscriptionsRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let device_id = resolve_requested_device_id(auth, None).map_err(map_api_error)?;
    let items = proto_subscriptions_to_items(request.subscriptions);
    state
        .rpc_realtime_runtime()
        .validate_subscriptions_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            &items,
        )
        .map_err(map_runtime_error)?;
    state
        .rpc_prepare_active_client_route(auth, device_id.as_str(), "grpc")
        .map_err(map_runtime_error)?;
    let snapshot = state
        .rpc_realtime_runtime()
        .sync_subscriptions_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
            items,
        )
        .map_err(map_runtime_error)?;
    let response = SyncRealtimeSubscriptionsResponse {
        subscriptions: snapshot_to_proto_subscriptions(&snapshot),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_list_events(
    state: &AppState,
    auth: &AppContext,
    request: ListRealtimeEventsRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let device_id = resolve_requested_device_id(auth, None).map_err(map_api_error)?;
    let after_seq = parse_cursor_u64(request.cursor.as_str())?;
    let limit = request
        .page
        .and_then(|page| usize::try_from(page.page_size).ok())
        .filter(|value| *value > 0)
        .unwrap_or(100);
    crate::realtime::validate_realtime_event_limit(limit).map_err(map_runtime_error)?;
    state
        .rpc_prepare_active_client_route(auth, device_id.as_str(), "grpc_poll")
        .map_err(map_runtime_error)?;
    let window = state
        .rpc_realtime_runtime()
        .list_events_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
            after_seq,
            limit,
        )
        .map_err(map_runtime_error)?;
    let response = ListRealtimeEventsResponse {
        events: window.items.iter().map(realtime_event_to_proto).collect(),
        page: None,
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_ack_events(
    state: &AppState,
    auth: &AppContext,
    request: AckRealtimeEventsRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let device_id = resolve_requested_device_id(auth, None).map_err(map_api_error)?;
    let acked_seq = parse_cursor_u64(request.cursor.as_str())?;
    let previous_route = state.rpc_current_active_client_route(auth, device_id.as_str());
    state
        .rpc_prepare_active_client_route(auth, device_id.as_str(), "grpc")
        .map_err(map_runtime_error)?;
    let bound_route = state.rpc_current_active_client_route(auth, device_id.as_str());
    let ack_result = state.rpc_realtime_runtime().ack_events_for_principal_kind(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id.as_str(),
        acked_seq,
    );
    let ack = match ack_result {
        Ok(ack) => ack,
        Err(error) => {
            match (previous_route, bound_route) {
                (Some(previous_route), Some(bound_route)) => {
                    state.rpc_restore_active_client_route_if_current(&bound_route, previous_route);
                }
                (None, _) => {
                    state.rpc_release_active_client_route_if_current_session(auth, device_id.as_str());
                }
                _ => {}
            }
            return Err(map_api_error(error.into()));
        }
    };
    let response = AckRealtimeEventsResponse {
        ack_cursor: ack.acked_through_seq.to_string(),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_watch_presence(
    state: &AppState,
    auth: &AppContext,
    request: WatchPresenceRequest,
) -> Result<ImRpcBoxStream<Result<ImRpcStreamResponse, ImRpcError>>, ImRpcError> {
    let watched_user_ids = request.user_ids;
    let state = state.clone();
    let auth = auth.clone();
    let stream = IntervalStream::new(tokio::time::interval(PRESENCE_WATCH_POLL_INTERVAL))
        .then(move |_| {
            let state = state.clone();
            let auth = auth.clone();
            let watched_user_ids = watched_user_ids.clone();
            async move {
                let sync_state = state
                    .rpc_client_route_state_snapshot(&auth, auth.device_id.as_deref())
                    .map_err(map_runtime_error)?;
                let snapshot = state
                    .rpc_presence_runtime()
                    .presence_snapshot(&auth, auth.device_id.clone(), sync_state.registered_route_keys)
                    .map_err(map_runtime_error)?;
                let presence = if watched_user_ids.is_empty()
                    || watched_user_ids.iter().any(|user_id| user_id == &auth.actor_id)
                {
                    presence_view_from_snapshot(&snapshot, snapshot.current_device_id.as_deref())
                } else {
                    PresenceView::default()
                };
                let response = WatchPresenceResponse {
                    presence: Some(presence),
                    metadata: None,
                };
                ImRpcStreamResponse::from_message(response)
            }
        });
    Ok(Box::pin(stream))
}

async fn dispatch_watch_realtime_events(
    state: &AppState,
    auth: &AppContext,
    request: WatchRealtimeEventsRequest,
) -> Result<ImRpcBoxStream<Result<ImRpcStreamResponse, ImRpcError>>, ImRpcError> {
    let after_seq = Arc::new(Mutex::new(parse_cursor_u64(request.cursor.as_str())?));
    let device_id = resolve_requested_device_id(auth, None).map_err(map_api_error)?;
    let state = state.clone();
    let auth = auth.clone();
    let stream = IntervalStream::new(tokio::time::interval(REALTIME_WATCH_POLL_INTERVAL)).then(
        move |_| {
            let state = state.clone();
            let auth = auth.clone();
            let device_id = device_id.clone();
            let after_seq = Arc::clone(&after_seq);
            async move {
                let current_after_seq = *after_seq
                    .lock()
                    .map_err(|_| ImRpcError::internal("realtime watch cursor lock poisoned"))?;
                state
                    .rpc_prepare_active_client_route(&auth, device_id.as_str(), "grpc_stream")
                    .map_err(map_runtime_error)?;
                let window = state
                    .rpc_realtime_runtime()
                    .list_events_for_principal_kind(
                        auth.tenant_id.as_str(),
                        auth.organization_id.as_str(),
                        auth.actor_id.as_str(),
                        auth.actor_kind.as_str(),
                        device_id.as_str(),
                        current_after_seq,
                        100,
                    )
                    .map_err(map_runtime_error)?;
                let event = window
                    .items
                    .last()
                    .map(realtime_event_to_proto)
                    .unwrap_or_default();
                if let Some(last) = window.items.last() {
                    if let Ok(mut cursor) = after_seq.lock() {
                        *cursor = last.realtime_seq;
                    }
                }
                let response = WatchRealtimeEventsResponse {
                    event: Some(event),
                    metadata: None,
                };
                ImRpcStreamResponse::from_message(response)
            }
        },
    );
    Ok(Box::pin(stream))
}

fn metadata_to_axum_headers(metadata: &RpcMetadata) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Some(value) = &metadata.authorization {
        if let Ok(parsed) = HeaderValue::from_str(value) {
            headers.insert(header::AUTHORIZATION, parsed);
        }
    }
    if let Some(value) = &metadata.access_token {
        if let Ok(parsed) = HeaderValue::from_str(value) {
            headers.insert("access-token", parsed);
        }
    }
    headers
}

fn optional_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn parse_cursor_u64(cursor: &str) -> Result<u64, ImRpcError> {
    let trimmed = cursor.trim();
    if trimmed.is_empty() {
        return Ok(0);
    }
    trimmed
        .parse::<u64>()
        .map_err(|error| ImRpcError::invalid_argument(format!("invalid realtime cursor `{cursor}`: {error}")))
}

fn proto_subscriptions_to_items(subscriptions: Vec<RealtimeSubscription>) -> Vec<RealtimeSubscriptionItemInput> {
    subscriptions
        .into_iter()
        .map(|subscription| {
            let (scope_type, scope_id) = if !subscription.stream_id.trim().is_empty() {
                ("stream", subscription.stream_id)
            } else if !subscription.conversation_id.trim().is_empty() {
                ("conversation", subscription.conversation_id)
            } else {
                ("conversation", subscription.subscription_id)
            };
            RealtimeSubscriptionItemInput {
                scope_type: scope_type.to_owned(),
                scope_id,
                event_types: Vec::new(),
            }
        })
        .collect()
}

fn snapshot_to_proto_subscriptions(snapshot: &RealtimeSubscriptionSnapshot) -> Vec<RealtimeSubscription> {
    snapshot
        .items
        .iter()
        .map(|item| RealtimeSubscription {
            subscription_id: format!("{}:{}", item.scope_type, item.scope_id),
            conversation_id: if item.scope_type == "conversation" {
                item.scope_id.clone()
            } else {
                String::new()
            },
            stream_id: if item.scope_type == "stream" {
                item.scope_id.clone()
            } else {
                String::new()
            },
            event_cursor: snapshot.synced_at.clone(),
        })
        .collect()
}

fn presence_view_from_snapshot(snapshot: &PresenceSnapshotView, device_id: Option<&str>) -> PresenceView {
    let selected_device = device_id
        .and_then(|device_id| {
            snapshot
                .devices
                .iter()
                .find(|device| device.device_id == device_id)
        })
        .or_else(|| snapshot.devices.first());
    match selected_device {
        Some(device) => PresenceView {
            user_id: snapshot.principal_id.clone(),
            device_id: device.device_id.clone(),
            status: device.status.as_str().to_owned(),
            last_seen_at: device.last_seen_at.clone().unwrap_or_default(),
        },
        None => PresenceView {
            user_id: snapshot.principal_id.clone(),
            device_id: device_id.unwrap_or_default().to_owned(),
            status: "offline".to_owned(),
            last_seen_at: String::new(),
        },
    }
}

fn realtime_event_to_proto(event: &RealtimeEvent) -> RealtimeEventView {
    RealtimeEventView {
        event_id: format!("{}:{}", event.realtime_seq, event.event_type),
        event_type: event.event_type.clone(),
        aggregate_type: event.scope_type.clone(),
        aggregate_id: event.scope_id.clone(),
        cursor: event.realtime_seq.to_string(),
        payload_json: event.payload.clone(),
    }
}

fn map_runtime_error<E: Into<ApiError>>(error: E) -> ImRpcError {
    map_api_error(error.into())
}

fn map_api_error(error: ApiError) -> ImRpcError {
    match error.status {
        axum::http::StatusCode::BAD_REQUEST | axum::http::StatusCode::PAYLOAD_TOO_LARGE => {
            ImRpcError::invalid_argument(error.message)
        }
        axum::http::StatusCode::UNAUTHORIZED => ImRpcError::unauthenticated(error.message),
        axum::http::StatusCode::FORBIDDEN => ImRpcError::permission_denied(error.message),
        axum::http::StatusCode::NOT_FOUND => ImRpcError::not_found(error.message),
        axum::http::StatusCode::CONFLICT => ImRpcError::already_exists(error.message),
        axum::http::StatusCode::SERVICE_UNAVAILABLE => ImRpcError::unavailable(error.message),
        _ => ImRpcError::internal(error.message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_gateway_rpc_service_keys_cover_realtime_surfaces() {
        assert_eq!(SESSION_GATEWAY_RPC_SERVICE_KEYS.len(), 2);
        assert!(SESSION_GATEWAY_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("PresenceService")));
        assert!(SESSION_GATEWAY_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("RealtimeService")));
    }

    #[test]
    fn parse_cursor_defaults_to_zero_for_empty_input() {
        assert_eq!(parse_cursor_u64("").expect("empty cursor"), 0);
        assert_eq!(parse_cursor_u64("42").expect("numeric cursor"), 42);
    }
}
