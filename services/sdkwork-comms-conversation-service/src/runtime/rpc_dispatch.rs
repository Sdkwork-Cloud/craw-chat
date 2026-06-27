//! gRPC runtime dispatch for conversation and message RPC services.

use std::collections::BTreeMap;
use std::sync::Arc;

use axum::http::{HeaderMap, HeaderValue, header};
use im_app_context::AppContext;
use im_domain_core::conversation::{member_id, ConversationMember, MembershipRole};
use im_domain_core::message::{ContentPart, MessageReplyReference, MessageType};
use prost::Message;
use sdkwork_im_rpc_sdk_rust::sdkwork::common::v1::{PageRequest, PageResponse};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    AddConversationMemberRequest, AddConversationMemberResponse, BindDirectChatRequest,
    BindDirectChatResponse, ChangeConversationMemberRoleRequest,
    ChangeConversationMemberRoleResponse, ConversationMemberView, ConversationView,
    CreateAgentDialogRequest, CreateAgentDialogResponse, CreateAgentHandoffRequest,
    CreateAgentHandoffResponse, CreateConversationMessageRequest,
    CreateConversationMessageResponse, CreateConversationRequest, CreateConversationResponse,
    CreateMessageReactionRequest, CreateMessageReactionResponse, CreateSystemChannelRequest,
    CreateSystemChannelResponse, CreateRoomRequest, CreateRoomResponse, CreateThreadRequest,
    CreateThreadResponse,
    DeleteMessageFavoriteRequest, DeleteMessageReactionRequest,
    EnterRoomRequest, EnterRoomResponse, LeaveRoomRequest, LeaveRoomResponse,
    DeleteMessageReactionResponse, DeleteMessageVisibilityRequest,
    EditMessageRequest, EditMessageResponse, LeaveConversationRequest, LeaveConversationResponse,
    ListConversationMemberDirectoryRequest, ListConversationMemberDirectoryResponse,
    ListConversationMembersRequest, ListConversationMembersResponse,
    ListConversationMessagesRequest, ListConversationMessagesResponse, ListFavoriteMessagesRequest,
    ListPinnedMessagesRequest, ListPinnedMessagesResponse,
    MessageBodyPart, MessageInteractionSummaryView, MessageMutationResponse,
    MessageView, PinMessageRequest, PinMessageResponse, PublishSystemChannelMessageRequest,
    PublishSystemChannelMessageResponse, RecallMessageRequest, RecallMessageResponse,
    ReadCursorView, RemoveConversationMemberRequest, RemoveConversationMemberResponse,
    RetrieveConversationPreferencesRequest, RetrieveConversationProfileRequest,
    RetrieveConversationRequest, RetrieveConversationResponse, RetrieveInboxRequest,
    RetrieveRoomRequest, RetrieveRoomResponse, RoomView,
    RetrieveInboxResponse, RetrieveMessageInteractionSummaryRequest,
    RetrieveMessageInteractionSummaryResponse, RetrieveReadCursorRequest,
    RetrieveReadCursorResponse, TransferConversationOwnerRequest,
    TransferConversationOwnerResponse, UnpinMessageRequest, UnpinMessageResponse,
    UpdateConversationPreferencesRequest, UpdateConversationProfileRequest, UpdateReadCursorRequest,
    UpdateReadCursorResponse,
};
use sdkwork_im_rpc_service_rust::{
    ImRpcBoxFuture, ImRpcBoxStream, ImRpcError, ImRpcRuntimeDispatcher, ImRpcStreamRequest,
    ImRpcStreamResponse, ImRpcUnaryRequest, ImRpcUnaryResponse, RpcMetadata,
    admit_app_unary_request, require_app_session_auth,
};
use sdkwork_utils_rust::sha256_hash;

use crate::http::{self, AppState};
use crate::{
    AddMessageReactionCommand, EditMessageCommand, PinMessageCommand, PostMessageCommand,
    PublishSystemChannelMessageCommand, RecallMessageCommand, RemoveMessageReactionCommand,
    RuntimeError, UnpinMessageCommand,
};

pub const CONVERSATION_RPC_SERVICE_KEYS: &[&str] = &[
    "sdkwork.communication.app.v3.ConversationService",
    "sdkwork.communication.app.v3.MessageService",
    "sdkwork.communication.app.v3.RoomService",
];

const PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV: &str = "SDKWORK_IM_PRINCIPAL_DIRECTORY_CATALOG_PATH";
const ALLOW_ALL_PRINCIPALS_ENV: &str = "SDKWORK_IM_ALLOW_ALL_PRINCIPALS";
const DEFAULT_PAGE_SIZE: usize = 50;
const MAX_PAGE_SIZE: usize = 200;

#[derive(Clone)]
pub struct ConversationRpcDispatcher {
    state: AppState,
}

impl ConversationRpcDispatcher {
    pub async fn bootstrap_from_env() -> Result<Self, String> {
        let state = bootstrap_conversation_app_state_from_env()?;
        Ok(Self { state })
    }

    pub fn from_app_state(state: AppState) -> Self {
        Self { state }
    }
}

impl ImRpcRuntimeDispatcher for ConversationRpcDispatcher {
    fn dispatch_unary(
        &self,
        request: ImRpcUnaryRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>> {
        let state = self.state.clone();
        Box::pin(async move {
            admit_app_unary_request(request.binding, &request.metadata)?;
            let auth = resolve_auth(&state, &request.metadata)?;
            match request.binding.operation_id {
                "conversations.create" => {
                    let payload =
                        CreateConversationRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_conversation(&state, &auth, &request.metadata, payload).await
                }
                "conversations.agentDialogs.create" => {
                    let payload =
                        CreateAgentDialogRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_agent_dialog(&state, &auth, &request.metadata, payload).await
                }
                "conversations.agentHandoffs.create" => {
                    let payload =
                        CreateAgentHandoffRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_agent_handoff(&state, &auth, &request.metadata, payload).await
                }
                "conversations.systemChannels.create" => {
                    let payload =
                        CreateSystemChannelRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_system_channel(&state, &auth, &request.metadata, payload)
                        .await
                }
                "conversations.threads.create" => {
                    let payload = CreateThreadRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_thread(&state, &auth, &request.metadata, payload).await
                }
                "conversations.directChats.bind" => {
                    let payload = BindDirectChatRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_bind_direct_chat(&state, &auth, &request.metadata, payload).await
                }
                "conversations.retrieve" => {
                    let payload =
                        RetrieveConversationRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_retrieve_conversation(&state, &auth, payload).await
                }
                "inbox.retrieve" => {
                    let payload = RetrieveInboxRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_retrieve_inbox(&state, &auth, payload).await
                }
                "conversations.members.list" => {
                    let payload =
                        ListConversationMembersRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_list_members(&state, &auth, payload).await
                }
                "conversations.members.add" => {
                    let payload =
                        AddConversationMemberRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_add_member(&state, &auth, payload).await
                }
                "conversations.members.remove" => {
                    let payload =
                        RemoveConversationMemberRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_remove_member(&state, &auth, payload).await
                }
                "conversations.members.transferOwner" => {
                    let payload = TransferConversationOwnerRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_transfer_owner(&state, &auth, payload).await
                }
                "conversations.members.changeRole" => {
                    let payload = ChangeConversationMemberRoleRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_change_member_role(&state, &auth, payload).await
                }
                "conversations.members.leave" => {
                    let payload =
                        LeaveConversationRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_leave_conversation(&state, &auth, payload).await
                }
                "conversations.preferences.retrieve" => {
                    let payload = RetrieveConversationPreferencesRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_projection_boundary("conversations.preferences.retrieve", payload).await
                }
                "conversations.preferences.update" => {
                    let payload = UpdateConversationPreferencesRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_projection_boundary("conversations.preferences.update", payload).await
                }
                "conversations.profile.retrieve" => {
                    let payload = RetrieveConversationProfileRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_projection_boundary("conversations.profile.retrieve", payload).await
                }
                "conversations.profile.update" => {
                    let payload =
                        UpdateConversationProfileRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_projection_boundary("conversations.profile.update", payload).await
                }
                "conversations.readCursor.retrieve" => {
                    let payload =
                        RetrieveReadCursorRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_retrieve_read_cursor(&state, &auth, payload).await
                }
                "conversations.readCursor.update" => {
                    let payload =
                        UpdateReadCursorRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_update_read_cursor(&state, &auth, payload).await
                }
                "conversations.memberDirectory.list" => {
                    let payload = ListConversationMemberDirectoryRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_list_member_directory(&state, &auth, payload).await
                }
                "conversations.pins.list" => {
                    let payload =
                        ListPinnedMessagesRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_list_pinned_messages(&state, &auth, payload).await
                }
                "conversations.messages.list" => {
                    let payload =
                        ListConversationMessagesRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_list_messages(&state, &auth, payload).await
                }
                "conversations.messages.create" => {
                    let payload = CreateConversationMessageRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_create_message(&state, &auth, &request.metadata, payload).await
                }
                "conversations.systemChannel.publish" => {
                    let payload = PublishSystemChannelMessageRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_publish_system_channel_message(
                        &state,
                        &auth,
                        &request.metadata,
                        payload,
                    )
                    .await
                }
                "conversations.messages.interactionSummary.retrieve" => {
                    let payload = RetrieveMessageInteractionSummaryRequest::decode(
                        request.request_bytes.as_slice(),
                    )?;
                    dispatch_retrieve_message_interaction_summary(&state, &auth, payload).await
                }
                "messages.edit" => {
                    let payload = EditMessageRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_edit_message(&state, &auth, payload).await
                }
                "messages.recall" => {
                    let payload = RecallMessageRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_recall_message(&state, &auth, payload).await
                }
                "messages.favorites.list" => {
                    let payload =
                        ListFavoriteMessagesRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_projection_boundary("messages.favorites.list", payload).await
                }
                "messages.favorites.create" => {
                    let _payload = request.request_bytes;
                    dispatch_projection_boundary_unit("messages.favorites.create").await
                }
                "messages.favorites.delete" => {
                    let payload =
                        DeleteMessageFavoriteRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_projection_boundary("messages.favorites.delete", payload).await
                }
                "messages.visibility.delete" => {
                    let payload =
                        DeleteMessageVisibilityRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_projection_boundary("messages.visibility.delete", payload).await
                }
                "messages.reactions.create" => {
                    let payload =
                        CreateMessageReactionRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_message_reaction(&state, &auth, payload).await
                }
                "messages.reactions.delete" => {
                    let payload =
                        DeleteMessageReactionRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_delete_message_reaction(&state, &auth, payload).await
                }
                "messages.pin.create" => {
                    let payload = PinMessageRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_pin_message(&state, &auth, payload).await
                }
                "messages.pin.delete" => {
                    let payload = UnpinMessageRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_unpin_message(&state, &auth, payload).await
                }
                "rooms.create" => {
                    let payload = CreateRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_create_room(&state, &auth, &request.metadata, payload).await
                }
                "rooms.get" => {
                    let payload = RetrieveRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_retrieve_room(&state, &auth, payload).await
                }
                "rooms.enter" => {
                    let payload = EnterRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_enter_room(&state, &auth, payload).await
                }
                "rooms.leave" => {
                    let payload = LeaveRoomRequest::decode(request.request_bytes.as_slice())?;
                    dispatch_leave_room(&state, &auth, payload).await
                }
                other => Err(ImRpcError::unimplemented(format!(
                    "conversation rpc host does not implement unary operation `{other}`"
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
            require_app_session_auth(request.binding, &request.metadata)?;
            let _auth = resolve_auth_placeholder(&request.metadata)?;
            Err(ImRpcError::unimplemented(format!(
                "conversation rpc host does not implement stream `{operation_id}` ({method_key})"
            )))
        })
    }
}

fn resolve_auth(state: &AppState, metadata: &RpcMetadata) -> Result<AppContext, ImRpcError> {
    let headers = metadata_to_axum_headers(metadata);
    http::resolve_active_rpc_auth_context(state, &headers).map_err(map_api_error)
}

fn resolve_auth_placeholder(metadata: &RpcMetadata) -> Result<(), ImRpcError> {
    if metadata.authorization.as_deref().is_none() && metadata.access_token.as_deref().is_none() {
        return Err(ImRpcError::unauthenticated(
            "app-session RPC requires authorization or access-token metadata",
        ));
    }
    Ok(())
}

async fn dispatch_projection_boundary<T: prost::Message>(
    operation_id: &str,
    _payload: T,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    dispatch_projection_boundary_unit(operation_id).await
}

async fn dispatch_projection_boundary_unit(
    operation_id: &str,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    Err(ImRpcError::unimplemented(format!(
        "operation `{operation_id}` is owned by the inbox/projection plane and is not served by \
         conversation-runtime RPC host; use the HTTP OpenAPI projection consumer"
    )))
}

async fn dispatch_create_conversation(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateConversationRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = derive_idempotent_resource_id(metadata, "conversation")?;
    let conversation_type = required_field(request.conversation_type, "conversation_type")?;
    let result = state
        .rpc_runtime()
        .create_conversation_from_auth_context(auth, conversation_id.clone(), conversation_type)
        .map_err(map_runtime_error)?;
    for user_id in request.member_user_ids {
        if user_id == auth.actor_id {
            continue;
        }
        http::ensure_active_rpc_principal(
            state,
            auth.tenant_id.as_str(),
            user_id.as_str(),
            "user",
        )
        .map_err(map_api_error)?;
        let _ = state.rpc_runtime().add_member_from_auth_context(
            auth,
            result.conversation_id.clone(),
            user_id,
            "user".into(),
            MembershipRole::Member,
            BTreeMap::new(),
        );
    }
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateConversationResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            request.title,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_agent_dialog(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateAgentDialogRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = derive_idempotent_resource_id(metadata, "agent-dialog")?;
    let agent_id = required_field(request.agent_id, "agent_id")?;
    let result = state
        .rpc_runtime()
        .create_agent_dialog_from_auth_context(auth, conversation_id, agent_id)
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateAgentDialogResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            request.title,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_agent_handoff(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateAgentHandoffRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let source_conversation_id =
        required_field(request.source_conversation_id, "source_conversation_id")?;
    let (target_id, target_kind) =
        resolve_handoff_target_from_source(state, auth, source_conversation_id.as_str())?;
    let conversation_id = derive_idempotent_resource_id(metadata, "agent-handoff")?;
    let handoff_reason = optional_string(request.reason);
    let result = state
        .rpc_runtime()
        .create_agent_handoff_from_auth_context(
            auth,
            conversation_id,
            target_id,
            target_kind,
            source_conversation_id,
            handoff_reason,
        )
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateAgentHandoffResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_system_channel(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateSystemChannelRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = if let Some(channel_key) = optional_string(request.channel_key) {
        channel_key
    } else {
        derive_idempotent_resource_id(metadata, "system-channel")?
    };
    let result = state
        .rpc_runtime()
        .create_system_channel_from_auth_context(
            auth,
            conversation_id,
            auth.actor_id.clone(),
        )
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateSystemChannelResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            request.title,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_thread(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateThreadRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = derive_idempotent_resource_id(metadata, "thread")?;
    let parent_conversation_id =
        required_field(request.parent_conversation_id, "parent_conversation_id")?;
    let root_message_id = required_field(request.root_message_id, "root_message_id")?;
    let result = state
        .rpc_runtime()
        .create_thread_conversation_from_auth_context(
            auth,
            conversation_id,
            parent_conversation_id,
            root_message_id,
        )
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateThreadResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_bind_direct_chat(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: BindDirectChatRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let peer_user_id = required_field(request.peer_user_id, "peer_user_id")?;
    http::ensure_active_rpc_principal(
        state,
        auth.tenant_id.as_str(),
        peer_user_id.as_str(),
        "user",
    )
    .map_err(map_api_error)?;
    let conversation_id = derive_idempotent_resource_id(metadata, "direct-chat")?;
    let direct_chat_id = canonical_direct_chat_id(auth.actor_id.as_str(), peer_user_id.as_str());
    let result = state
        .rpc_runtime()
        .bind_direct_chat_conversation_from_auth_context(
            auth,
            conversation_id,
            direct_chat_id,
            auth.actor_id.clone(),
            auth.actor_kind.clone(),
            peer_user_id,
            "user".into(),
        )
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = BindDirectChatResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_retrieve_conversation(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveConversationRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = RetrieveConversationResponse {
        conversation: Some(conversation_view_from_binding(
            conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_retrieve_inbox(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveInboxRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let (limit, cursor) = page_request(request.page);
    let inbox = state
        .rpc_runtime()
        .retrieve_inbox_from_auth_context(auth, limit, cursor.as_deref())
        .map_err(map_runtime_error)?;
    let mut conversations = Vec::with_capacity(inbox.conversation_ids.len());
    for conversation_id in &inbox.conversation_ids {
        if let Ok(binding) = state
            .rpc_runtime()
            .conversation_business_binding_from_auth_context(auth, conversation_id.as_str())
        {
            conversations.push(conversation_view_from_binding(
                conversation_id.as_str(),
                &binding,
                String::new(),
            ));
        }
    }
    let conversation_count = conversations.len();
    let response = RetrieveInboxResponse {
        conversations,
        page: Some(page_response(
            inbox.next_cursor,
            inbox.has_more,
            conversation_count,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_list_members(
    state: &AppState,
    auth: &AppContext,
    request: ListConversationMembersRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let (limit, cursor) = page_request(request.page);
    let members = state
        .rpc_runtime()
        .list_members_window_from_auth_context(
            auth,
            conversation_id.as_str(),
            Some(limit),
            cursor.as_deref(),
        )
        .map_err(map_runtime_error)?;
    let response = ListConversationMembersResponse {
        members: members
            .items
            .iter()
            .map(member_view_from_domain)
            .collect(),
        page: Some(page_response(
            members.next_cursor,
            members.has_more,
            members.items.len(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_add_member(
    state: &AppState,
    auth: &AppContext,
    request: AddConversationMemberRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let user_id = required_field(request.user_id, "user_id")?;
    http::ensure_active_rpc_principal(state, auth.tenant_id.as_str(), user_id.as_str(), "user")
        .map_err(map_api_error)?;
    let member = state
        .rpc_runtime()
        .add_member_from_auth_context(
            auth,
            conversation_id,
            user_id,
            "user".into(),
            parse_membership_role(request.role.as_str()),
            BTreeMap::new(),
        )
        .map_err(map_runtime_error)?;
    let response = AddConversationMemberResponse {
        member: Some(member_view_from_domain(&member)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_remove_member(
    state: &AppState,
    auth: &AppContext,
    request: RemoveConversationMemberRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let user_id = required_field(request.user_id, "user_id")?;
    let member_id = member_id(conversation_id.as_str(), "user", user_id.as_str());
    let _member = state
        .rpc_runtime()
        .remove_member_from_auth_context(auth, conversation_id.clone(), member_id)
        .map_err(map_runtime_error)?;
    let response = RemoveConversationMemberResponse {
        conversation_id,
        user_id,
        status: "removed".into(),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_transfer_owner(
    state: &AppState,
    auth: &AppContext,
    request: TransferConversationOwnerRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let new_owner_user_id = required_field(request.new_owner_user_id, "new_owner_user_id")?;
    let target_member_id = member_id(
        conversation_id.as_str(),
        "user",
        new_owner_user_id.as_str(),
    );
    let result = state
        .rpc_runtime()
        .transfer_conversation_owner_from_auth_context(
            auth,
            conversation_id.clone(),
            target_member_id,
        )
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = TransferConversationOwnerResponse {
        conversation: Some(conversation_view_from_binding(
            conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        metadata: None,
    };
    let _ = result;
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_change_member_role(
    state: &AppState,
    auth: &AppContext,
    request: ChangeConversationMemberRoleRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let user_id = required_field(request.user_id, "user_id")?;
    let target_member_id = member_id(conversation_id.as_str(), "user", user_id.as_str());
    let result = state
        .rpc_runtime()
        .change_conversation_member_role_from_auth_context(
            auth,
            conversation_id,
            target_member_id,
            parse_membership_role(request.role.as_str()),
        )
        .map_err(map_runtime_error)?;
    let response = ChangeConversationMemberRoleResponse {
        member: Some(member_view_from_domain(&result.updated_member)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_leave_conversation(
    state: &AppState,
    auth: &AppContext,
    request: LeaveConversationRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let _member = state
        .rpc_runtime()
        .leave_conversation_from_auth_context(auth, conversation_id.clone())
        .map_err(map_runtime_error)?;
    let response = LeaveConversationResponse {
        conversation_id,
        status: "left".into(),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_retrieve_read_cursor(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveReadCursorRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let cursor = state
        .rpc_runtime()
        .read_cursor_view_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = RetrieveReadCursorResponse {
        cursor: Some(read_cursor_view_from_domain(&cursor)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_update_read_cursor(
    state: &AppState,
    auth: &AppContext,
    request: UpdateReadCursorRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let read_seq = parse_cursor_u64(request.event_cursor.as_str())?;
    let last_read_message_id = optional_string(request.message_id);
    state
        .rpc_runtime()
        .update_read_cursor_from_auth_context(
            auth,
            conversation_id.clone(),
            read_seq,
            last_read_message_id,
        )
        .map_err(map_runtime_error)?;
    let cursor = state
        .rpc_runtime()
        .read_cursor_view_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let response = UpdateReadCursorResponse {
        cursor: Some(read_cursor_view_from_domain(&cursor)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_list_member_directory(
    state: &AppState,
    auth: &AppContext,
    request: ListConversationMemberDirectoryRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let query = request.query.to_ascii_lowercase();
    let (limit, cursor) = page_request(request.page);
    let members = state
        .rpc_runtime()
        .list_members_window_from_auth_context(
            auth,
            conversation_id.as_str(),
            Some(limit.saturating_mul(4).min(MAX_PAGE_SIZE)),
            cursor.as_deref(),
        )
        .map_err(map_runtime_error)?;
    let filtered = members
        .items
        .into_iter()
        .filter(|member| {
            query.is_empty()
                || member.principal_id.to_ascii_lowercase().contains(query.as_str())
        })
        .take(limit)
        .collect::<Vec<_>>();
    let response = ListConversationMemberDirectoryResponse {
        members: filtered.iter().map(member_view_from_domain).collect(),
        page: Some(page_response(
            members.next_cursor,
            members.has_more,
            filtered.len(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_list_pinned_messages(
    state: &AppState,
    auth: &AppContext,
    request: ListPinnedMessagesRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let (limit, cursor) = page_request(request.page);
    let pinned = state
        .rpc_runtime()
        .list_pinned_message_ids_from_auth_context(
            auth,
            conversation_id.as_str(),
            limit,
            cursor.as_deref(),
        )
        .map_err(map_runtime_error)?;
    let message_count = pinned.message_ids.len();
    let response = ListPinnedMessagesResponse {
        message_ids: pinned.message_ids,
        page: Some(page_response(
            pinned.next_cursor,
            pinned.has_more,
            message_count,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_list_messages(
    state: &AppState,
    auth: &AppContext,
    request: ListConversationMessagesRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let page = request.page;
    let (limit, cursor) = page_request(page.clone());
    let after_seq = cursor
        .as_deref()
        .map(parse_cursor_u64)
        .transpose()?;
    let history = state
        .rpc_runtime()
        .list_messages_window_from_auth_context(
            auth,
            conversation_id.as_str(),
            after_seq,
            limit,
        )
        .map_err(map_runtime_error)?;
    let response = ListConversationMessagesResponse {
        messages: history
            .items
            .iter()
            .map(message_view_from_stored)
            .collect(),
        page: Some(page_response(
            history
                .next_after_seq
                .map(|seq: u64| seq.to_string()),
            history.has_more,
            history.items.len(),
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_message(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateConversationMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let reply_to = optional_string(request.reply_to_message_id).map(|message_id| {
        MessageReplyReference {
            message_id,
            sender_display_name: String::new(),
            content_preview: String::new(),
        }
    });
    let body = proto_parts_to_message_body(request.body_parts, reply_to).map_err(map_api_error)?;
    let client_msg_id = metadata
        .idempotency_key
        .clone()
        .filter(|value| !value.trim().is_empty());
    let result = state
        .rpc_runtime()
        .post_message(PostMessageCommand::from_auth_context(
            auth,
            conversation_id.clone(),
            client_msg_id,
            MessageType::Standard,
            body,
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, result.message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateConversationMessageResponse {
        message: Some(message_view_from_stored(&stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_publish_system_channel_message(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: PublishSystemChannelMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let body = proto_parts_to_message_body(request.body_parts, None).map_err(map_api_error)?;
    let client_msg_id = metadata
        .idempotency_key
        .clone()
        .filter(|value| !value.trim().is_empty());
    let result = state
        .rpc_runtime()
        .publish_system_channel_message(PublishSystemChannelMessageCommand::from_auth_context(
            auth,
            conversation_id,
            client_msg_id,
            body,
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, result.message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = PublishSystemChannelMessageResponse {
        message: Some(message_view_from_stored(&stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_retrieve_message_interaction_summary(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveMessageInteractionSummaryRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = RetrieveMessageInteractionSummaryResponse {
        summary: Some(interaction_summary_from_stored(message_id.as_str(), &stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_edit_message(
    state: &AppState,
    auth: &AppContext,
    request: EditMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let body = proto_parts_to_message_body(request.body_parts, None).map_err(map_api_error)?;
    let result = state
        .rpc_runtime()
        .edit_message(EditMessageCommand::from_auth_context(
            auth,
            message_id.clone(),
            body,
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = EditMessageResponse {
        result: Some(message_mutation_response_from_stored(
            &stored,
            result.event_id,
        )),
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_recall_message(
    state: &AppState,
    auth: &AppContext,
    request: RecallMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let result = state
        .rpc_runtime()
        .recall_message(RecallMessageCommand::from_auth_context(
            auth,
            message_id.clone(),
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = RecallMessageResponse {
        result: Some(message_mutation_response_from_stored(
            &stored,
            result.event_id,
        )),
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_message_reaction(
    state: &AppState,
    auth: &AppContext,
    request: CreateMessageReactionRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let reaction_key = required_field(request.reaction, "reaction")?;
    let _mutation = state
        .rpc_runtime()
        .add_message_reaction(AddMessageReactionCommand::from_auth_context(
            auth,
            message_id.clone(),
            reaction_key,
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = CreateMessageReactionResponse {
        summary: Some(interaction_summary_from_stored(message_id.as_str(), &stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_delete_message_reaction(
    state: &AppState,
    auth: &AppContext,
    request: DeleteMessageReactionRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let reaction_key = required_field(request.reaction, "reaction")?;
    let _mutation = state
        .rpc_runtime()
        .remove_message_reaction(RemoveMessageReactionCommand::from_auth_context(
            auth,
            message_id.clone(),
            reaction_key,
        ))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = DeleteMessageReactionResponse {
        summary: Some(interaction_summary_from_stored(message_id.as_str(), &stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_pin_message(
    state: &AppState,
    auth: &AppContext,
    request: PinMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let _mutation = state
        .rpc_runtime()
        .pin_message(PinMessageCommand::from_auth_context(auth, message_id.clone()))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = PinMessageResponse {
        summary: Some(interaction_summary_from_stored(message_id.as_str(), &stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_unpin_message(
    state: &AppState,
    auth: &AppContext,
    request: UnpinMessageRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let _mutation = state
        .rpc_runtime()
        .unpin_message(UnpinMessageCommand::from_auth_context(auth, message_id.clone()))
        .map_err(map_runtime_error)?;
    let stored = state
        .rpc_runtime()
        .stored_message_from_auth_context(auth, message_id.as_str())
        .map_err(map_runtime_error)?;
    let response = UnpinMessageResponse {
        summary: Some(interaction_summary_from_stored(message_id.as_str(), &stored)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_create_room(
    state: &AppState,
    auth: &AppContext,
    metadata: &RpcMetadata,
    request: CreateRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = if request.conversation_id.trim().is_empty() {
        derive_idempotent_resource_id(metadata, "room-conversation")?
    } else {
        request.conversation_id
    };
    let room_id = if request.room_id.trim().is_empty() {
        derive_idempotent_resource_id(metadata, "room")?
    } else {
        request.room_id
    };
    let room_kind = required_field(request.room_kind, "room_kind")?;
    let result = state
        .rpc_runtime()
        .create_room_from_auth_context(auth, conversation_id, room_id.clone(), room_kind)
        .map_err(map_runtime_error)?;
    let binding = state
        .rpc_runtime()
        .conversation_business_binding_from_auth_context(auth, result.conversation_id.as_str())
        .map_err(map_runtime_error)?;
    let room = state
        .rpc_runtime()
        .room_view_from_auth_context(auth, room_id)
        .map_err(map_runtime_error)?;
    let response = CreateRoomResponse {
        conversation: Some(conversation_view_from_binding(
            result.conversation_id.as_str(),
            &binding,
            String::new(),
        )),
        room: Some(room_view_to_proto(&room)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_retrieve_room(
    state: &AppState,
    auth: &AppContext,
    request: RetrieveRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let room_id = required_field(request.room_id, "room_id")?;
    let room = state
        .rpc_runtime()
        .room_view_from_auth_context(auth, room_id)
        .map_err(map_runtime_error)?;
    let response = RetrieveRoomResponse {
        room: Some(room_view_to_proto(&room)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_enter_room(
    state: &AppState,
    auth: &AppContext,
    request: EnterRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let room_id = required_field(request.room_id, "room_id")?;
    let member = state
        .rpc_runtime()
        .enter_room_from_auth_context(auth, room_id)
        .map_err(map_runtime_error)?;
    let response = EnterRoomResponse {
        member: Some(member_view_from_domain(&member)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

async fn dispatch_leave_room(
    state: &AppState,
    auth: &AppContext,
    request: LeaveRoomRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let room_id = required_field(request.room_id, "room_id")?;
    let member = state
        .rpc_runtime()
        .leave_room_from_auth_context(auth, room_id)
        .map_err(map_runtime_error)?;
    let response = LeaveRoomResponse {
        member: Some(member_view_from_domain(&member)),
        status: membership_state_label(&member.state).into(),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) fn bootstrap_conversation_app_state_from_env() -> Result<AppState, String> {
    if let Some(catalog_path) = std::env::var(PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        let directory = crate::http::StaticPrincipalDirectory::from_json_file(std::path::Path::new(
            catalog_path.as_str(),
        ))?;
        return Ok(crate::http::app_state_with_principal_directory(Arc::new(directory)));
    }

    let allow_all = std::env::var(ALLOW_ALL_PRINCIPALS_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        });
    if allow_all {
        tracing::warn!(
            "{} is enabled - all principals are allowed without verification; \
             this must never be used in production",
            ALLOW_ALL_PRINCIPALS_ENV
        );
        return Ok(crate::http::default_app_state());
    }

    Err(format!(
        "principal directory is required: set {PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV} to a JSON catalog file path, \
         or set {ALLOW_ALL_PRINCIPALS_ENV}=true for development-only mode"
    ))
}

fn resolve_handoff_target_from_source(
    state: &AppState,
    auth: &AppContext,
    source_conversation_id: &str,
) -> Result<(String, String), ImRpcError> {
    let members = state
        .rpc_runtime()
        .list_members_from_auth_context(auth, source_conversation_id)
        .map_err(map_runtime_error)?;
    members
        .into_iter()
        .find(|member| {
            member.is_active()
                && (member.principal_id != auth.actor_id
                    || member.principal_kind != auth.actor_kind)
        })
        .map(|member| (member.principal_id, member.principal_kind))
        .ok_or_else(|| {
            ImRpcError::failed_precondition(
                "agent handoff requires an active non-source member in source conversation",
            )
        })
}

fn derive_idempotent_resource_id(metadata: &RpcMetadata, namespace: &str) -> Result<String, ImRpcError> {
    let key = metadata
        .idempotency_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ImRpcError::invalid_argument("idempotency-key metadata is required for this RPC write")
        })?;
    Ok(format!("rpc-{namespace}-{}", sha256_hash(key.as_bytes())))
}

fn canonical_direct_chat_id(left_user_id: &str, right_user_id: &str) -> String {
    if left_user_id <= right_user_id {
        format!("{left_user_id}:{right_user_id}")
    } else {
        format!("{right_user_id}:{left_user_id}")
    }
}

fn conversation_view_from_binding(
    conversation_id: &str,
    binding: &im_domain_core::conversation::ConversationBusinessBinding,
    title: String,
) -> ConversationView {
    ConversationView {
        conversation_id: conversation_id.to_owned(),
        conversation_type: binding.business_type.clone(),
        title,
        owner_user_id: binding.business_id.clone(),
        state: "active".into(),
    }
}

fn room_view_to_proto(view: &crate::RoomView) -> RoomView {
    RoomView {
        room_id: view.room_id.clone(),
        room_kind: view.room_kind.clone(),
        conversation_id: view.conversation_id.clone(),
        active_member_count: view.active_member_count as i32,
        max_members: view.max_members as i32,
    }
}

fn member_view_from_domain(member: &ConversationMember) -> ConversationMemberView {
    ConversationMemberView {
        conversation_id: member.conversation_id.clone(),
        user_id: member.principal_id.clone(),
        role: membership_role_label(member.role.clone()).into(),
        state: membership_state_label(&member.state).into(),
    }
}

fn read_cursor_view_from_domain(
    cursor: &im_domain_core::conversation::ConversationReadCursorView,
) -> ReadCursorView {
    ReadCursorView {
        conversation_id: cursor.conversation_id.clone(),
        user_id: cursor.principal_id.clone(),
        message_id: cursor.last_read_message_id.clone().unwrap_or_default(),
        event_cursor: cursor.read_seq.to_string(),
    }
}

fn message_view_from_stored(stored: &im_domain_core::message::StoredMessage) -> MessageView {
    MessageView {
        message_id: stored.message.message_id.clone(),
        conversation_id: stored.message.conversation_id.clone(),
        sender_user_id: stored.message.sender.id.clone(),
        body_parts: stored
            .message
            .body
            .parts
            .iter()
            .map(content_part_to_proto)
            .collect(),
        state: if stored.recalled {
            "recalled".into()
        } else {
            "active".into()
        },
        created_at: stored.message.occurred_at.clone(),
    }
}

fn message_mutation_response_from_stored(
    stored: &im_domain_core::message::StoredMessage,
    _event_id: String,
) -> MessageMutationResponse {
    MessageMutationResponse {
        message: Some(message_view_from_stored(stored)),
        status: "applied".into(),
        metadata: None,
    }
}

fn interaction_summary_from_stored(
    message_id: &str,
    stored: &im_domain_core::message::StoredMessage,
) -> MessageInteractionSummaryView {
    let reaction_count = stored
        .reactions
        .values()
        .map(|actors| actors.len())
        .sum::<usize>() as i64;
    MessageInteractionSummaryView {
        message_id: message_id.to_owned(),
        reaction_count,
        reply_count: if stored.message.body.reply_to.is_some() {
            1
        } else {
            0
        },
        pinned: stored.pin.is_some(),
        favorited: false,
    }
}

fn content_part_to_proto(part: &ContentPart) -> MessageBodyPart {
    match part {
        ContentPart::Text(text_part) => MessageBodyPart {
            kind: "text".into(),
            text: text_part.text.clone(),
            media: None,
            payload_json: String::new(),
        },
        ContentPart::Data(data_part) => MessageBodyPart {
            kind: "data".into(),
            text: String::new(),
            media: None,
            payload_json: data_part.payload.clone(),
        },
        ContentPart::Media(media_part) => MessageBodyPart {
            kind: "media".into(),
            text: String::new(),
            media: Some(domain_media_resource_to_proto(&media_part.resource, &media_part.drive)),
            payload_json: String::new(),
        },
        ContentPart::Signal(signal_part) => MessageBodyPart {
            kind: "signal".into(),
            text: signal_part.signal_type.clone(),
            media: None,
            payload_json: signal_part.payload.clone(),
        },
        ContentPart::StreamRef(stream_part) => MessageBodyPart {
            kind: "stream_ref".into(),
            text: stream_part.stream_type.clone(),
            media: None,
            payload_json: stream_part.stream_id.clone(),
        },
    }
}

fn domain_media_resource_to_proto(
    resource: &im_domain_core::media::MediaResource,
    drive: &im_domain_core::media::DriveReference,
) -> sdkwork_im_rpc_sdk_rust::sdkwork::common::v1::MediaResource {
    sdkwork_im_rpc_sdk_rust::sdkwork::common::v1::MediaResource {
        media_id: resource.id.clone().unwrap_or_default(),
        source: resource.source.as_wire_value().to_owned(),
        kind: resource.kind.as_wire_value().to_owned(),
        content_type: resource.mime_type.clone().unwrap_or_default(),
        filename: resource.file_name.clone().unwrap_or_default(),
        file_size_bytes: resource.content_length().unwrap_or_default() as i64,
        width: resource.width.unwrap_or_default() as i32,
        height: resource.height.unwrap_or_default() as i32,
        duration_ms: resource
            .duration_seconds
            .map(|seconds| (seconds as i32) * 1000)
            .unwrap_or_default(),
        checksum: resource
            .checksum
            .as_ref()
            .map(|checksum| checksum.value.clone())
            .unwrap_or_default(),
        access: resource
            .access
            .as_ref()
            .map(|access| format!("{:?}", access.visibility).to_ascii_lowercase())
            .unwrap_or_default(),
        expires_at: resource
            .access
            .as_ref()
            .and_then(|access| access.expires_at.clone())
            .unwrap_or_default(),
        drive: Some(sdkwork_im_rpc_sdk_rust::sdkwork::common::v1::DriveReference {
            space_id: drive.space_id.clone(),
            node_id: drive.node_id.clone(),
            drive_uri: drive.drive_uri.clone(),
            upload_session_id: String::new(),
        }),
        metadata: std::collections::HashMap::new(),
    }
}

fn proto_parts_to_message_body(
    parts: Vec<MessageBodyPart>,
    reply_to: Option<MessageReplyReference>,
) -> Result<im_domain_core::message::MessageBody, http::ApiError> {
    let mut content_parts = Vec::new();
    for part in parts {
        let kind = part.kind.trim();
        if kind.is_empty() || kind == "text" {
            if !part.text.trim().is_empty() {
                content_parts.push(ContentPart::text(part.text));
            }
            continue;
        }
        if kind == "data" {
            content_parts.push(ContentPart::Data(im_domain_core::message::DataPart {
                schema_ref: String::new(),
                encoding: "json".into(),
                payload: part.payload_json,
            }));
            continue;
        }
        if kind == "media" {
            if let Some(media) = part.media {
                if let Some(drive) = media.drive {
                    content_parts.push(ContentPart::media(im_domain_core::message::MediaPart {
                        resource: im_domain_core::media::MediaResource {
                            id: optional_string(media.media_id),
                            kind: im_domain_core::media::MediaKind::Other,
                            source: im_domain_core::media::MediaSource::Drive,
                            url: None,
                            public_url: None,
                            uri: None,
                            object_blob_id: None,
                            file_name: optional_string(media.filename),
                            mime_type: optional_string(media.content_type),
                            size_bytes: Some(media.file_size_bytes.to_string()),
                            checksum: None,
                            width: Some(media.width.max(0) as u32),
                            height: Some(media.height.max(0) as u32),
                            duration_seconds: Some((media.duration_ms.max(0) as u32) / 1000),
                            alt_text: None,
                            title: None,
                            poster: None,
                            thumbnails: None,
                            variants: None,
                            access: None,
                            ai: None,
                            metadata: None,
                        },
                        drive: im_domain_core::media::DriveReference {
                            drive_uri: drive.drive_uri,
                            space_id: drive.space_id,
                            node_id: drive.node_id,
                            node_version: None,
                        },
                        media_role: Some("attachment".into()),
                    }));
                }
            }
            continue;
        }
        if !part.payload_json.trim().is_empty() {
            content_parts.push(ContentPart::Data(im_domain_core::message::DataPart {
                schema_ref: kind.to_owned(),
                encoding: "json".into(),
                payload: part.payload_json,
            }));
        } else if !part.text.trim().is_empty() {
            content_parts.push(ContentPart::text(part.text));
        }
    }
    http::build_rpc_message_body(content_parts, reply_to)
}

fn page_request(page: Option<PageRequest>) -> (usize, Option<String>) {
    let limit = page
        .as_ref()
        .map(|value| value.page_size.max(0) as usize)
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .min(MAX_PAGE_SIZE);
    let cursor = page
        .and_then(|value| optional_string(value.cursor))
        .filter(|value| !value.is_empty());
    (limit, cursor)
}

fn page_response(
    next_cursor: Option<String>,
    has_more: bool,
    item_count: usize,
) -> PageResponse {
    PageResponse {
        next_cursor: next_cursor.unwrap_or_default(),
        has_more,
        total_count: item_count as i64,
    }
}

/// Build standard app-session RPC metadata from a local dual-token `AppContext`.
pub fn rpc_metadata_from_app_context(
    context: &AppContext,
    idempotency_key: Option<String>,
    request_id: Option<String>,
) -> RpcMetadata {
    use im_app_context::build_dual_token_headers_for_context;

    let headers = build_dual_token_headers_for_context(context, context.permission_scope.iter());
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let access_token = headers
        .get("access-token")
        .or_else(|| headers.get("Access-Token"))
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    RpcMetadata {
        authorization,
        access_token,
        idempotency_key,
        request_id,
        ..RpcMetadata::default()
    }
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
    if let Some(value) = &metadata.request_id {
        if let Ok(parsed) = HeaderValue::from_str(value) {
            headers.insert("x-request-id", parsed);
        }
    }
    if let Some(value) = &metadata.idempotency_key {
        if let Ok(parsed) = HeaderValue::from_str(value) {
            headers.insert("idempotency-key", parsed);
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

fn required_field(value: String, field: &str) -> Result<String, ImRpcError> {
    optional_string(value).ok_or_else(|| ImRpcError::invalid_argument(format!("{field} is required")))
}

fn parse_cursor_u64(cursor: &str) -> Result<u64, ImRpcError> {
    let trimmed = cursor.trim();
    if trimmed.is_empty() {
        return Ok(0);
    }
    trimmed
        .parse::<u64>()
        .map_err(|error| ImRpcError::invalid_argument(format!("invalid cursor `{cursor}`: {error}")))
}

fn parse_membership_role(role: &str) -> MembershipRole {
    match role.trim().to_ascii_lowercase().as_str() {
        "owner" => MembershipRole::Owner,
        "admin" => MembershipRole::Admin,
        "guest" => MembershipRole::Guest,
        _ => MembershipRole::Member,
    }
}

fn membership_role_label(role: MembershipRole) -> &'static str {
    match role {
        MembershipRole::Owner => "owner",
        MembershipRole::Admin => "admin",
        MembershipRole::Member => "member",
        MembershipRole::Guest => "guest",
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

fn map_runtime_error(error: RuntimeError) -> ImRpcError {
    map_api_error(error.into())
}

fn map_api_error(error: http::ApiError) -> ImRpcError {
    http::map_api_error_to_im_rpc(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_rpc_service_keys_cover_write_message_and_room_surfaces() {
        assert_eq!(CONVERSATION_RPC_SERVICE_KEYS.len(), 3);
        assert!(CONVERSATION_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("ConversationService")));
        assert!(CONVERSATION_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("MessageService")));
        assert!(CONVERSATION_RPC_SERVICE_KEYS
            .iter()
            .any(|key| key.ends_with("RoomService")));
    }

    #[test]
    fn canonical_direct_chat_id_orders_participants() {
        assert_eq!(
            canonical_direct_chat_id("1068", "1067"),
            "1067:1068"
        );
    }

    #[test]
    fn derive_idempotent_resource_id_requires_metadata_key() {
        let error = derive_idempotent_resource_id(&RpcMetadata::default(), "conversation")
            .expect_err("missing idempotency key");
        assert!(error.message().contains("idempotency-key"));
    }

    #[test]
    fn rpc_metadata_from_app_context_includes_dual_token_headers() {
        let context = im_app_context::local_service_app_context(
            "100001",
            "1",
            "user",
            Some("d_test"),
            ["*"],
        );
        let metadata = rpc_metadata_from_app_context(
            &context,
            Some("idem-1".into()),
            Some("req-1".into()),
        );
        assert!(metadata.authorization.as_deref().is_some_and(|v| v.starts_with("Bearer ")));
        assert!(metadata.access_token.as_deref().is_some_and(|v| !v.is_empty()));
        assert_eq!(metadata.idempotency_key.as_deref(), Some("idem-1"));
        assert_eq!(metadata.request_id.as_deref(), Some("req-1"));
    }
}
