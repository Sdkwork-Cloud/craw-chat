use axum::http::StatusCode;
use im_app_context::AppContext;
use im_domain_core::conversation::{
    history_read_allowed, ConversationInboxEntry, ConversationMember, ConversationReadCursorView,
};
use im_domain_core::social::DirectChatStatus;
use im_platform_contracts::normalize_realtime_organization_id;

use super::{
    ClientRouteSyncFeedWindowView, ContactView, ContactWindowView,
    ConversationMemberDirectoryEntry, ConversationPreferencesView, ConversationProfileView,
    ConversationSummaryView, DeleteMessageFavoriteResponse, FavoriteMessageRequest,
    FavoriteMessagesWindowView, MessageFavoriteView, MessageInteractionSummaryView,
    NotificationRecipientView, PROJECTION_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT,
    PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT, PROJECTION_LIST_DEFAULT_LIMIT,
    PROJECTION_LIST_MAX_LIMIT, PROJECTION_TIMELINE_DEFAULT_LIMIT, PROJECTION_TIMELINE_MAX_LIMIT,
    RealtimeFanoutTarget, RegisteredClientRouteView, TimelineProjectionService, TimelineWindowView,
    UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
};
use super::message_favorites::filter_message_favorites;

const PROJECTION_MAX_DEVICE_ID_BYTES: usize = 256;
const PROJECTION_MAX_CONVERSATION_ID_BYTES: usize = 256;
const PROJECTION_MAX_MESSAGE_ID_BYTES: usize = 256;

#[derive(Debug)]
pub struct ProjectionAccessError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientRouteSyncStateSnapshot {
    pub registered_client_routes: Vec<String>,
    pub latest_sync_seq: Option<u64>,
}

impl ProjectionAccessError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }

    fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }
}

impl TimelineProjectionService {
    fn auth_organization_id(auth: &AppContext) -> String {
        normalize_realtime_organization_id(auth.organization_id.as_str())
    }

    fn direct_chat_binding_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<super::model::ContactDirectChatBindingView> {
        super::lock_projection_mutex(
            &self.direct_chat_bindings,
            "contact direct chat binding store",
        )
        .get_by_conversation(tenant_id, organization_id, conversation_id)
        .cloned()
    }

    fn ensure_conversation_not_archived_direct_chat(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        let Some(binding) =
            self.direct_chat_binding_for_conversation(tenant_id, organization_id, conversation_id)
        else {
            return Ok(());
        };
        if binding.status != DirectChatStatus::Archived {
            return Ok(());
        }

        Err(ProjectionAccessError::forbidden(
            "conversation_archived",
            format!("direct chat conversation is archived: {conversation_id}"),
        ))
    }

    pub fn is_archived_direct_chat_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> bool {
        self.direct_chat_binding_for_conversation(tenant_id, organization_id, conversation_id)
            .is_some_and(|binding| binding.status == DirectChatStatus::Archived)
    }

    pub fn direct_chat_id_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<String> {
        self.direct_chat_binding_for_conversation(tenant_id, organization_id, conversation_id)
            .map(|binding| binding.direct_chat_id)
    }

    pub fn ensure_active_member_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        validate_conversation_id(conversation_id)?;
        self.ensure_conversation_not_archived_direct_chat(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        )?;
        let organization_id = Self::auth_organization_id(auth);
        let is_active = self
            .member_snapshot_for_principal_kind(
                auth.tenant_id.as_str(),
                organization_id.as_str(),
                conversation_id,
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
            .is_some_and(|member| member.is_active());
        if is_active {
            return Ok(());
        }

        Err(ProjectionAccessError::forbidden(
            "conversation_permission_denied",
            format!(
                "principal is not active conversation member: {}",
                auth.actor_id
            ),
        ))
    }

    pub fn conversation_profile_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationProfileView, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.conversation_profile(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        ))
    }

    pub fn update_conversation_profile_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        update: UpdateConversationProfileRequest,
    ) -> Result<ConversationProfileView, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.update_conversation_profile(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            update,
        ))
    }

    pub fn conversation_preferences_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationPreferencesView, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.conversation_preferences(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        ))
    }

    pub fn update_conversation_preferences_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        update: UpdateConversationPreferencesRequest,
    ) -> Result<ConversationPreferencesView, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.update_conversation_preferences(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            update,
        ))
    }

    pub fn ensure_history_reader_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<(), ProjectionAccessError> {
        validate_conversation_id(conversation_id)?;
        self.ensure_conversation_not_archived_direct_chat(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        )?;
        let organization_id = Self::auth_organization_id(auth);
        let history_visibility = self.history_visibility_for_conversation(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        );
        let member = self.member_snapshot_for_principal_kind(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        if history_read_allowed(history_visibility.as_str(), member.as_ref()) {
            return Ok(());
        }

        Err(ProjectionAccessError::forbidden(
            "conversation_permission_denied",
            format!(
                "principal cannot read conversation history: {}",
                auth.actor_id
            ),
        ))
    }

    fn history_read_allowed_for_member(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member: &ConversationMember,
    ) -> bool {
        let history_visibility =
            self.history_visibility_for_conversation(tenant_id, organization_id, conversation_id);
        history_read_allowed(history_visibility.as_str(), Some(member))
    }

    pub fn active_conversation_principal_recipients_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<NotificationRecipientView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(
            super::client_route_sync::active_conversation_principal_recipients(
                self,
                auth.tenant_id.as_str(),
                Self::auth_organization_id(auth).as_str(),
                conversation_id,
            ),
        )
    }

    pub fn message_posted_notification_recipients_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<NotificationRecipientView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.message_posted_notification_recipients(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        ))
    }

    pub(crate) fn message_posted_notification_recipients(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Vec<NotificationRecipientView> {
        let scope = super::scope::scope_key(tenant_id, organization_id, conversation_id);
        let mut recipients = super::lock_projection_mutex(&self.members, "member store")
            .get(scope.as_str())
            .map(|scope_members| {
                scope_members
                    .values()
                    .filter(|member| {
                        self.history_read_allowed_for_member(
                            tenant_id,
                            organization_id,
                            conversation_id,
                            member,
                        )
                    })
                    .map(|member| NotificationRecipientView {
                        principal_id: member.principal_id.clone(),
                        principal_kind: member.principal_kind.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        recipients.sort();
        recipients.dedup();
        recipients
    }

    pub fn register_client_route_from_auth_context(
        &self,
        auth: &AppContext,
        requested_device_id: Option<String>,
    ) -> Result<RegisteredClientRouteView, ProjectionAccessError> {
        let device_id = self.ensure_client_route_registration_allowed_from_auth_context(
            auth,
            requested_device_id,
        )?;
        Ok(self.register_client_route_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
        ))
    }

    pub fn ensure_client_route_registration_allowed_from_auth_context(
        &self,
        auth: &AppContext,
        requested_device_id: Option<String>,
    ) -> Result<String, ProjectionAccessError> {
        let device_id = resolve_requested_device_id(auth, requested_device_id)?;
        ensure_client_route_registration_available(self, auth, device_id.as_str())?;
        Ok(device_id)
    }

    pub fn registered_client_routes_from_auth_context(
        &self,
        auth: &AppContext,
    ) -> Vec<RegisteredClientRouteView> {
        self.registered_client_routes_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
    }

    pub fn client_route_sync_state_snapshot_from_auth_context(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<ClientRouteSyncStateSnapshot, ProjectionAccessError> {
        let registered_client_routes = self
            .registered_client_routes_from_auth_context(auth)
            .into_iter()
            .map(|item| item.device_id)
            .collect::<Vec<_>>();
        let latest_sync_seq = match requested_device_id.or(auth.device_id.as_deref()) {
            Some(device_id) => {
                validate_device_scope(auth, device_id)?;
                ensure_client_route_owned_by_auth_kind(self, auth, device_id)?;
                Some(self.latest_client_route_sync_seq_for_principal_kind(
                    auth.tenant_id.as_str(),
                    normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
                    auth.actor_id.as_str(),
                    auth.actor_kind.as_str(),
                    device_id,
                ))
            }
            None => None,
        };

        Ok(ClientRouteSyncStateSnapshot {
            registered_client_routes,
            latest_sync_seq,
        })
    }

    pub fn realtime_fanout_targets_for_recipients_from_auth_context(
        &self,
        auth: &AppContext,
        recipients: impl IntoIterator<Item = NotificationRecipientView>,
    ) -> Vec<RealtimeFanoutTarget> {
        super::client_route_sync::realtime_fanout_targets_for_recipients(
            self,
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            recipients,
        )
    }

    pub fn latest_client_route_sync_seq_from_auth_context(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<u64, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_client_route_owned_by_auth_kind(self, auth, device_id)?;
        Ok(self.latest_client_route_sync_seq_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
        ))
    }

    pub fn client_route_sync_feed_window_from_auth_context(
        &self,
        auth: &AppContext,
        device_id: &str,
        after_seq: Option<u64>,
        limit: Option<usize>,
    ) -> Result<ClientRouteSyncFeedWindowView, ProjectionAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_client_route_owned_by_auth_kind(self, auth, device_id)?;
        let limit = validate_client_route_sync_feed_limit(limit)?;
        Ok(self.client_route_sync_feed_window_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
            after_seq,
            limit,
        ))
    }

    pub fn inbox_from_auth_context(&self, auth: &AppContext) -> Vec<ConversationInboxEntry> {
        self.inbox_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
        .into_iter()
        .filter(|entry| {
            !self.is_archived_direct_chat_conversation(
                auth.tenant_id.as_str(),
                Self::auth_organization_id(auth).as_str(),
                entry.conversation_id.as_str(),
            )
        })
        .collect()
    }

    pub fn inbox_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<super::InboxWindowView, ProjectionAccessError> {
        let limit = validate_list_limit(limit)?;
        list_window(self.inbox_from_auth_context(auth), limit, cursor).map(|window| {
            super::InboxWindowView {
                items: window.items,
                next_cursor: window.next_cursor,
                has_more: window.has_more,
            }
        })
    }

    pub fn contacts_from_auth_context(
        &self,
        auth: &AppContext,
    ) -> Result<Vec<ContactView>, ProjectionAccessError> {
        ensure_user_contact_owner(auth)?;
        Ok(self.contacts(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            auth.actor_id.as_str(),
        ))
    }

    pub fn contact_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<ContactWindowView, ProjectionAccessError> {
        let limit = validate_list_limit(limit)?;
        let contacts = self.contacts_from_auth_context(auth)?;
        list_window(contacts, limit, cursor).map(|window| ContactWindowView {
            items: window.items,
            next_cursor: window.next_cursor,
            has_more: window.has_more,
        })
    }

    pub fn timeline_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        after_seq: Option<u64>,
        limit: Option<usize>,
    ) -> Result<TimelineWindowView, ProjectionAccessError> {
        self.ensure_history_reader_from_auth_context(auth, conversation_id)?;
        let limit = validate_timeline_limit(limit)?;
        let organization_id = Self::auth_organization_id(auth);
        Ok(self.timeline_window(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            after_seq,
            limit,
        ))
    }

    pub fn conversation_summary_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationSummaryView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.conversation_summary(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        ))
    }

    pub fn message_interaction_summary_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<Option<MessageInteractionSummaryView>, ProjectionAccessError> {
        validate_conversation_id(conversation_id)?;
        validate_message_id(message_id)?;
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.message_interaction_summary(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            message_id,
        ))
    }

    pub fn pinned_messages_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<MessageInteractionSummaryView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.pinned_messages(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        ))
    }

    pub fn read_cursor_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationReadCursorView>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.read_cursor_for_principal_kind(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        ))
    }

    pub fn member_directory_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMemberDirectoryEntry>, ProjectionAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.member_directory(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        ))
    }

    pub fn message_favorites_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
        favorite_type: Option<&str>,
        query: Option<&str>,
    ) -> Result<FavoriteMessagesWindowView, ProjectionAccessError> {
        let limit = validate_list_limit(limit)?;
        let favorites = filter_message_favorites(
            self.message_favorites_for_principal(
                auth.tenant_id.as_str(),
                Self::auth_organization_id(auth).as_str(),
                auth.actor_kind.as_str(),
                auth.actor_id.as_str(),
            ),
            favorite_type,
            query,
        );
        list_window(favorites, limit, cursor).map(|window| FavoriteMessagesWindowView {
            items: window.items,
            next_cursor: window.next_cursor,
            has_more: window.has_more,
        })
    }

    pub fn create_message_favorite_from_auth_context(
        &self,
        auth: &AppContext,
        message_id: &str,
        request: FavoriteMessageRequest,
    ) -> Result<MessageFavoriteView, ProjectionAccessError> {
        validate_message_id(message_id)?;
        validate_conversation_id(request.conversation_id.as_str())?;
        self.ensure_active_member_from_auth_context(auth, request.conversation_id.as_str())?;
        Ok(self.create_message_favorite(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            message_id,
            request,
        ))
    }

    pub fn delete_message_favorite_from_auth_context(
        &self,
        auth: &AppContext,
        favorite_id: &str,
    ) -> Result<DeleteMessageFavoriteResponse, ProjectionAccessError> {
        if favorite_id.trim().is_empty() {
            return Err(ProjectionAccessError::bad_request(
                "favorite_id_invalid",
                "favorite id must not be empty",
            ));
        }
        let deleted = self.delete_message_favorite(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            favorite_id,
        );
        if !deleted {
            return Err(ProjectionAccessError {
                status: StatusCode::NOT_FOUND,
                code: "message_favorite_not_found",
                message: format!("message favorite not found: {favorite_id}"),
            });
        }
        Ok(DeleteMessageFavoriteResponse {
            favorite_id: favorite_id.to_owned(),
            deleted: true,
        })
    }
}

fn resolve_requested_device_id(
    auth: &AppContext,
    requested_device_id: Option<String>,
) -> Result<String, ProjectionAccessError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(ProjectionAccessError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => {
            validate_device_id(requested.as_str())?;
            Ok(requested)
        }
        (None, Some(bound)) => {
            validate_device_id(bound.as_str())?;
            Ok(bound)
        }
        (None, None) => Err(ProjectionAccessError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AppContext, device_id: &str) -> Result<(), ProjectionAccessError> {
    validate_device_id(device_id)?;
    if let Some(bound_device_id) = auth.device_id.as_deref() {
        validate_device_id(bound_device_id)?;
        if bound_device_id != device_id {
            return Err(ProjectionAccessError::forbidden(
                "device_scope_forbidden",
                format!("device scope forbidden: {device_id}"),
            ));
        }
    }
    Ok(())
}

fn ensure_client_route_registration_available(
    service: &TimelineProjectionService,
    auth: &AppContext,
    device_id: &str,
) -> Result<(), ProjectionAccessError> {
    let has_conflict = super::lock_projection_mutex(
        &service.registered_client_routes,
        "registered client route store",
    )
    .iter()
    .filter(|(scope, devices)| {
        scope.tenant_id == auth.tenant_id.as_str() && devices.contains_key(device_id)
    })
    .filter_map(|(_, devices)| devices.get(device_id))
    .any(|client_route| !client_route_registration_is_compatible_with_auth(client_route, auth));

    if has_conflict {
        return Err(ProjectionAccessError::conflict(
            "device_scope_conflict",
            format!("device scope already bound to a different principal: {device_id}"),
        ));
    }

    Ok(())
}

fn client_route_registration_is_compatible_with_auth(
    client_route: &RegisteredClientRouteView,
    auth: &AppContext,
) -> bool {
    client_route.principal_id == auth.actor_id
        && (client_route.principal_kind == auth.actor_kind
            || matches!(
                (
                    client_route.principal_kind.as_str(),
                    auth.actor_kind.as_str()
                ),
                ("user", "device") | ("device", "user")
            ))
}

fn ensure_client_route_owned_by_auth_kind(
    service: &TimelineProjectionService,
    auth: &AppContext,
    device_id: &str,
) -> Result<(), ProjectionAccessError> {
    if service
        .registered_client_routes_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )
        .into_iter()
        .any(|device| device.device_id == device_id)
    {
        return Ok(());
    }

    Err(ProjectionAccessError::forbidden(
        "device_scope_forbidden",
        format!("device scope forbidden: {device_id}"),
    ))
}

fn ensure_user_contact_owner(auth: &AppContext) -> Result<(), ProjectionAccessError> {
    if auth.actor_kind == "user" {
        return Ok(());
    }

    Err(ProjectionAccessError::forbidden(
        "contact_scope_forbidden",
        format!(
            "contact scope forbidden for actor kind: {}",
            auth.actor_kind
        ),
    ))
}

fn validate_device_id(device_id: &str) -> Result<(), ProjectionAccessError> {
    let actual_bytes = device_id.len();
    if actual_bytes > PROJECTION_MAX_DEVICE_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "deviceId",
            PROJECTION_MAX_DEVICE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_conversation_id(conversation_id: &str) -> Result<(), ProjectionAccessError> {
    let actual_bytes = conversation_id.len();
    if actual_bytes > PROJECTION_MAX_CONVERSATION_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "conversationId",
            PROJECTION_MAX_CONVERSATION_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_message_id(message_id: &str) -> Result<(), ProjectionAccessError> {
    if message_id.trim().is_empty() {
        return Err(ProjectionAccessError::bad_request(
            "invalid_message_id",
            "messageId is required",
        ));
    }
    let actual_bytes = message_id.len();
    if actual_bytes > PROJECTION_MAX_MESSAGE_ID_BYTES {
        return Err(ProjectionAccessError::payload_too_large(
            "messageId",
            PROJECTION_MAX_MESSAGE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_timeline_limit(limit: Option<usize>) -> Result<usize, ProjectionAccessError> {
    let limit = limit.unwrap_or(PROJECTION_TIMELINE_DEFAULT_LIMIT);
    if limit == 0 || limit > PROJECTION_TIMELINE_MAX_LIMIT {
        return Err(ProjectionAccessError::bad_request(
            "limit_invalid",
            format!(
                "timeline limit must be between 1 and {PROJECTION_TIMELINE_MAX_LIMIT}: {limit}"
            ),
        ));
    }
    Ok(limit)
}

fn validate_client_route_sync_feed_limit(
    limit: Option<usize>,
) -> Result<usize, ProjectionAccessError> {
    let limit = limit.unwrap_or(PROJECTION_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT);
    if limit == 0 || limit > PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT {
        return Err(ProjectionAccessError::bad_request(
            "limit_invalid",
            format!(
                "client route sync feed limit must be between 1 and {PROJECTION_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT}: {limit}"
            ),
        ));
    }
    Ok(limit)
}

fn validate_list_limit(limit: Option<usize>) -> Result<usize, ProjectionAccessError> {
    let limit = limit.unwrap_or(PROJECTION_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > PROJECTION_LIST_MAX_LIMIT {
        return Err(ProjectionAccessError::bad_request(
            "limit_invalid",
            format!("list limit must be between 1 and {PROJECTION_LIST_MAX_LIMIT}: {limit}"),
        ));
    }
    Ok(limit)
}

#[derive(Debug)]
struct ProjectionListWindow<T> {
    items: Vec<T>,
    next_cursor: Option<String>,
    has_more: bool,
}

fn list_window<T>(
    items: Vec<T>,
    limit: usize,
    cursor: Option<&str>,
) -> Result<ProjectionListWindow<T>, ProjectionAccessError> {
    let offset = parse_list_cursor(cursor)?;
    if offset > items.len() {
        return Ok(ProjectionListWindow {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
        });
    }

    let mut window = items
        .into_iter()
        .skip(offset)
        .take(limit + 1)
        .collect::<Vec<_>>();
    let has_more = window.len() > limit;
    if has_more {
        window.truncate(limit);
    }
    let next_cursor = has_more.then(|| (offset + window.len()).to_string());

    Ok(ProjectionListWindow {
        items: window,
        next_cursor,
        has_more,
    })
}

fn parse_list_cursor(cursor: Option<&str>) -> Result<usize, ProjectionAccessError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(0);
    };
    cursor.parse::<usize>().map_err(|_| {
        ProjectionAccessError::bad_request(
            "cursor_invalid",
            format!("list cursor is invalid: {cursor}"),
        )
    })
}
