//! Application-specific gateway bootstrap for sdkwork-im.
//! Mounts route crates through `gateway_mount` in standalone unified-process mode.

use std::sync::Arc;

use axum::Router;
use im_adapters_social_postgres::SocialPostgresConfig;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use social_service::SocialRuntime;
use tokio::task::JoinHandle;

const SOCIAL_RUNTIME_DIR_ENV: &str = "SDKWORK_IM_RUNTIME_DIR";

pub struct ApplicationAssembly {
    pub router: Router,
    _background: ApplicationAssemblyBackground,
}

struct ApplicationAssemblyBackground {
    _social_shared_channel_sync: Option<JoinHandle<()>>,
    /// Keep postgres-backed handler state alive when router merge replaces route handlers.
    _social_postgres_state: Option<social_service::PostgresAppState>,
    _space_state: Option<space_service::http::AppState>,
    _projection_journal_consumer:
        Option<projection_service::ProjectionJournalConsumerHandle>,
}

pub async fn assemble_application_router() -> ApplicationAssembly {
    let mut router = Router::new();
    let mut background = ApplicationAssemblyBackground {
        _social_shared_channel_sync: None,
        _social_postgres_state: None,
        _space_state: None,
        _projection_journal_consumer: None,
    };

    let social_runtime = build_social_runtime();
    background._social_shared_channel_sync =
        social_service::spawn_shared_channel_sync_stale_reclaim_scheduler_from_env(
            social_runtime.clone(),
        );

    router = router.merge(sdkwork_routes_im_audit_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_automation_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_calls_open_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_chat_open_api::gateway_mount().await);
    router = router.merge(sdkwork_routes_im_governance_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_media_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_notification_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_ops_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_projection_open_api::build_supplemental_public_app());
    background._projection_journal_consumer =
        projection_service::spawn_projection_journal_consumer_from_env(
            projection_service::default_projection_runtime(),
        );
    router = router.merge(
        sdkwork_routes_im_social_backend_api::build_control_embedded_public_app(
            social_runtime.clone(),
        ),
    );
    router = router.merge(
        sdkwork_routes_im_social_open_api::build_runtime_public_app(social_runtime.clone()),
    );

    if let Some(pool) = resolve_embedded_social_postgres_pool().await {
        let social_state = social_service::app_state_from_postgres_pool(pool.clone()).await;
        router = router.merge(
            sdkwork_routes_im_social_open_api::gateway_mount(social_state.clone()),
        );
        background._social_postgres_state = Some(social_state);

        let space_state = space_service::app_state_from_postgres_pool(pool).await;
        router = router.merge(sdkwork_routes_im_space_open_api::gateway_mount(space_state.clone()));
        background._space_state = Some(space_state);
    }

    router = router.merge(sdkwork_routes_im_stream_app_api::gateway_mount());

    ApplicationAssembly {
        router,
        _background: background,
    }
}

fn build_social_runtime() -> Arc<SocialRuntime> {
    match std::env::var(SOCIAL_RUNTIME_DIR_ENV) {
        Ok(runtime_dir) if !runtime_dir.trim().is_empty() => Arc::new(SocialRuntime::from_runtime_dir(
            runtime_dir.as_str(),
        )),
        _ => Arc::new(SocialRuntime::default()),
    }
}

async fn resolve_embedded_social_postgres_pool(
) -> Option<im_adapters_social_postgres::SocialPostgresPool> {
    if let Ok(pool) = sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool() {
        return Some(im_adapters_social_postgres::SocialPostgresPool::new(pool));
    }

    let config = DatabaseConfig::from_env("IM").ok()?;
    if config.engine != DatabaseEngine::Postgres {
        return None;
    }

    SocialPostgresConfig::from_database_config(&config)
        .connect_pool()
        .ok()
}
