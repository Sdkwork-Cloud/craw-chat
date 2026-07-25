//! Bootstrap helpers for space-service process startup.

use std::sync::Arc;

use axum::Router;
use im_adapters_postgres_journal::{PostgresCommitJournal, PostgresJournalPool};
use im_adapters_social_postgres::governance_store::{
    PostgresBanStore, PostgresChannelAccessRuleStore, PostgresInvitationStore,
    PostgresSpaceMemberStore,
};
use im_adapters_social_postgres::organization_store::{
    PostgresChannelStore, PostgresGroupMemberStore, PostgresGroupStore, PostgresSpaceStore,
};
use im_adapters_social_postgres::SocialPostgresPool;

use crate::http::{AppState, build_embedded_app, build_public_app};
use crate::id::build_runtime_id_generator_for_space;
use crate::write_authority::SpaceWriteAuthority;

/// Environment variable name for database connection URL.
/// Referenced in doc comments but loaded via configuration system.
#[allow(dead_code)]
pub const DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";

pub async fn app_state_from_postgres_pool(pool: SocialPostgresPool) -> AppState {
    let pool_arc = Arc::new(pool.inner().clone());
    let journal = PostgresCommitJournal::from_pool(
        PostgresJournalPool::from_pool(pool.inner().clone()),
    );
    let write_authority = Some(Arc::new(SpaceWriteAuthority::new(journal)));
    AppState {
        postgres_pool: Some(pool),
        space_store: Arc::new(PostgresSpaceStore::new(pool_arc.clone())),
        group_store: Arc::new(PostgresGroupStore::new(pool_arc.clone())),
        channel_store: Arc::new(PostgresChannelStore::new(pool_arc.clone())),
        group_member_store: Arc::new(PostgresGroupMemberStore::new(pool_arc.clone())),
        space_member_store: Arc::new(PostgresSpaceMemberStore::new(pool_arc.clone())),
        ban_store: Arc::new(PostgresBanStore::new(pool_arc.clone())),
        invitation_store: Arc::new(PostgresInvitationStore::new(pool_arc.clone())),
        channel_access_rule_store: Arc::new(PostgresChannelAccessRuleStore::new(pool_arc)),
        id_generator: build_runtime_id_generator_for_space().await,
        group_conversation_binder: None,
        channel_conversation_binder: None,
        write_authority,
    }
}

/// Resolves space [`AppState`] when the IM PostgreSQL database is configured.
pub async fn try_app_state_from_database_url_env() -> Option<AppState> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM").ok()?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return None;
    }
    let pool = sdkwork_im_database_pool::clone_shared_im_postgres_r2d2_pool()
        .map(SocialPostgresPool::new)?;
    Some(app_state_from_postgres_pool(pool).await)
}

/// Builds the embedded space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub async fn try_build_embedded_app_from_database_url_env() -> Option<Router> {
    try_app_state_from_database_url_env()
        .await
        .map(build_embedded_app)
}

/// Builds the public space router when `SDKWORK_IM_DATABASE_URL` is configured.
pub async fn try_build_public_app_from_database_url_env() -> Option<Router> {
    try_app_state_from_database_url_env()
        .await
        .map(build_public_app)
}
