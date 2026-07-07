use std::sync::Arc;

use im_adapters_redis_cache::TypingCache;
use im_app_context::AppContext;
use im_domain_core::typing::{
    TYPING_DEFAULT_TTL_SECONDS, TYPING_EVENT_TYPE, TYPING_SCOPE_TYPE, TypingIndicator,
    TypingIndicatorList, TypingIndicatorListItem,
};
use im_platform_contracts::{
    RealtimeEventPublisher, RealtimeEventRecipient, RealtimeScopeEventPublishCommand,
};
use im_time::utc_now_rfc3339_millis;

use super::{
    ConversationRuntime, RuntimeError, organization_id_from_auth_context,
};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalTypingResult {
    /// Number of connected devices that received the realtime push.
    pub delivered: usize,
}

impl<J> ConversationRuntime<J> {
    pub fn typing_cache(&self) -> Option<&Arc<dyn TypingCache>> {
        self.typing_cache.as_ref()
    }

    pub fn resolve_realtime_publisher(&self) -> Option<Arc<dyn RealtimeEventPublisher>> {
        self.realtime_publisher
            .clone()
            .or_else(crate::embedded_wiring::resolve_embedded_realtime_publisher)
    }

    pub async fn signal_typing_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<SignalTypingResult, RuntimeError> {
        let tenant_id = auth.tenant_id.as_str();
        let organization_id = organization_id_from_auth_context(auth).as_str();
        let actor_id = auth.actor_id.as_str();
        let actor_kind = auth.actor_kind.as_str();

        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;

        if let Some(cache) = self.typing_cache() {
            cache
                .set_typing(
                    tenant_id,
                    organization_id,
                    conversation_id,
                    actor_id,
                    TYPING_DEFAULT_TTL_SECONDS,
                )
                .await?;
        }

        let delivered = self
            .publish_typing_indicator_to_peers(
                tenant_id,
                organization_id,
                conversation_id,
                actor_id,
                actor_kind,
            )
            .unwrap_or(0);

        Ok(SignalTypingResult { delivered })
    }

    pub async fn list_typing_indicators_from_auth_context(
        &self,
        auth: &AppContext,
        conversation_id: &str,
    ) -> Result<TypingIndicatorList, RuntimeError> {
        let tenant_id = auth.tenant_id.as_str();
        let organization_id = organization_id_from_auth_context(auth).as_str();
        let actor_id = auth.actor_id.as_str();
        let actor_kind = auth.actor_kind.as_str();

        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id)?;

        let Some(cache) = self.typing_cache() else {
            return Ok(TypingIndicatorList {
                conversation_id: conversation_id.to_owned(),
                items: Vec::new(),
            });
        };

        let typing_users = cache
            .list_typing_users(tenant_id, organization_id, conversation_id)
            .await?;

        let members = self.list_members(tenant_id, organization_id, conversation_id)?;
        let member_kinds = members
            .iter()
            .map(|member| (member.principal_id.as_str(), member.principal_kind.as_str()))
            .collect::<std::collections::HashMap<_, _>>();

        let items = typing_users
            .into_iter()
            .filter(|user_id| user_id.as_str() != actor_id)
            .filter_map(|user_id| {
                member_kinds
                    .get(user_id.as_str())
                    .map(|principal_kind| TypingIndicatorListItem {
                        user_id,
                        user_kind: (*principal_kind).to_owned(),
                    })
            })
            .collect();

        let _ = actor_kind;
        Ok(TypingIndicatorList {
            conversation_id: conversation_id.to_owned(),
            items,
        })
    }

    fn publish_typing_indicator_to_peers(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        actor_id: &str,
        actor_kind: &str,
    ) -> Result<usize, RuntimeError> {
        let Some(publisher) = self.resolve_realtime_publisher() else {
            return Ok(0);
        };

        let members = self.list_members(tenant_id, organization_id, conversation_id)?;
        let recipients = members
            .iter()
            .filter(|member| {
                !(member.principal_id.as_str() == actor_id
                    && member.principal_kind.as_str() == actor_kind)
            })
            .map(|member| {
                RealtimeEventRecipient::new(
                    member.principal_id.clone(),
                    member.principal_kind.clone(),
                )
            })
            .collect::<Vec<_>>();

        if recipients.is_empty() {
            return Ok(0);
        }

        let indicator = TypingIndicator::new(
            conversation_id,
            actor_id,
            actor_kind,
            utc_now_rfc3339_millis(),
        );
        let payload = indicator
            .to_payload_json()
            .map_err(|error| RuntimeError::InvalidInput(format!("typing payload encode failed: {error}")))?;

        publisher
            .publish_ephemeral_scope_event_to_recipients(
                RealtimeScopeEventPublishCommand {
                    tenant_id,
                    organization_id,
                    scope_type: TYPING_SCOPE_TYPE,
                    scope_id: conversation_id,
                    event_type: TYPING_EVENT_TYPE,
                    payload,
                    recipients,
                },
            )
            .map_err(RuntimeError::from)
    }
}
