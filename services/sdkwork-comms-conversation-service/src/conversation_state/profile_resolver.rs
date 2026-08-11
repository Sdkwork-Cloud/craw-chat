//! Read-time display enrichment for conversation inbox peers and member
//! directory entries, sourced from the IM user profile table.
//!
//! Conversation members are event-sourced without display metadata; display
//! names for user principals come from `im_user_profiles.im_nickname` and are
//! resolved lazily at read time so inbox and member-directory responses always
//! carry names without mutating the in-memory member store.

use std::sync::Arc;

use im_adapters_social_postgres::user_profile_store::{PostgresUserProfileStore, UserProfileStore};

/// Display attributes resolved for a user principal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedUserDisplay {
    pub display_name: String,
    pub avatar_url: Option<String>,
}

/// Blocking resolver for user display attributes (display name / avatar).
pub trait UserProfileResolver: Send + Sync {
    fn resolve_display(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Option<ResolvedUserDisplay>;
}

/// PostgreSQL-backed resolver reading `im_user_profiles`.
#[derive(Clone)]
pub struct PostgresUserProfileResolver {
    store: Arc<PostgresUserProfileStore>,
}

impl PostgresUserProfileResolver {
    pub fn new(store: Arc<PostgresUserProfileStore>) -> Self {
        Self { store }
    }
}

impl UserProfileResolver for PostgresUserProfileResolver {
    fn resolve_display(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Option<ResolvedUserDisplay> {
        let record = self
            .store
            .get_by_user_id(tenant_id, organization_id, user_id)
            .ok()
            .flatten()?;
        let display_name = record
            .im_nickname
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())?
            .to_owned();
        Some(ResolvedUserDisplay {
            display_name,
            avatar_url: record
                .im_avatar_url
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned),
        })
    }
}
