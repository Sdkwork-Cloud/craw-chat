//! Application-specific gateway bootstrap for sdkwork-im.
//! Mounts route crates through `gateway_mount` in standalone unified-process mode.

use std::sync::Arc;

use axum::Router;
use social_service::SocialRuntime;
use tokio::task::JoinHandle;

const SOCIAL_RUNTIME_DIR_ENV: &str = "SDKWORK_IM_RUNTIME_DIR";

pub struct ApplicationAssembly {
    pub router: Router,
    _background: ApplicationAssemblyBackground,
}

struct ApplicationAssemblyBackground {
    _social_shared_channel_sync: Option<JoinHandle<()>>,
}

pub async fn assemble_application_router() -> ApplicationAssembly {
    let mut router = Router::new();
    let mut background = ApplicationAssemblyBackground {
        _social_shared_channel_sync: None,
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
    router = router.merge(
        sdkwork_routes_im_social_backend_api::build_control_embedded_public_app(
            social_runtime.clone(),
        ),
    );
    router = router.merge(
        sdkwork_routes_im_social_open_api::build_runtime_public_app(social_runtime.clone()),
    );

    if let Some(social_state) =
        social_service::try_postgres_app_state_from_database_url_env().await
    {
        router = router.merge(sdkwork_routes_im_social_open_api::gateway_mount(social_state));
    }

    if let Some(space_state) = space_service::try_app_state_from_database_url_env().await {
        router = router.merge(sdkwork_routes_im_space_open_api::gateway_mount(space_state));
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
