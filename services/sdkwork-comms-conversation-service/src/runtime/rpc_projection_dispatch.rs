//! RPC dispatch for inbox/projection-owned conversation and message surfaces.

use im_app_context::AppContext;
use projection_service::{
    FavoriteMessageRequest, ProjectionAccessError,
    UpdateConversationPreferencesRequest as ProjectionUpdateConversationPreferencesRequest,
    UpdateConversationProfileRequest as ProjectionUpdateConversationProfileRequest,
    default_projection_service,
};
use sdkwork_im_rpc_sdk_rust::sdkwork::communication::app::v3::{
    ConversationPreferencesView as RpcConversationPreferencesView,
    ConversationProfileView as RpcConversationProfileView, CreateMessageFavoriteRequest,
    CreateMessageFavoriteResponse, DeleteMessageFavoriteRequest, DeleteMessageFavoriteResponse,
    DeleteMessageVisibilityRequest, DeleteMessageVisibilityResponse, ListFavoriteMessagesRequest,
    ListFavoriteMessagesResponse, MessageFavoriteView as RpcMessageFavoriteView,
    RetrieveConversationPreferencesRequest, RetrieveConversationPreferencesResponse,
    RetrieveConversationProfileRequest, RetrieveConversationProfileResponse,
    UpdateConversationPreferencesRequest, UpdateConversationPreferencesResponse,
    UpdateConversationProfileRequest, UpdateConversationProfileResponse,
};
use sdkwork_im_rpc_service_rust::{ImRpcError, ImRpcUnaryResponse};

use super::message_realtime::ConversationRealtimeEvent;
use super::rpc_dispatch::{map_api_error, page_request, page_response, required_field};

fn projection_service() -> std::sync::Arc<projection_service::TimelineProjectionService> {
    default_projection_service()
}

fn map_projection_error(error: ProjectionAccessError) -> ImRpcError {
    map_api_error(error.into())
}

fn rpc_preferences_view(
    view: projection_service::ConversationPreferencesView,
) -> RpcConversationPreferencesView {
    RpcConversationPreferencesView {
        conversation_id: view.conversation_id,
        muted: view.is_muted,
        pinned: view.is_pinned,
    }
}

fn rpc_profile_view(
    view: projection_service::ConversationProfileView,
) -> RpcConversationProfileView {
    RpcConversationProfileView {
        conversation_id: view.conversation_id,
        title: view.display_name,
        avatar_uri: view.avatar_url,
        description: view.notice,
    }
}

fn rpc_favorite_view(view: projection_service::MessageFavoriteView) -> RpcMessageFavoriteView {
    RpcMessageFavoriteView {
        favorite_id: view.favorite_id,
        message_id: view.message_id,
        created_at: view.favorited_at,
    }
}

fn optional_non_empty(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

pub(crate) async fn dispatch_retrieve_conversation_preferences(
    auth: &AppContext,
    request: RetrieveConversationPreferencesRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let preferences = projection_service()
        .conversation_preferences_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_projection_error)?;
    let response = RetrieveConversationPreferencesResponse {
        preferences: Some(rpc_preferences_view(preferences)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_update_conversation_preferences(
    auth: &AppContext,
    request: UpdateConversationPreferencesRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let update = ProjectionUpdateConversationPreferencesRequest {
        is_muted: Some(request.muted),
        is_pinned: Some(request.pinned),
        is_marked_unread: None,
        is_hidden: None,
    };
    let preferences = projection_service()
        .update_conversation_preferences_from_auth_context(auth, conversation_id.as_str(), update)
        .map_err(map_projection_error)?;
    let response = UpdateConversationPreferencesResponse {
        preferences: Some(rpc_preferences_view(preferences)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_retrieve_conversation_profile(
    auth: &AppContext,
    request: RetrieveConversationProfileRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let profile = projection_service()
        .conversation_profile_from_auth_context(auth, conversation_id.as_str())
        .map_err(map_projection_error)?;
    let response = RetrieveConversationProfileResponse {
        profile: Some(rpc_profile_view(profile)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_update_conversation_profile(
    state: &super::http::AppState,
    auth: &AppContext,
    request: UpdateConversationProfileRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let conversation_id = required_field(request.conversation_id, "conversation_id")?;
    let update = ProjectionUpdateConversationProfileRequest {
        display_name: optional_non_empty(request.title),
        avatar_url: optional_non_empty(request.avatar_uri),
        notice: optional_non_empty(request.description),
    };
    let profile = projection_service()
        .update_conversation_profile_from_auth_context(auth, conversation_id.as_str(), update)
        .map_err(map_projection_error)?;
    let realtime_payload = serde_json::json!({
        "conversationId": conversation_id,
        "displayName": profile.display_name.clone(),
        "avatarUrl": profile.avatar_url.clone(),
        "notice": profile.notice.clone(),
        "updatedAt": profile.updated_at.clone(),
    })
    .to_string();
    let event_id = format!(
        "conversation:profile.updated:{}:{}",
        conversation_id, profile.updated_at
    );
    if let Err(error) =
        state
            .rpc_runtime()
            .publish_or_enqueue_conversation_event(ConversationRealtimeEvent {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: auth.organization_id.as_str(),
                conversation_id: conversation_id.as_str(),
                event_type: "conversation.updated",
                journal_event_id: event_id.as_str(),
                payload_json: realtime_payload,
                occurred_at: profile.updated_at.as_str(),
            })
    {
        tracing::warn!(
            conversation_id = %conversation_id,
            error = ?error,
            "conversation.updated realtime delivery failed after profile commit"
        );
    }
    let response = UpdateConversationProfileResponse {
        profile: Some(rpc_profile_view(profile)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_list_favorite_messages(
    auth: &AppContext,
    request: ListFavoriteMessagesRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let (limit, cursor) = page_request(request.page)?;
    let window = projection_service()
        .message_favorites_window_from_auth_context(
            auth,
            Some(limit),
            cursor.as_deref(),
            None,
            None,
        )
        .map_err(map_projection_error)?;
    let favorite_count = window.items.len();
    let response = ListFavoriteMessagesResponse {
        favorites: window.items.into_iter().map(rpc_favorite_view).collect(),
        page: Some(page_response(
            window.page_info.next_cursor.clone(),
            window.page_info.has_more == Some(true),
            favorite_count,
        )),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_create_message_favorite(
    auth: &AppContext,
    request: CreateMessageFavoriteRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let service = projection_service();
    let organization_id =
        im_platform_contracts::normalize_realtime_organization_id(auth.organization_id.as_str());
    let conversation_id = service
        .conversation_id_for_message(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            message_id.as_str(),
        )
        .ok_or_else(|| ImRpcError::not_found(format!("message not found: {message_id}")))?;
    let favorite_request = FavoriteMessageRequest {
        conversation_id,
        favorite_type: "message".into(),
        title: String::new(),
        content_preview: String::new(),
        source_display_name: String::new(),
    };
    let favorite = service
        .create_message_favorite_from_auth_context(auth, message_id.as_str(), favorite_request)
        .map_err(map_projection_error)?;
    let response = CreateMessageFavoriteResponse {
        favorite: Some(rpc_favorite_view(favorite)),
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_delete_message_favorite(
    auth: &AppContext,
    request: DeleteMessageFavoriteRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let favorite_id = required_field(request.favorite_id, "favorite_id")?;
    let deleted = projection_service()
        .delete_message_favorite_from_auth_context(auth, favorite_id.as_str())
        .map_err(map_projection_error)?;
    let response = DeleteMessageFavoriteResponse {
        favorite_id: deleted.favorite_id,
        status: if deleted.deleted {
            "deleted".into()
        } else {
            "unchanged".into()
        },
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

pub(crate) async fn dispatch_delete_message_visibility(
    auth: &AppContext,
    request: DeleteMessageVisibilityRequest,
) -> Result<ImRpcUnaryResponse, ImRpcError> {
    let message_id = required_field(request.message_id, "message_id")?;
    let result = projection_service()
        .delete_message_visibility_from_auth_context(auth, message_id.as_str())
        .map_err(map_projection_error)?;
    let response = DeleteMessageVisibilityResponse {
        message_id: result.message_id,
        status: if result.is_deleted {
            "deleted".into()
        } else {
            "visible".into()
        },
        metadata: None,
    };
    ImRpcUnaryResponse::from_message(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpc_preferences_view_maps_muted_and_pinned_fields() {
        let view = projection_service::ConversationPreferencesView {
            tenant_id: "tenant".into(),
            conversation_id: "conv-1".into(),
            principal_kind: "user".into(),
            principal_id: "user-1".into(),
            is_pinned: true,
            is_muted: false,
            is_marked_unread: true,
            is_hidden: false,
            updated_at: "2026-01-01T00:00:00.000Z".into(),
        };
        let mapped = rpc_preferences_view(view);
        assert_eq!(mapped.conversation_id, "conv-1");
        assert!(mapped.pinned);
        assert!(!mapped.muted);
    }
}
