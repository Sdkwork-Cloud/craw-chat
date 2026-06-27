//! gRPC runtime dispatch for internal conversation RPC services (service-mtls).

use im_app_context::AppContext;
use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};
use prost::Message;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::MessageView;
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::internal::v1::{
    DispatchConversationMessageRequest, DispatchConversationMessageResponse,
    OrchestrateCreateRoomRequest, OrchestrateCreateRoomResponse, OrchestrateEnterRoomRequest,
    OrchestrateEnterRoomResponse, OrchestrateLeaveRoomRequest, OrchestrateLeaveRoomResponse,
    OrchestrateRetrieveRoomRequest, OrchestrateRetrieveRoomResponse, OrchestratedRoomView,
};
use sdkwork_im_rpc_service_rust::{
    ImRpcBoxFuture, ImRpcBoxStream, ImRpcError, ImRpcRuntimeDispatcher, ImRpcStreamRequest,
    ImRpcStreamResponse, ImRpcUnaryRequest, ImRpcUnaryResponse, RpcMetadata,
    admit_internal_unary_request,
};
use sdkwork_utils_rust::sha256_hash;

use crate::http::{self, AppState};
use crate::{
    CreateRoomCommand, EnterRoomCommand, PostMessageCommand, RuntimeError, RoomView,
};

pub const CONVERSATION_INTERNAL_RPC_SERVICE_KEYS: &[&str] = &[
    "sdkwork.communication.internal.v1.RoomOrchestrationService",
    "sdkwork.communication.internal.v1.MessageDispatchService",
];

#[derive(Clone)]
pub struct ConversationInternalRpcDispatcher {
    state: AppState,
}

impl ConversationInternalRpcDispatcher {
    pub async fn bootstrap_from_env() -> Result<Self, String> {
        let state = super::rpc_dispatch::bootstrap_conversation_app_state_from_env()?;
        Ok(Self { state })
    }

    pub fn from_app_state(state: AppState) -> Self {
        Self { state }
    }
}

impl ImRpcRuntimeDispatcher for ConversationInternalRpcDispatcher {
    fn dispatch_unary(
        &self,
        request: ImRpcUnaryRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>> {
        let state = self.state.clone();
        Box::pin(async move {
            admit_internal_unary_request(request.binding, &request.metadata)?;
            match request.binding.operation_id {
                "internal.rooms.create" => {
                    let payload =
                        OrchestrateCreateRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_internal_create_room(&state, &request.metadata, payload).await
                }
                "internal.rooms.retrieve" => {
                    let payload =
                        OrchestrateRetrieveRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_internal_retrieve_room(&state, payload).await
                }
                "internal.rooms.enter" => {
                    let payload =
                        OrchestrateEnterRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_internal_enter_room(&state, payload).await
                }
                "internal.rooms.leave" => {
                    let payload =
                        OrchestrateLeaveRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_internal_leave_room(&state, payload).await
                }
                "internal.messages.dispatch" => {
                    let payload = DispatchConversationMessageRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_internal_conversation_message(&state, &request.metadata, payload)
                        .await
                }
                other => Err(ImRpcError::unimplemented(format!(
                    "conversation internal rpc host does not implement unary operation `{other}`"
                ))),
            }
        })
    }

    fn dispatch_server_stream(
        &self,
        request: ImRpcStreamRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcBoxStream<Result<ImRpcStreamResponse, ImRpcError>>, ImRpcError>>
    {
        let operation_id = request.binding.operation_id;
        let method_key = request.binding.method_key;
        Box::pin(async move {
            admit_internal_unary_request(request.binding, &request.metadata)?;
            Err(ImRpcError::unimplemented(format!(
                "conversation internal rpc host does not implement stream `{operation_id}` ({method_key})"
            )))
        })
    }
}

async fn dispatch_internal_create_room(
    state: &AppState,
    metadata: &RpcMetadata,
    request: OrchestrateCreateRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let tenant_id = required_field(request.tenant_id, "tenant_id")?;
    let organization_id = optional_organization_id(request.organization_id);
    let actor_id = required_field(request.actor_id, "actor_id")?;
    let actor_kind = required_field(request.actor_kind, "actor_kind")?;
    let conversation_id = if request.conversation_id.trim().is_empty() {
        derive_idempotent_resource_id(metadata, "internal-room-conversation")?
    } else {
        request.conversation_id
    };
    let room_id = if request.room_id.trim().is_empty() {
        derive_idempotent_resource_id(metadata, "internal-room")?
    } else {
        request.room_id
    };
    let room_kind = required_field(request.room_kind, "room_kind")?;
    let result = state
        .rpc_runtime()
        .create_room_with_creator_kind(
            CreateRoomCommand {
                tenant_id: tenant_id.clone(),
                organization_id: organization_id.clone(),
                conversation_id,
                room_id: room_id.clone(),
                room_kind,
                creator_id: actor_id,
            },
            actor_kind.as_str(),
        )
        .map_err(map_runtime_error)?;
    let room = state
        .rpc_runtime()
        .room_view(tenant_id.as_str(), organization_id.as_str(), room_id.as_str())
        .map_err(map_runtime_error)?;
    let response = OrchestrateCreateRoomResponse {
        conversation_id: result.conversation_id,
        event_id: result.event_id,
        room: Some(orchestrated_room_view_from_domain(&room)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_internal_retrieve_room(
    state: &AppState,
    request: OrchestrateRetrieveRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let tenant_id = required_field(request.tenant_id, "tenant_id")?;
    let organization_id = optional_organization_id(request.organization_id);
    let room_id = required_field(request.room_id, "room_id")?;
    let room = state
        .rpc_runtime()
        .room_view(tenant_id.as_str(), organization_id.as_str(), room_id.as_str())
        .map_err(map_runtime_error)?;
    let response = OrchestrateRetrieveRoomResponse {
        room: Some(orchestrated_room_view_from_domain(&room)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_internal_enter_room(
    state: &AppState,
    request: OrchestrateEnterRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let tenant_id = required_field(request.tenant_id, "tenant_id")?;
    let organization_id = optional_organization_id(request.organization_id);
    let room_id = required_field(request.room_id, "room_id")?;
    let principal_id = required_field(request.principal_id, "principal_id")?;
    let principal_kind = required_field(request.principal_kind, "principal_kind")?;
    let member = state
        .rpc_runtime()
        .enter_room_with_principal_kind(
            EnterRoomCommand {
                tenant_id: tenant_id.clone(),
                organization_id: organization_id.clone(),
                room_id: room_id.clone(),
                principal_id: principal_id.clone(),
                principal_kind: principal_kind.clone(),
            },
            principal_kind.as_str(),
        )
        .map_err(map_runtime_error)?;
    let conversation_id = state
        .rpc_runtime()
        .room_view(tenant_id.as_str(), organization_id.as_str(), room_id.as_str())
        .map_err(map_runtime_error)?
        .conversation_id;
    let response = OrchestrateEnterRoomResponse {
        member_id: member.member_id,
        conversation_id,
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_internal_leave_room(
    state: &AppState,
    request: OrchestrateLeaveRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let tenant_id = required_field(request.tenant_id, "tenant_id")?;
    let organization_id = optional_organization_id(request.organization_id);
    let room_id = required_field(request.room_id, "room_id")?;
    let principal_id = required_field(request.principal_id, "principal_id")?;
    let principal_kind = required_field(request.principal_kind, "principal_kind")?;
    let auth = internal_actor_context(
        tenant_id.as_str(),
        organization_id.as_str(),
        principal_id.as_str(),
        principal_kind.as_str(),
    );
    let member = state
        .rpc_runtime()
        .leave_room_from_auth_context(&auth, room_id.clone())
        .map_err(map_runtime_error)?;
    let conversation_id = state
        .rpc_runtime()
        .room_view(tenant_id.as_str(), organization_id.as_str(), room_id.as_str())
        .map_err(map_runtime_error)?
        .conversation_id;
    let response = OrchestrateLeaveRoomResponse {
        member_id: member.member_id,
        conversation_id,
        status: membership_state_label(&member.state).into(),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_internal_conversation_message(
    state: &AppState,
    metadata: &RpcMetadata,
    request: DispatchConversationMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let tenant_id = required_field(request.tenant_id, "tenant_id")?;
    let organization_id = optional_organization_id(request.organization_id);
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let sender_id = required_field(request.sender_id, "sender_id")?;
    let sender_kind = required_field(request.sender_kind, "sender_kind")?;
    let schema_ref = required_field(request.schema_ref, "schema_ref")?;
    let payload_json = required_field(request.payload_json, "payload_json")?;
    let encoding = optional_string(request.encoding).unwrap_or_else(|| "application/json".into());
    let client_msg_id = optional_string(request.client_msg_id).or_else(|| {
        metadata
            .idempotency_key
            .clone()
            .filter(|value| !value.trim().is_empty())
    });
    let body = MessageBody {
        summary: None,
        parts: vec![ContentPart::Data(im_domain_core::message::DataPart {
            schema_ref,
            encoding,
            payload: payload_json,
        })],
        render_hints: Default::default(),
        reply_to: None,
    };
    let result = state
        .rpc_runtime()
        .post_message(PostMessageCommand {
            tenant_id: tenant_id.clone(),
            organization_id: organization_id.clone(),
            conversation_id,
            sender: Sender {
                id: sender_id.clone(),
                kind: sender_kind.clone(),
                member_id: None,
                device_id: None,
                session_id: metadata.request_id.clone(),
                metadata: Default::default(),
            },
            client_msg_id,
            message_type: MessageType::Standard,
            body,
        })
        .map_err(map_runtime_error)?;
    let auth = internal_actor_context(
        tenant_id.as_str(),
        organization_id.as_str(),
        sender_id.as_str(),
        sender_kind.as_str(),
    );
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(&auth, result.message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = DispatchConversationMessageResponse {
        message: Some(message_view_from_stored(&stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

fn internal_actor_context(
    tenant_id: &str,
    organization_id: &str,
    actor_id: &str,
    actor_kind: &str,
) -> AppContext {
    AppContext {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        user_id: actor_id.to_owned(),
        session_id: None,
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        actor_id: actor_id.to_owned(),
        actor_kind: actor_kind.to_owned(),
        device_id: None,
    }
}

fn orchestrated_room_view_from_domain(view: &RoomView) -> OrchestratedRoomView {
    OrchestratedRoomView {
        room_id: view.room_id.clone(),
        room_kind: view.room_kind.clone(),
        conversation_id: view.conversation_id.clone(),
        active_member_count: view.active_member_count as i32,
        max_members: view.max_members as i32,
    }
}

fn message_view_from_stored(stored: &im_domain_core::message::StoredMessage) -> MessageView {
    use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::MessageBodyPart;

    MessageView {
        message_id: stored.message.message_id.clone(),
        conversation_id: stored.message.conversation_id.clone(),
        sender_user_id: stored.message.sender.id.clone(),
        body_parts: stored
            .message
            .body
            .parts
            .iter()
            .map(|part| match part {
                ContentPart::Text(text_part) => MessageBodyPart {
                    kind: "text".into(),
                    text: text_part.text.clone(),
                    payload_json: String::new(),
                    media: None,
                },
                ContentPart::Data(data) => MessageBodyPart {
                    kind: "data".into(),
                    text: String::new(),
                    payload_json: data.payload.clone(),
                    media: None,
                },
                ContentPart::Media(_) | ContentPart::Signal(_) | ContentPart::StreamRef(_) => MessageBodyPart {
                    kind: "structured".into(),
                    text: String::new(),
                    payload_json: String::new(),
                    media: None,
                },
            })
            .collect(),
        state: if stored.recalled {
            "recalled".into()
        } else {
            "active".into()
        },
        created_at: stored.message.occurred_at.clone(),
    }
}

fn membership_state_label(state: &im_domain_core::conversation::MembershipState) -> &'static str {
    match state {
        im_domain_core::conversation::MembershipState::Joined => "joined",
        im_domain_core::conversation::MembershipState::Invited => "invited",
        im_domain_core::conversation::MembershipState::Linked => "linked",
        im_domain_core::conversation::MembershipState::Left => "left",
        im_domain_core::conversation::MembershipState::Removed => "removed",
    }
}

fn derive_idempotent_resource_id(
    metadata: &RpcMetadata,
    namespace: &str,
) -> Result<String, ImRpcError> {
    let key = metadata
        .idempotency_key
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            ImRpcError::invalid_argument("idempotency-key metadata is required for this RPC method")
        })?;
    Ok(format!("rpc-{namespace}-{}", sha256_hash(key.as_bytes())))
}

fn required_field(value: String, field: &str) -> Result<String, ImRpcError> {
    if value.trim().is_empty() {
        return Err(ImRpcError::invalid_argument(format!("{field} is required")));
    }
    Ok(value)
}

fn optional_string(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

fn optional_organization_id(value: String) -> String {
    if value.trim().is_empty() {
        "0".to_owned()
    } else {
        value
    }
}

fn map_runtime_error(error: RuntimeError) -> ImRpcError {
    http::map_api_error_to_im_rpc(error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_rpc_service_keys_cover_orchestration_and_dispatch() {
        assert_eq!(CONVERSATION_INTERNAL_RPC_SERVICE_KEYS.len(), 2);
        assert!(CONVERSATION_INTERNAL_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("RoomOrchestrationService")));
        assert!(CONVERSATION_INTERNAL_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("MessageDispatchService")));
    }
}
