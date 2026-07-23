use im_domain_events::CommitEnvelope;
use serde::{Deserialize, Serialize};

use crate::conversation_state::event_apply::ConversationStateError;
use crate::conversation_state::model::ConversationCatalogEntry;
use crate::conversation_state::scope::{
    conversation_state_organization_id_for_event, group_scope_key, scope_key,
};
use crate::conversation_state::{ConversationStateService, lock_conversation_state_mutex};
const CANONICAL_GROUP_CONVERSATION_ID_PREFIX: &str = "g_";
const CANONICAL_GROUP_CONVERSATION_ID_DIGEST_LEN: usize = 24;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GroupConversationBinding {
    pub(super) tenant_id: String,
    pub(super) organization_id: String,
    pub(super) group_id: String,
    pub(super) conversation_id: String,
    pub(super) updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupCreatedConversationStatePayload {
    group_id: String,
    group_name: String,
    #[serde(default)]
    conversation_id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    avatar_url: Option<String>,
    #[serde(default)]
    announcement: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupUpdatedConversationStatePayload {
    group_id: String,
    group_name: String,
    #[serde(default)]
    conversation_id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    avatar_url: Option<String>,
    #[serde(default)]
    announcement: Option<String>,
    updated_at: String,
}

impl ConversationStateService {
    pub(crate) fn apply_group_created(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: GroupCreatedConversationStatePayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_group_aggregate_id(event, payload.group_id.as_str())?;
        let Some(conversation_id) = trimmed_owned(payload.conversation_id.as_deref())
            .or_else(|| infer_group_conversation_id_from_group_id(payload.group_id.as_str()))
        else {
            tracing::warn!(
                target: "sdkwork.im.conversation_state.group_metadata",
                event = "im.conversation_state.group_created_without_conversation_binding",
                tenant_id = %event.tenant_id,
                group_id = %payload.group_id,
                "group.created cannot update conversation profile without an explicit or canonical group conversation binding",
            );
            return Ok(());
        };
        let organization_id = conversation_state_organization_id_for_event(event);
        let updated_at =
            first_non_empty([payload.updated_at.as_str(), payload.created_at.as_str()])
                .unwrap_or(event.committed_at.as_str());

        self.upsert_group_conversation_binding(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.group_id.as_str(),
            conversation_id.as_str(),
            updated_at,
        );
        self.apply_group_profile_metadata(
            event,
            organization_id.as_str(),
            conversation_id.as_str(),
            GroupProfileMetadata {
                group_name: payload.group_name.as_str(),
                avatar_url: payload.avatar_url.as_deref(),
                announcement: payload.announcement.as_deref(),
                description: payload.description.as_deref(),
                updated_at,
            },
            false,
        );
        Ok(())
    }

    pub(crate) fn apply_group_updated(
        &self,
        event: &CommitEnvelope,
    ) -> Result<(), ConversationStateError> {
        let payload: GroupUpdatedConversationStatePayload =
            serde_json::from_str(&event.payload).map_err(ConversationStateError::InvalidPayload)?;
        validate_group_aggregate_id(event, payload.group_id.as_str())?;
        let organization_id = conversation_state_organization_id_for_event(event);
        let conversation_id = trimmed_owned(payload.conversation_id.as_deref())
            .or_else(|| {
                self.group_conversation_id(
                    event.tenant_id.as_str(),
                    organization_id.as_str(),
                    payload.group_id.as_str(),
                )
            })
            .or_else(|| infer_group_conversation_id_from_group_id(payload.group_id.as_str()));
        let Some(conversation_id) = conversation_id else {
            tracing::warn!(
                target: "sdkwork.im.conversation_state.group_metadata",
                event = "im.conversation_state.group_update_without_conversation_binding",
                tenant_id = %event.tenant_id,
                organization_id = %organization_id,
                group_id = %payload.group_id,
                "group.updated cannot update conversation profile without a group conversation binding",
            );
            return Ok(());
        };
        let updated_at =
            non_empty(payload.updated_at.as_str()).unwrap_or(event.committed_at.as_str());

        self.upsert_group_conversation_binding(
            event.tenant_id.as_str(),
            organization_id.as_str(),
            payload.group_id.as_str(),
            conversation_id.as_str(),
            updated_at,
        );
        self.apply_group_profile_metadata(
            event,
            organization_id.as_str(),
            conversation_id.as_str(),
            GroupProfileMetadata {
                group_name: payload.group_name.as_str(),
                avatar_url: payload.avatar_url.as_deref(),
                announcement: payload.announcement.as_deref(),
                description: payload.description.as_deref(),
                updated_at,
            },
            true,
        );
        Ok(())
    }

    fn upsert_group_conversation_binding(
        &self,
        tenant_id: &str,
        organization_id: &str,
        group_id: &str,
        conversation_id: &str,
        updated_at: &str,
    ) {
        let Some(group_id) = non_empty(group_id) else {
            return;
        };
        let Some(conversation_id) = non_empty(conversation_id) else {
            return;
        };
        let key = group_scope_key(tenant_id, organization_id, group_id);
        let binding = GroupConversationBinding {
            tenant_id: tenant_id.to_owned(),
            organization_id: key.organization_id.clone(),
            group_id: group_id.to_owned(),
            conversation_id: conversation_id.to_owned(),
            updated_at: updated_at.to_owned(),
        };
        lock_conversation_state_mutex(
            &self.group_conversation_bindings,
            "group conversation binding store",
        )
        .insert(key, binding);
    }

    pub(crate) fn group_conversation_id(
        &self,
        tenant_id: &str,
        organization_id: &str,
        group_id: &str,
    ) -> Option<String> {
        let key = group_scope_key(tenant_id, organization_id, group_id);
        lock_conversation_state_mutex(
            &self.group_conversation_bindings,
            "group conversation binding store",
        )
        .get(&key)
        .map(|binding| binding.conversation_id.clone())
    }

    fn apply_group_profile_metadata(
        &self,
        event: &CommitEnvelope,
        organization_id: &str,
        conversation_id: &str,
        metadata: GroupProfileMetadata<'_>,
        overwrite_existing: bool,
    ) {
        let Some(display_name) = non_empty(metadata.group_name) else {
            return;
        };
        let scope = scope_key(event.tenant_id.as_str(), organization_id, conversation_id);
        {
            let mut conversations =
                lock_conversation_state_mutex(&self.conversations, "conversation store");
            let entry =
                conversations
                    .entry(scope.clone())
                    .or_insert_with(|| ConversationCatalogEntry {
                        conversation_type: "group".into(),
                        created_at: metadata.updated_at.to_owned(),
                        history_visibility: "joined".into(),
                        title: None,
                        agent_assignments: None,
                    });
            if entry.conversation_type.trim().is_empty() || entry.conversation_type == "unknown" {
                entry.conversation_type = "group".into();
            }
            if overwrite_existing || entry.title.as_deref().and_then(non_empty).is_none() {
                entry.title = Some(display_name.to_owned());
            }
        }

        let mut profiles = lock_conversation_state_mutex(
            &self.conversation_profiles,
            "conversation profile store",
        );
        let profile = profiles.entry(scope).or_insert_with(|| {
            crate::conversation_state::ConversationProfileView {
                tenant_id: event.tenant_id.clone(),
                conversation_id: conversation_id.to_owned(),
                display_name: String::new(),
                avatar_url: String::new(),
                notice: String::new(),
                updated_at: metadata.updated_at.to_owned(),
                updated_by_principal_kind: Some(event.actor.actor_kind.clone()),
                updated_by_principal_id: Some(event.actor.actor_id.clone()),
            }
        });
        if overwrite_existing || profile.display_name.trim().is_empty() {
            profile.display_name = display_name.to_owned();
        }
        if let Some(avatar_url) = metadata.avatar_url.and_then(non_empty)
            && (overwrite_existing || profile.avatar_url.trim().is_empty())
        {
            profile.avatar_url = avatar_url.to_owned();
        }
        if let Some(notice) = metadata
            .announcement
            .and_then(non_empty)
            .or_else(|| metadata.description.and_then(non_empty))
            && (overwrite_existing || profile.notice.trim().is_empty())
        {
            profile.notice = notice.to_owned();
        }
        profile.updated_at = metadata.updated_at.to_owned();
        profile.updated_by_principal_kind = Some(event.actor.actor_kind.clone());
        profile.updated_by_principal_id = Some(event.actor.actor_id.clone());
    }
}

struct GroupProfileMetadata<'a> {
    group_name: &'a str,
    avatar_url: Option<&'a str>,
    announcement: Option<&'a str>,
    description: Option<&'a str>,
    updated_at: &'a str,
}

fn trimmed_owned(value: Option<&str>) -> Option<String> {
    value.and_then(non_empty).map(str::to_owned)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn first_non_empty<'a>(values: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    values.into_iter().find_map(non_empty)
}

fn validate_group_aggregate_id(
    event: &CommitEnvelope,
    payload_group_id: &str,
) -> Result<(), ConversationStateError> {
    let aggregate_id = event.aggregate_id.trim();
    let normalized_payload_group_id = payload_group_id.trim();
    if aggregate_id.is_empty()
        || normalized_payload_group_id.is_empty()
        || event.aggregate_id != aggregate_id
        || payload_group_id != normalized_payload_group_id
        || aggregate_id != normalized_payload_group_id
    {
        return Err(ConversationStateError::InvalidEvent(format!(
            "{} groupId {} does not match aggregate {}",
            event.event_type, normalized_payload_group_id, aggregate_id
        )));
    }
    Ok(())
}

fn infer_group_conversation_id_from_group_id(group_id: &str) -> Option<String> {
    let group_id = non_empty(group_id)?;
    if let Some(suffix) = group_id.strip_prefix(CANONICAL_GROUP_CONVERSATION_ID_PREFIX) {
        if is_canonical_group_conversation_suffix(suffix) {
            return Some(group_id.to_owned());
        }
        return None;
    }
    if is_canonical_group_conversation_suffix(group_id) {
        return Some(format!(
            "{CANONICAL_GROUP_CONVERSATION_ID_PREFIX}{group_id}"
        ));
    }
    None
}

fn is_canonical_group_conversation_suffix(value: &str) -> bool {
    value.len() == CANONICAL_GROUP_CONVERSATION_ID_DIGEST_LEN
        && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
