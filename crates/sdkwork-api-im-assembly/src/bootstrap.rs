//! Application-specific gateway bootstrap for sdkwork-im.
//! Mounts route crates through `gateway_mount` in standalone single-ingress mode.

use std::sync::Arc;

use axum::Router;
use conversation_runtime::resolve_embedded_conversation_runtime;
use im_adapters_social_postgres::SocialPostgresConfig;
use im_app_context::allows_header_only_app_context_fallback;
use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use session_gateway::RealtimePlaneBootstrap;
use social_service::SocialRuntime;
use tokio::task::JoinHandle;

use crate::space_conversation_wiring::wire_space_conversation_binders;

const SOCIAL_RUNTIME_DIR_ENV: &str = "SDKWORK_IM_RUNTIME_DIR";

pub struct ApiAssembly {
    pub router: Router,
    pub social_runtime: Arc<SocialRuntime>,
    _background: ApiAssemblyBackground,
}

struct ApiAssemblyBackground {
    _social_shared_channel_sync: Option<JoinHandle<()>>,
    _social_friend_request_expiration: Option<JoinHandle<()>>,
    /// Keep postgres-backed handler state alive when router merge replaces route handlers.
    _social_postgres_state: Option<social_service::PostgresAppState>,
    _space_state: Option<space_service::http::AppState>,
    _projection_journal_consumer: Option<projection_service::ProjectionJournalConsumerHandle>,
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    assemble_api_router_with_realtime_bootstrap(None).await
}

pub async fn assemble_api_router_with_realtime_bootstrap(
    realtime_bootstrap: Option<&RealtimePlaneBootstrap>,
) -> Result<ApiAssembly, String> {
    sdkwork_im_database_pool::try_bootstrap_im_process_database_pools_from_env().await?;

    let mut router = Router::new();
    let mut background = ApiAssemblyBackground {
        _social_shared_channel_sync: None,
        _social_friend_request_expiration: None,
        _social_postgres_state: None,
        _space_state: None,
        _projection_journal_consumer: None,
    };

    let conversation_state =
        conversation_runtime::http::bootstrap_conversation_app_state_from_env()?;
    conversation_state
        .ensure_group_knowledgebase_outbox_relay_started()
        .await
        .map_err(|error| format!("group knowledgebase outbox relay readiness failed: {error}"))?;

    let social_runtime = build_social_runtime()?;
    background._social_shared_channel_sync =
        social_service::spawn_shared_channel_sync_stale_reclaim_scheduler_from_env(
            social_runtime.clone(),
        );
    background._social_friend_request_expiration =
        social_service::spawn_friend_request_expiration_scheduler_from_env(social_runtime.clone());

    router = router.merge(sdkwork_routes_im_audit_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_automation_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_calls_open_api::gateway_mount());
    router = router.merge(
        sdkwork_routes_im_chat_open_api::gateway_mount_with_state(conversation_state.clone())
            .await?,
    );
    router = router.merge(
        sdkwork_routes_im_knowledgebase_app_api::gateway_mount_with_state(conversation_state)
            .await?,
    );
    router = router.merge(sdkwork_routes_im_governance_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_media_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_notification_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_ops_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_portal_app_api::gateway_mount());
    router = router.merge(sdkwork_routes_im_projection_open_api::build_supplemental_public_app());
    router = router.merge(match realtime_bootstrap {
        Some(bootstrap) => {
            sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap_from_env(
                bootstrap,
            )
            .await
        }
        None => sdkwork_routes_im_realtime_open_api::gateway_mount(),
    });
    background._projection_journal_consumer =
        projection_service::spawn_projection_journal_consumer_from_env(
            projection_service::default_projection_runtime(),
        );
    router = router.merge(
        sdkwork_routes_im_social_backend_api::build_control_embedded_public_app(
            social_runtime.clone(),
        ),
    );
    router = router.merge(sdkwork_routes_im_social_open_api::build_runtime_public_app(
        social_runtime.clone(),
    ));

    if let Some(pool) = resolve_embedded_social_postgres_pool().await {
        let social_state = social_service::app_state_from_postgres_pool(pool.clone()).await;
        router = router.merge(sdkwork_routes_im_social_open_api::gateway_mount(
            social_state.clone(),
        ));
        background._social_postgres_state = Some(social_state);

        let mut space_state = space_service::app_state_from_postgres_pool(pool).await;
        if let Some(conversation_runtime) = resolve_embedded_conversation_runtime() {
            space_state = wire_space_conversation_binders(space_state, conversation_runtime);
        }
        router = router.merge(sdkwork_routes_im_space_open_api::gateway_mount(
            space_state.clone(),
        ));
        background._space_state = Some(space_state);
    }

    router = router.merge(sdkwork_routes_im_stream_app_api::gateway_mount());

    Ok(ApiAssembly {
        router,
        social_runtime,
        _background: background,
    })
}

fn build_social_runtime() -> Result<Arc<SocialRuntime>, String> {
    match social_service::build_social_runtime_from_env() {
        Ok(runtime) => Ok(runtime),
        Err(error) if allows_header_only_app_context_fallback() => {
            tracing::warn!(
                error = %error,
                "social runtime env bootstrap failed; falling back to file or in-memory runtime (development/test only)"
            );
            match std::env::var(SOCIAL_RUNTIME_DIR_ENV) {
                Ok(runtime_dir) if !runtime_dir.trim().is_empty() => Ok(Arc::new(
                    SocialRuntime::from_runtime_dir(runtime_dir.as_str()),
                )),
                _ => Ok(Arc::new(SocialRuntime::default())),
            }
        }
        Err(error) => Err(format!(
            "social runtime env bootstrap failed in production-like environment: {error}"
        )),
    }
}

async fn resolve_embedded_social_postgres_pool()
-> Option<im_adapters_social_postgres::SocialPostgresPool> {
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
