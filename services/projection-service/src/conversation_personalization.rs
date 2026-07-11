use im_time::utc_now_rfc3339_millis;

use super::model::{
    ConversationPreferencesView, ConversationProfileView, UpdateConversationPreferencesRequest,
    UpdateConversationProfileRequest,
};
use super::{TimelineProjectionService, lock_projection_mutex, scope::scope_key, snapshot};

pub(super) fn conversation_preferences_key(
    tenant_id: &str,
    organization_id: &str,
    conversation_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    format!(
        "{}:{}:{}",
        scope_key(tenant_id, organization_id, conversation_id),
        principal_kind,
        principal_id
    )
}

fn default_conversation_profile(tenant_id: &str, conversation_id: &str) -> ConversationProfileView {
    ConversationProfileView {
        tenant_id: tenant_id.into(),
        conversation_id: conversation_id.into(),
        display_name: String::new(),
        avatar_url: String::new(),
        notice: String::new(),
        updated_at: utc_now_rfc3339_millis(),
        updated_by_principal_kind: None,
        updated_by_principal_id: None,
    }
}

fn default_conversation_preferences(
    tenant_id: &str,
    conversation_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> ConversationPreferencesView {
    ConversationPreferencesView {
        tenant_id: tenant_id.into(),
        conversation_id: conversation_id.into(),
        principal_kind: principal_kind.into(),
        principal_id: principal_id.into(),
        is_pinned: false,
        is_muted: false,
        is_marked_unread: false,
        is_hidden: false,
        updated_at: utc_now_rfc3339_millis(),
    }
}

impl TimelineProjectionService {
    pub fn conversation_profile(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> ConversationProfileView {
        let key = scope_key(tenant_id, organization_id, conversation_id);
        let profile = if let Some(profile) =
            lock_projection_mutex(&self.conversation_profiles, "conversation profile store")
                .get(key.as_str())
                .cloned()
        {
            profile
        } else {
            self.load_conversation_profile_from_durable_store(key.as_str())
                .unwrap_or_else(|| default_conversation_profile(tenant_id, conversation_id))
        };

        self.apply_catalog_title_profile_fallback(key.as_str(), profile)
    }

    fn apply_catalog_title_profile_fallback(
        &self,
        scope: &str,
        mut profile: ConversationProfileView,
    ) -> ConversationProfileView {
        if !profile.display_name.trim().is_empty() {
            return profile;
        }
        let Some((display_name, created_at)) = self.catalog_title_for_profile_fallback(scope)
        else {
            return profile;
        };

        profile.display_name = display_name;
        if profile.updated_at.trim().is_empty() {
            profile.updated_at = created_at;
        }
        lock_projection_mutex(&self.conversation_profiles, "conversation profile store")
            .insert(scope.to_owned(), profile.clone());
        profile
    }

    fn catalog_title_for_profile_fallback(&self, scope: &str) -> Option<(String, String)> {
        let catalog = lock_projection_mutex(&self.conversations, "conversation store")
            .get(scope)
            .cloned()
            .or_else(|| self.load_conversation_catalog_from_durable_store(scope))?;
        let title = catalog
            .title
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())?
            .to_owned();
        Some((title, catalog.created_at))
    }

    /// Read-through fallback: load conversation profile from the durable
    /// metadata store and hydrate the in-memory profile cache. Returns
    /// `None` when the durable store is not configured, the snapshot is
    /// absent, or the load fails (warn-logged).
    fn load_conversation_profile_from_durable_store(
        &self,
        scope: &str,
    ) -> Option<ConversationProfileView> {
        let store = self.durable_metadata_store()?;
        match snapshot::load_metadata_snapshot::<ConversationProfileView>(
            store.as_ref(),
            scope,
            snapshot::CONVERSATION_PROFILE_KEY,
        ) {
            Ok(Some(profile)) => {
                lock_projection_mutex(&self.conversation_profiles, "conversation profile store")
                    .insert(scope.to_owned(), profile.clone());
                Some(profile)
            }
            Ok(None) => None,
            Err(error) => {
                tracing::warn!(
                    target: "sdkwork.im.projection.read_through",
                    event = "im.projection.conversation_profile_durable_read_failed",
                    scope = %scope,
                    error = %error,
                    "durable metadata read-through failed for conversation profile",
                );
                None
            }
        }
    }

    pub fn update_conversation_profile(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        update: UpdateConversationProfileRequest,
    ) -> ConversationProfileView {
        let key = scope_key(tenant_id, organization_id, conversation_id);
        let mut profiles =
            lock_projection_mutex(&self.conversation_profiles, "conversation profile store");
        let mut profile = profiles
            .get(key.as_str())
            .cloned()
            .unwrap_or_else(|| default_conversation_profile(tenant_id, conversation_id));

        if let Some(display_name) = update.display_name {
            profile.display_name = display_name;
        }
        if let Some(avatar_url) = update.avatar_url {
            profile.avatar_url = avatar_url;
        }
        if let Some(notice) = update.notice {
            profile.notice = notice;
        }
        profile.updated_at = utc_now_rfc3339_millis();
        profile.updated_by_principal_kind = Some(principal_kind.into());
        profile.updated_by_principal_id = Some(principal_id.into());

        profiles.insert(key, profile.clone());
        profile
    }

    pub fn conversation_preferences(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
    ) -> ConversationPreferencesView {
        let key = conversation_preferences_key(
            tenant_id,
            organization_id,
            conversation_id,
            principal_kind,
            principal_id,
        );
        lock_projection_mutex(
            &self.conversation_preferences,
            "conversation preferences store",
        )
        .get(key.as_str())
        .cloned()
        .unwrap_or_else(|| {
            default_conversation_preferences(
                tenant_id,
                conversation_id,
                principal_kind,
                principal_id,
            )
        })
    }

    pub fn update_conversation_preferences(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        principal_kind: &str,
        principal_id: &str,
        update: UpdateConversationPreferencesRequest,
    ) -> ConversationPreferencesView {
        let key = conversation_preferences_key(
            tenant_id,
            organization_id,
            conversation_id,
            principal_kind,
            principal_id,
        );
        let mut preferences = lock_projection_mutex(
            &self.conversation_preferences,
            "conversation preferences store",
        );
        let mut view = preferences.get(key.as_str()).cloned().unwrap_or_else(|| {
            default_conversation_preferences(
                tenant_id,
                conversation_id,
                principal_kind,
                principal_id,
            )
        });

        if let Some(is_pinned) = update.is_pinned {
            view.is_pinned = is_pinned;
        }
        if let Some(is_muted) = update.is_muted {
            view.is_muted = is_muted;
        }
        if let Some(is_marked_unread) = update.is_marked_unread {
            view.is_marked_unread = is_marked_unread;
        }
        if let Some(is_hidden) = update.is_hidden {
            view.is_hidden = is_hidden;
        }
        view.updated_at = utc_now_rfc3339_millis();

        preferences.insert(key, view.clone());
        view
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_profile_and_preferences_round_trip_in_memory_store() {
        let service = TimelineProjectionService::default();
        let profile = service.update_conversation_profile(
            "100001",
            "default",
            "c_agent_e7f6182d320811b42f4484f9",
            "user",
            "1",
            UpdateConversationProfileRequest {
                display_name: Some("SdkWork Assistant".into()),
                avatar_url: Some("https://example.test/assistant.png".into()),
                notice: None,
            },
        );
        assert_eq!(profile.display_name, "SdkWork Assistant");
        assert_eq!(profile.avatar_url, "https://example.test/assistant.png");

        let loaded =
            service.conversation_profile("100001", "default", "c_agent_e7f6182d320811b42f4484f9");
        assert_eq!(loaded.display_name, "SdkWork Assistant");

        let preferences = service.update_conversation_preferences(
            "100001",
            "default",
            "c_agent_e7f6182d320811b42f4484f9",
            "user",
            "1",
            UpdateConversationPreferencesRequest {
                is_pinned: Some(true),
                is_muted: None,
                is_marked_unread: None,
                is_hidden: Some(false),
            },
        );
        assert!(preferences.is_pinned);
        assert!(!preferences.is_hidden);
    }
}
