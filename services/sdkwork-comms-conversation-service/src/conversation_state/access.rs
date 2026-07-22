use axum::http::StatusCode;
use im_adapters_postgres_journal::MemberSearchQuery;
use im_app_context::AppContext;
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursorView, MembershipRole,
    history_read_allowed,
};
use im_domain_core::social::DirectChatStatus;
use im_platform_contracts::normalize_realtime_organization_id;

use crate::conversation_state::inbox::InboxWindowQuery;
use crate::conversation_state::message_favorites::MessageFavoritesWindowQuery;
use crate::conversation_state::message_visibilities::TimelineWindowForPrincipalQuery;

use super::{
    ClientRouteSyncFeedWindowQuery, ClientRouteSyncFeedWindowView, ContactView, ContactWindowView,
    ConversationMemberDirectoryEntry, ConversationPreferencesView, ConversationProfileView,
    ConversationSummaryView, DeleteMessageFavoriteResponse, FavoriteMessageRequest,
    FavoriteMessagesWindowView, MessageFavoriteView, MessageInteractionSummaryView,
    MessageSearchHitView, MessageSearchWindowView, MessageVisibilityMutationResult,
    NotificationRecipientView, CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT,
    CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT, CONVERSATION_STATE_LIST_DEFAULT_LIMIT,
    CONVERSATION_STATE_LIST_MAX_LIMIT, CONVERSATION_STATE_TIMELINE_DEFAULT_LIMIT, CONVERSATION_STATE_TIMELINE_MAX_LIMIT,
    RealtimeFanoutTarget, RegisteredClientRouteView, ConversationStateService, TimelineWindowView,
    UpdateConversationPreferencesRequest, UpdateConversationProfileRequest,
};

const CONVERSATION_STATE_MAX_DEVICE_ID_BYTES: usize = 256;
const CONVERSATION_STATE_MAX_CONVERSATION_ID_BYTES: usize = 256;
const CONVERSATION_STATE_MAX_MESSAGE_ID_BYTES: usize = 256;
const CONVERSATION_STATE_MAX_SEARCH_QUERY_BYTES: usize = 512;
const CONVERSATION_STATE_SEARCH_DEFAULT_LIMIT: usize = 20;
const CONVERSATION_STATE_SEARCH_MAX_LIMIT: usize = 200;

#[derive(Debug)]
pub struct ConversationStateAccessError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientRouteSyncStateView {
    pub registered_client_routes: Vec<String>,
    pub latest_sync_seq: Option<u64>,
}

impl ConversationStateAccessError {
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

    fn store_unavailable(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "conversation_state_store_unavailable",
            message: message.into(),
        }
    }
}

impl From<super::event_apply::ConversationStateError> for ConversationStateAccessError {
    fn from(value: super::event_apply::ConversationStateError) -> Self {
        match value {
            super::event_apply::ConversationStateError::InvalidEvent(message) => {
                Self::bad_request("invalid_conversation_state_cursor", message)
            }
            super::event_apply::ConversationStateError::InvalidPayload(error) => {
                Self::bad_request("invalid_conversation_state_payload", error.to_string())
            }
            super::event_apply::ConversationStateError::InvalidState(error) => {
                Self::bad_request("invalid_conversation_state", error.to_string())
            }
            super::event_apply::ConversationStateError::StoreFailure(error) => {
                Self::store_unavailable(format!("{error:?}"))
            }
        }
    }
}

impl ConversationStateService {
    fn auth_organization_id(auth: &AppContext) -> String {
        normalize_realtime_organization_id(auth.organization_id.as_str())
    }

    fn direct_chat_binding_for_conversation(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Option<super::model::ContactDirectChatBindingView> {
        super::lock_conversation_state_mutex(
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
    ) -> Result<(), ConversationStateAccessError> {
        let Some(binding) =
            self.direct_chat_binding_for_conversation(tenant_id, organization_id, conversation_id)
        else {
            return Ok(());
        };
        if binding.status != DirectChatStatus::Archived {
            return Ok(());
        }

        Err(ConversationStateAccessError::forbidden(
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
    ) -> Result<(), ConversationStateAccessError> {
        validate_conversation_id(conversation_id)?;
        self.ensure_conversation_not_archived_direct_chat(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
        )?;
        let organization_id = Self::auth_organization_id(auth);
        let is_active = self
            .member_view_for_principal_kind(
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

        Err(ConversationStateAccessError::forbidden(
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
    ) -> Result<ConversationProfileView, ConversationStateAccessError> {
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
    ) -> Result<ConversationProfileView, ConversationStateAccessError> {
        self.ensure_conversation_profile_mutation_allowed(auth, conversation_id)?;
        self.update_conversation_profile_checked(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            update,
        )
        .map_err(ConversationStateAccessError::from)
    }

    fn ensure_conversation_profile_mutation_allowed(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<(), ConversationStateAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        let organization_id = Self::auth_organization_id(auth);
        let scope = super::scope::scope_key(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
        );
        let conversation_type =
            super::lock_conversation_state_mutex(&self.conversations, "conversation catalog")
                .get(scope.as_str())
                .map(|entry| entry.conversation_type.clone())
                .unwrap_or_else(|| "unknown".into());
        let Some(member) = self.member_view_for_principal_kind(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        ) else {
            return Err(ConversationStateAccessError::forbidden(
                "conversation_permission_denied",
                format!(
                    "principal is not active conversation member: {}",
                    auth.actor_id
                ),
            ));
        };
        if matches!(conversation_type.as_str(), "group" | "thread" | "room")
            && !matches!(member.role, MembershipRole::Owner | MembershipRole::Admin)
        {
            return Err(ConversationStateAccessError::forbidden(
                "conversation_profile_mutation_denied",
                "only owner or admin can update group conversation profile",
            ));
        }
        Ok(())
    }

    pub fn conversation_preferences_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<ConversationPreferencesView, ConversationStateAccessError> {
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
    ) -> Result<ConversationPreferencesView, ConversationStateAccessError> {
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
    ) -> Result<(), ConversationStateAccessError> {
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
        let member = self.member_view_for_principal_kind(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        );
        if history_read_allowed(history_visibility.as_str(), member.as_ref()) {
            return Ok(());
        }

        Err(ConversationStateAccessError::forbidden(
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
    ) -> Result<Vec<NotificationRecipientView>, ConversationStateAccessError> {
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
    ) -> Result<Vec<NotificationRecipientView>, ConversationStateAccessError> {
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
        let mut recipients = super::lock_conversation_state_mutex(&self.members, "member store")
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
    ) -> Result<RegisteredClientRouteView, ConversationStateAccessError> {
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
    ) -> Result<String, ConversationStateAccessError> {
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

    pub fn client_route_sync_state_from_auth_context(
        &self,
        auth: &AppContext,
        requested_device_id: Option<&str>,
    ) -> Result<ClientRouteSyncStateView, ConversationStateAccessError> {
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

        Ok(ClientRouteSyncStateView {
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
    ) -> Result<u64, ConversationStateAccessError> {
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
    ) -> Result<ClientRouteSyncFeedWindowView, ConversationStateAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_client_route_owned_by_auth_kind(self, auth, device_id)?;
        let limit = validate_client_route_sync_feed_limit(limit)?;
        let organization_id = normalize_realtime_organization_id(auth.organization_id.as_str());
        Ok(
            self.client_route_sync_feed_window_for_principal_kind(ClientRouteSyncFeedWindowQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                principal_id: auth.actor_id.as_str(),
                principal_kind: auth.actor_kind.as_str(),
                device_id,
                after_seq,
                limit,
            }),
        )
    }

    pub fn ack_client_route_sync_feed_from_auth_context(
        &self,
        auth: &AppContext,
        device_id: &str,
        acked_through_sync_seq: u64,
    ) -> Result<super::ClientRouteSyncAckStateView, ConversationStateAccessError> {
        validate_device_scope(auth, device_id)?;
        ensure_client_route_owned_by_auth_kind(self, auth, device_id)?;
        Ok(self.ack_client_route_sync_feed_for_principal_kind(
            auth.tenant_id.as_str(),
            normalize_realtime_organization_id(auth.organization_id.as_str()).as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
            acked_through_sync_seq,
        ))
    }

    pub fn inbox_from_auth_context(
        &self,
        auth: &AppContext,
    ) -> Result<Vec<ConversationInboxEntry>, ConversationStateAccessError> {
        Ok(self
            .inbox_for_principal_kind(
                auth.tenant_id.as_str(),
                Self::auth_organization_id(auth).as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
            )
            .map_err(ConversationStateAccessError::from)?
            .into_iter()
            .filter(|entry| {
                !self.is_archived_direct_chat_conversation(
                    auth.tenant_id.as_str(),
                    Self::auth_organization_id(auth).as_str(),
                    entry.conversation_id.as_str(),
                )
            })
            .collect())
    }

    pub fn inbox_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<super::InboxWindowView, ConversationStateAccessError> {
        self.inbox_window_from_auth_context_filtered(auth, limit, cursor, None, None)
    }

    pub fn inbox_window_from_auth_context_filtered(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
        conversation_type: Option<&str>,
        search_query: Option<&str>,
    ) -> Result<super::InboxWindowView, ConversationStateAccessError> {
        let limit = validate_list_limit(limit)?;
        let list_cursor = parse_inbox_list_cursor(cursor)?;
        let organization_id = Self::auth_organization_id(auth);
        let requested_conversation_type = conversation_type
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase);
        let requested_search_query = search_query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_lowercase);
        let search_query_char_count = requested_search_query
            .as_ref()
            .map_or(0, |query| query.chars().count());
        if search_query_char_count > 256 {
            return Err(ConversationStateAccessError::payload_too_large(
                "q",
                256,
                search_query_char_count,
            ));
        }
        self.inbox_window_for_principal_kind_filtered(
            InboxWindowQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                principal_id: auth.actor_id.as_str(),
                principal_kind: auth.actor_kind.as_str(),
                limit,
                cursor: list_cursor,
            },
            |entry| {
                !self.is_archived_direct_chat_conversation(
                    auth.tenant_id.as_str(),
                    organization_id.as_str(),
                    entry.conversation_id.as_str(),
                ) && requested_conversation_type
                    .as_ref()
                    .is_none_or(|conversation_type| {
                        entry
                            .conversation_type
                            .eq_ignore_ascii_case(conversation_type.as_str())
                    })
                    && requested_search_query.as_ref().is_none_or(|query| {
                        entry.conversation_id.to_lowercase().contains(query)
                            || entry.display_name.as_ref().is_some_and(|display_name| {
                                display_name.to_lowercase().contains(query)
                            })
                            || entry.peer.as_ref().is_some_and(|peer| {
                                peer.display_name.as_ref().is_some_and(|display_name| {
                                    display_name.to_lowercase().contains(query)
                                })
                            })
                    })
            },
        )
        .map_err(ConversationStateAccessError::from)
    }

    pub fn contacts_from_auth_context(
        &self,
        auth: &AppContext,
    ) -> Result<Vec<ContactView>, ConversationStateAccessError> {
        ensure_user_contact_owner(auth)?;
        Ok(self.contacts(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_state_social_principal(auth)?,
        ))
    }

    pub fn contact_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
        search_query: Option<&str>,
    ) -> Result<ContactWindowView, ConversationStateAccessError> {
        ensure_user_contact_owner(auth)?;
        let limit = validate_list_limit(limit)?;
        let list_cursor = parse_contact_list_cursor(cursor)?;
        let requested_search_query = search_query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_lowercase);
        let search_query_char_count = requested_search_query
            .as_ref()
            .map_or(0, |query| query.chars().count());
        if search_query_char_count > 256 {
            return Err(ConversationStateAccessError::payload_too_large(
                "q",
                256,
                search_query_char_count,
            ));
        }
        Ok(self.contact_window(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_state_social_principal(auth)?,
            limit,
            list_cursor,
            requested_search_query.as_deref(),
        ))
    }

    pub fn timeline_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        after_seq: Option<u64>,
        limit: Option<usize>,
    ) -> Result<TimelineWindowView, ConversationStateAccessError> {
        self.ensure_history_reader_from_auth_context(auth, conversation_id)?;
        let limit = validate_timeline_limit(limit)?;
        let organization_id = Self::auth_organization_id(auth);
        let mut window = self
            .timeline_window_for_principal(TimelineWindowForPrincipalQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                conversation_id,
                principal_kind: auth.actor_kind.as_str(),
                principal_id: auth.actor_id.as_str(),
                after_seq,
                limit,
            })
            .map_err(|error| ConversationStateAccessError::store_unavailable(error.to_string()))?;

        // Batch-enrich timeline entries with inline interaction data (reactions, pin)
        // to eliminate N+1 client-side interaction_summary requests.
        self.enrich_timeline_entries_with_interactions(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            conversation_id,
            &mut window.items,
        );

        Ok(window)
    }

    pub fn conversation_summary_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationSummaryView>, ConversationStateAccessError> {
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
    ) -> Result<Option<MessageInteractionSummaryView>, ConversationStateAccessError> {
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

    pub fn pinned_messages_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<
        sdkwork_utils_rust::SdkWorkPageData<MessageInteractionSummaryView>,
        ConversationStateAccessError,
    > {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        let limit = validate_list_limit(limit)?;
        let list_cursor = parse_pinned_messages_list_cursor(cursor)?;
        let window = self.pinned_messages_window(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            limit,
            list_cursor,
        );
        Ok(sdkwork_utils_rust::SdkWorkPageData {
            items: window
                .items
                .into_iter()
                .map(|view| {
                    let message_id = view.message_id.clone();
                    self.enrich_interaction_summary_with_read_receipt(
                        view,
                        auth.tenant_id.as_str(),
                        Self::auth_organization_id(auth).as_str(),
                        conversation_id,
                        message_id.as_str(),
                    )
                })
                .collect(),
            page_info: window.page_info,
        })
    }

    pub fn pinned_messages_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<MessageInteractionSummaryView>, ConversationStateAccessError> {
        let limit = validate_list_limit(None)?;
        Ok(self
            .pinned_messages_window_from_auth_context(auth, conversation_id, Some(limit), None)?
            .items)
    }

    pub fn read_cursor_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Option<ConversationReadCursorView>, ConversationStateAccessError> {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        Ok(self.read_cursor_for_principal_kind_and_device(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            auth.device_id.as_deref(),
        ))
    }

    pub fn member_directory_window_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<
        sdkwork_utils_rust::SdkWorkPageData<ConversationMemberDirectoryEntry>,
        ConversationStateAccessError,
    > {
        self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        let limit = validate_list_limit(limit)?;
        let list_cursor = parse_member_directory_list_cursor(cursor)?;
        self.member_directory_window(
            auth.tenant_id.as_str(),
            Self::auth_organization_id(auth).as_str(),
            conversation_id,
            limit,
            list_cursor,
        )
        .map_err(ConversationStateAccessError::from)
    }

    pub fn member_directory_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMemberDirectoryEntry>, ConversationStateAccessError> {
        let limit = validate_list_limit(None)?;
        Ok(self
            .member_directory_window_from_auth_context(auth, conversation_id, Some(limit), None)?
            .items)
    }

    pub fn message_favorites_window_from_auth_context(
        &self,
        auth: &AppContext,
        limit: Option<usize>,
        cursor: Option<&str>,
        favorite_type: Option<&str>,
        query: Option<&str>,
    ) -> Result<FavoriteMessagesWindowView, ConversationStateAccessError> {
        let limit = validate_list_limit(limit)?;
        let list_cursor = parse_favorite_messages_list_cursor(cursor)?;
        let organization_id = Self::auth_organization_id(auth);
        self.message_favorites_window_for_principal(MessageFavoritesWindowQuery {
            tenant_id: auth.tenant_id.as_str(),
            organization_id: organization_id.as_str(),
            principal_kind: auth.actor_kind.as_str(),
            principal_id: auth.actor_id.as_str(),
            limit,
            cursor: list_cursor,
            favorite_type,
            search_query: query,
        })
        .map_err(ConversationStateAccessError::from)
    }

    pub fn create_message_favorite_from_auth_context(
        &self,
        auth: &AppContext,
        message_id: &str,
        request: FavoriteMessageRequest,
    ) -> Result<MessageFavoriteView, ConversationStateAccessError> {
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
    ) -> Result<DeleteMessageFavoriteResponse, ConversationStateAccessError> {
        if favorite_id.trim().is_empty() {
            return Err(ConversationStateAccessError::bad_request(
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
            return Err(ConversationStateAccessError {
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

    /// Soft-delete (hide) a message from the current principal's view.
    ///
    /// Implements `DELETE /im/v3/api/chat/messages/{messageId}/visibility`
    /// (operationId `messages.visibility.delete`). The mutation is idempotent
    /// and scoped to the calling principal. Membership is required; unprojected
    /// messages return `404 message_not_found`.
    pub fn delete_message_visibility_from_auth_context(
        &self,
        auth: &AppContext,
        message_id: &str,
    ) -> Result<MessageVisibilityMutationResult, ConversationStateAccessError> {
        validate_message_id(message_id)?;
        // Locate the conversation that owns the message so we can enforce
        // membership. When the message is not currently projected we cannot
        // enforce membership and surface 404 instead of silently accepting.
        let organization_id = Self::auth_organization_id(auth);
        let conversation_id = self
            .conversation_id_for_message(
                auth.tenant_id.as_str(),
                organization_id.as_str(),
                message_id,
            )
            .ok_or_else(|| ConversationStateAccessError {
                status: StatusCode::NOT_FOUND,
                code: "message_not_found",
                message: format!("message not found: {message_id}"),
            })?;
        self.ensure_active_member_from_auth_context(auth, conversation_id.as_str())?;
        Ok(self.delete_message_visibility(
            auth.tenant_id.as_str(),
            organization_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
            message_id,
            Some(conversation_id.as_str()),
        ))
    }

    pub fn search_messages_from_auth_context(
        &self,
        auth: &AppContext,
        query: &str,
        conversation_id: Option<&str>,
        limit: Option<usize>,
        cursor: Option<&str>,
    ) -> Result<MessageSearchWindowView, ConversationStateAccessError> {
        let query = query.trim();
        if query.is_empty() {
            return Err(ConversationStateAccessError::bad_request(
                "search_query_required",
                "search query q is required",
            ));
        }
        if query.len() > CONVERSATION_STATE_MAX_SEARCH_QUERY_BYTES {
            return Err(ConversationStateAccessError::payload_too_large(
                "q",
                CONVERSATION_STATE_MAX_SEARCH_QUERY_BYTES,
                query.len(),
            ));
        }
        if let Some(conversation_id) = conversation_id {
            validate_conversation_id(conversation_id)?;
            self.ensure_active_member_from_auth_context(auth, conversation_id)?;
        }
        let limit = validate_search_limit(limit)?;
        let search_provider = crate::conversation_state::bootstrap::shared_conversation_state_runtime()
            .search_provider()
            .ok_or_else(|| ConversationStateAccessError {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "message_search_unconfigured",
                message: "message search provider is not configured".into(),
            })?;
        let organization_id = Self::auth_organization_id(auth);
        let principal_kind = auth.actor_kind.as_str();
        let principal_id = auth.social_principal_user_id();
        let result = search_provider
            .search_for_member(MemberSearchQuery {
                tenant_id: auth.tenant_id.as_str(),
                organization_id: organization_id.as_str(),
                principal_kind,
                principal_id,
                query,
                conversation_id,
                limit,
                cursor,
            })
            .map_err(|error| ConversationStateAccessError {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "message_search_failed",
                message: format!("message search failed: {error:?}"),
            })?;
        let hits = result
            .hits
            .into_iter()
            .filter(|hit| {
                let message_id = hit.message_id.to_string();
                !self
                    .message_visibility_for_principal(
                        auth.tenant_id.as_str(),
                        organization_id.as_str(),
                        principal_kind,
                        principal_id,
                        message_id.as_str(),
                    )
                    .is_some_and(|visibility| visibility.is_deleted)
            })
            .collect::<Vec<_>>();
        let has_more = result.next_cursor.is_some();
        let items = hits
            .into_iter()
            .map(|hit| MessageSearchHitView {
                conversation_id: hit.conversation_id,
                message_id: hit.message_id.to_string(),
                message_seq: hit.message_seq,
            })
            .collect();
        Ok(crate::conversation_state::list_page::cursor_page_with_total(
            items,
            limit,
            result.next_cursor,
            has_more,
            result.total_count,
        ))
    }
}

fn resolve_requested_device_id(
    auth: &AppContext,
    requested_device_id: Option<String>,
) -> Result<String, ConversationStateAccessError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(ConversationStateAccessError::bad_request(
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
        (None, None) => Err(ConversationStateAccessError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AppContext, device_id: &str) -> Result<(), ConversationStateAccessError> {
    validate_device_id(device_id)?;
    if let Some(bound_device_id) = auth.device_id.as_deref() {
        validate_device_id(bound_device_id)?;
        if bound_device_id != device_id {
            return Err(ConversationStateAccessError::forbidden(
                "device_scope_forbidden",
                format!("device scope forbidden: {device_id}"),
            ));
        }
    }
    Ok(())
}

fn ensure_client_route_registration_available(
    service: &ConversationStateService,
    auth: &AppContext,
    device_id: &str,
) -> Result<(), ConversationStateAccessError> {
    let has_conflict = super::lock_conversation_state_mutex(
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
        return Err(ConversationStateAccessError::conflict(
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
    service: &ConversationStateService,
    auth: &AppContext,
    device_id: &str,
) -> Result<(), ConversationStateAccessError> {
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

    Err(ConversationStateAccessError::forbidden(
        "device_scope_forbidden",
        format!("device scope forbidden: {device_id}"),
    ))
}

fn ensure_user_contact_owner(auth: &AppContext) -> Result<&str, ConversationStateAccessError> {
    auth.ensure_user_actor_principal().map_err(|error| {
        ConversationStateAccessError::forbidden("contact_scope_forbidden", error.message())
    })
}

fn conversation_state_social_principal(auth: &AppContext) -> Result<&str, ConversationStateAccessError> {
    ensure_user_contact_owner(auth)
}

fn validate_device_id(device_id: &str) -> Result<(), ConversationStateAccessError> {
    let actual_bytes = device_id.len();
    if actual_bytes > CONVERSATION_STATE_MAX_DEVICE_ID_BYTES {
        return Err(ConversationStateAccessError::payload_too_large(
            "deviceId",
            CONVERSATION_STATE_MAX_DEVICE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_conversation_id(conversation_id: &str) -> Result<(), ConversationStateAccessError> {
    let actual_bytes = conversation_id.len();
    if actual_bytes > CONVERSATION_STATE_MAX_CONVERSATION_ID_BYTES {
        return Err(ConversationStateAccessError::payload_too_large(
            "conversationId",
            CONVERSATION_STATE_MAX_CONVERSATION_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_message_id(message_id: &str) -> Result<(), ConversationStateAccessError> {
    if message_id.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "invalid_message_id",
            "messageId is required",
        ));
    }
    let actual_bytes = message_id.len();
    if actual_bytes > CONVERSATION_STATE_MAX_MESSAGE_ID_BYTES {
        return Err(ConversationStateAccessError::payload_too_large(
            "messageId",
            CONVERSATION_STATE_MAX_MESSAGE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}

fn validate_search_limit(limit: Option<usize>) -> Result<usize, ConversationStateAccessError> {
    let limit = limit.unwrap_or(CONVERSATION_STATE_SEARCH_DEFAULT_LIMIT);
    if limit == 0 || limit > CONVERSATION_STATE_SEARCH_MAX_LIMIT {
        return Err(ConversationStateAccessError::bad_request(
            "limit_invalid",
            format!("search limit must be between 1 and {CONVERSATION_STATE_SEARCH_MAX_LIMIT}: {limit}"),
        ));
    }
    Ok(limit)
}

fn validate_timeline_limit(limit: Option<usize>) -> Result<usize, ConversationStateAccessError> {
    let limit = limit.unwrap_or(CONVERSATION_STATE_TIMELINE_DEFAULT_LIMIT);
    if limit == 0 || limit > CONVERSATION_STATE_TIMELINE_MAX_LIMIT {
        return Err(ConversationStateAccessError::bad_request(
            "limit_invalid",
            format!(
                "timeline limit must be between 1 and {CONVERSATION_STATE_TIMELINE_MAX_LIMIT}: {limit}"
            ),
        ));
    }
    Ok(limit)
}

fn validate_client_route_sync_feed_limit(
    limit: Option<usize>,
) -> Result<usize, ConversationStateAccessError> {
    let limit = limit.unwrap_or(CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_DEFAULT_LIMIT);
    if limit == 0 || limit > CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT {
        return Err(ConversationStateAccessError::bad_request(
            "limit_invalid",
            format!(
                "client route sync feed limit must be between 1 and {CONVERSATION_STATE_CLIENT_ROUTE_SYNC_FEED_MAX_LIMIT}: {limit}"
            ),
        ));
    }
    Ok(limit)
}

fn validate_list_limit(limit: Option<usize>) -> Result<usize, ConversationStateAccessError> {
    let limit = limit.unwrap_or(CONVERSATION_STATE_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > CONVERSATION_STATE_LIST_MAX_LIMIT {
        return Err(ConversationStateAccessError::bad_request(
            "limit_invalid",
            format!("list limit must be between 1 and {CONVERSATION_STATE_LIST_MAX_LIMIT}: {limit}"),
        ));
    }
    Ok(limit)
}

fn parse_contact_list_cursor(
    cursor: Option<&str>,
) -> Result<super::model::ContactListCursor, ConversationStateAccessError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(super::model::ContactListCursor::Start);
    };
    if cursor.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "numeric offset contact cursors are unsupported; use the opaque keyset cursor from data.pageInfo.nextCursor",
        ));
    }
    let wire: super::model::ContactKeysetCursorWire = if cursor.contains('.') {
        let payload = crate::conversation_state::cursor_auth::decode_signed_conversation_state_cursor(cursor)
            .map_err(|error| ConversationStateAccessError::bad_request("cursor_invalid", error))?;
        serde_json::from_value(payload).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("contact cursor is invalid: {cursor}"),
            )
        })?
    } else {
        serde_json::from_str(cursor).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("contact cursor is invalid: {cursor}"),
            )
        })?
    };
    if wire.last_interaction_at.trim().is_empty() || wire.target_user_id.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "contact cursor must include lastInteractionAt and targetUserId",
        ));
    }
    Ok(super::model::ContactListCursor::Keyset {
        last_interaction_at: wire.last_interaction_at,
        target_user_id: wire.target_user_id,
    })
}

fn parse_inbox_list_cursor(
    cursor: Option<&str>,
) -> Result<super::model::InboxListCursor, ConversationStateAccessError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(super::model::InboxListCursor::Start);
    };
    if cursor.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "numeric offset inbox cursors are unsupported; use the opaque keyset cursor from data.pageInfo.nextCursor",
        ));
    }
    let wire: super::model::InboxKeysetCursorWire = if cursor.contains('.') {
        let payload = crate::conversation_state::cursor_auth::decode_signed_conversation_state_cursor(cursor)
            .map_err(|error| ConversationStateAccessError::bad_request("cursor_invalid", error))?;
        serde_json::from_value(payload).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("inbox cursor is invalid: {cursor}"),
            )
        })?
    } else {
        serde_json::from_str(cursor).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("inbox cursor is invalid: {cursor}"),
            )
        })?
    };
    if wire.activity_at.trim().is_empty() || wire.scope.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "inbox cursor must include activityAt and scope",
        ));
    }
    Ok(super::model::InboxListCursor::Keyset {
        activity_at: wire.activity_at,
        scope: wire.scope,
    })
}

fn parse_member_directory_list_cursor(
    cursor: Option<&str>,
) -> Result<super::model::MemberDirectoryListCursor, ConversationStateAccessError> {
    parse_conversation_state_keyset_cursor(
        cursor,
        "member directory",
        decode_member_directory_keyset_cursor,
    )
}

fn decode_member_directory_keyset_cursor(
    wire: super::model::MemberDirectoryKeysetCursorWire,
) -> Result<super::model::MemberDirectoryListCursor, ConversationStateAccessError> {
    if wire.joined_at.trim().is_empty() || wire.principal_id.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "member directory cursor must include joinedAt and principalId",
        ));
    }
    Ok(super::model::MemberDirectoryListCursor::Keyset {
        role_rank: wire.role_rank,
        joined_at: wire.joined_at,
        principal_id: wire.principal_id,
    })
}

fn parse_pinned_messages_list_cursor(
    cursor: Option<&str>,
) -> Result<super::model::PinnedMessagesListCursor, ConversationStateAccessError> {
    parse_conversation_state_keyset_cursor(
        cursor,
        "pinned messages",
        decode_pinned_messages_keyset_cursor,
    )
}

fn decode_pinned_messages_keyset_cursor(
    wire: super::model::PinnedMessagesKeysetCursorWire,
) -> Result<super::model::PinnedMessagesListCursor, ConversationStateAccessError> {
    if wire.pinned_at.trim().is_empty() || wire.message_id.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "pinned messages cursor must include pinnedAt and messageId",
        ));
    }
    Ok(super::model::PinnedMessagesListCursor::Keyset {
        pinned_at: wire.pinned_at,
        message_seq: wire.message_seq,
        message_id: wire.message_id,
    })
}

fn parse_favorite_messages_list_cursor(
    cursor: Option<&str>,
) -> Result<super::model::FavoriteMessagesListCursor, ConversationStateAccessError> {
    parse_conversation_state_keyset_cursor(
        cursor,
        "favorite messages",
        decode_favorite_messages_keyset_cursor,
    )
}

fn decode_favorite_messages_keyset_cursor(
    wire: super::model::FavoriteMessagesKeysetCursorWire,
) -> Result<super::model::FavoriteMessagesListCursor, ConversationStateAccessError> {
    if wire.favorited_at.trim().is_empty() || wire.favorite_id.trim().is_empty() {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            "favorite messages cursor must include favoritedAt and favoriteId",
        ));
    }
    Ok(super::model::FavoriteMessagesListCursor::Keyset {
        favorited_at: wire.favorited_at,
        favorite_id: wire.favorite_id,
    })
}

fn parse_conversation_state_keyset_cursor<Wire, Cursor, F>(
    cursor: Option<&str>,
    label: &str,
    decode_keyset: F,
) -> Result<Cursor, ConversationStateAccessError>
where
    Wire: serde::de::DeserializeOwned,
    Cursor: FromStartCursor,
    F: FnOnce(Wire) -> Result<Cursor, ConversationStateAccessError>,
{
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(Cursor::start());
    };
    if cursor.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(ConversationStateAccessError::bad_request(
            "cursor_invalid",
            format!(
                "numeric offset {label} cursors are unsupported; use the opaque keyset cursor from data.pageInfo.nextCursor"
            ),
        ));
    }
    let wire: Wire = if cursor.contains('.') {
        let payload = crate::conversation_state::cursor_auth::decode_signed_conversation_state_cursor(cursor)
            .map_err(|error| ConversationStateAccessError::bad_request("cursor_invalid", error))?;
        serde_json::from_value(payload).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("{label} cursor is invalid: {cursor}"),
            )
        })?
    } else {
        serde_json::from_str(cursor).map_err(|_| {
            ConversationStateAccessError::bad_request(
                "cursor_invalid",
                format!("{label} cursor is invalid: {cursor}"),
            )
        })?
    };
    decode_keyset(wire)
}

trait FromStartCursor {
    fn start() -> Self;
}

impl FromStartCursor for super::model::MemberDirectoryListCursor {
    fn start() -> Self {
        Self::Start
    }
}

impl FromStartCursor for super::model::PinnedMessagesListCursor {
    fn start() -> Self {
        Self::Start
    }
}

impl FromStartCursor for super::model::FavoriteMessagesListCursor {
    fn start() -> Self {
        Self::Start
    }
}

#[cfg(test)]
mod keyset_cursor_tests {
    use super::*;

    #[test]
    fn numeric_conversation_state_list_cursors_are_rejected_in_every_environment() {
        assert!(parse_contact_list_cursor(Some("1")).is_err());
        assert!(parse_inbox_list_cursor(Some("1")).is_err());
        assert!(parse_member_directory_list_cursor(Some("1")).is_err());
        assert!(parse_pinned_messages_list_cursor(Some("1")).is_err());
        assert!(parse_favorite_messages_list_cursor(Some("1")).is_err());
    }
}
