//! Application-specific gateway bootstrap for sdkwork-im.
//! Mounts route crates through `gateway_mount` in standalone single-ingress mode.

use std::sync::Arc;

use audit_service::AuditRuntime;
use axum::Router;
use conversation_runtime::resolve_embedded_conversation_runtime;
use ops_service::OpsRuntime;
use portal_service::PortalRuntime;
use sdkwork_web_bootstrap::{
    ApiAssemblyContribution, CompositeReadinessCheck, ReadinessCheck, ReadinessFuture,
};
use sdkwork_web_core::HttpRouteManifest;
use session_gateway::RealtimePlaneBootstrap;
use social_service::SocialRuntime;
use tokio::task::JoinHandle;

use crate::ops_realtime_wiring::{OpsRealtimeMirrorHandle, spawn_ops_realtime_mirror};
use crate::space_conversation_wiring::wire_space_conversation_binders;

pub struct ApiAssembly {
    pub contribution: ApiAssemblyContribution,
    pub runtime: ApiAssemblyRuntime,
}

pub struct ApiAssemblyRuntime {
    pub social_runtime: Arc<SocialRuntime>,
    pub ops_runtime: Arc<OpsRuntime>,
    _background: ApiAssemblyBackground,
}

struct ApiAssemblyBackground {
    _retention_scheduler: Option<im_adapters_postgres_journal::RetentionPurgeSchedulerHandle>,
    _social_shared_channel_sync: Option<JoinHandle<()>>,
    _social_friend_request_expiration: Option<JoinHandle<()>>,
    /// Keep postgres-backed handler state alive when router merge replaces route handlers.
    _social_postgres_state: Option<social_service::PostgresAppState>,
    _space_state: Option<space_service::http::AppState>,
    _ops_realtime_mirror: Option<OpsRealtimeMirrorHandle>,
}

#[derive(Clone)]
struct RealtimePlaneReadinessCheck {
    readiness: session_gateway::ServiceReadiness,
}

impl RealtimePlaneReadinessCheck {
    fn new(readiness: session_gateway::ServiceReadiness) -> Self {
        Self { readiness }
    }
}

impl ReadinessCheck for RealtimePlaneReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let readiness = self.readiness.clone();
        Box::pin(async move {
            if readiness.is_ready().await {
                Ok(())
            } else {
                Err("embedded realtime plane is unavailable or draining".to_owned())
            }
        })
    }
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    assemble_api_router_with_realtime_bootstrap(None).await
}

pub async fn assemble_api_router_with_realtime_bootstrap(
    realtime_bootstrap: Option<&RealtimePlaneBootstrap>,
) -> Result<ApiAssembly, String> {
    sdkwork_im_database_pool::bootstrap_im_process_database_pools_from_env().await?;

    let mut router = Router::new();
    let mut background = ApiAssemblyBackground {
        _retention_scheduler:
            im_adapters_postgres_journal::spawn_retention_purge_scheduler_from_env(),
        _social_shared_channel_sync: None,
        _social_friend_request_expiration: None,
        _social_postgres_state: None,
        _space_state: None,
        _ops_realtime_mirror: None,
    };

    let conversation_state =
        conversation_runtime::http::bootstrap_conversation_app_state_from_env()?;
    conversation_state
        .ensure_group_knowledgebase_outbox_relay_started()
        .await
        .map_err(|error| format!("group knowledgebase outbox relay readiness failed: {error}"))?;

    let social_runtime = social_service::build_social_runtime_from_env()?;
    background._social_shared_channel_sync =
        social_service::spawn_shared_channel_sync_stale_reclaim_scheduler_from_env(
            social_runtime.clone(),
        );
    background._social_friend_request_expiration =
        social_service::spawn_friend_request_expiration_scheduler_from_env(social_runtime.clone());

    let audit_runtime = Arc::new(AuditRuntime::from_env());
    let automation_runtime = automation_service::build_runtime_from_env()?;
    let notification_runtime = notification_service::build_runtime_from_env()?;
    let ops_runtime = Arc::new(match realtime_bootstrap {
        Some(bootstrap) => OpsRuntime::from_env_with_node_id(bootstrap.node_id.as_str()),
        None => OpsRuntime::from_env(),
    });
    let portal_runtime = Arc::new(PortalRuntime::new(
        ops_runtime.clone(),
        audit_runtime.clone(),
    ));
    let realtime_cluster = realtime_bootstrap
        .map(|bootstrap| bootstrap.assembly.realtime_cluster())
        .unwrap_or_else(|| Arc::new(session_gateway::RealtimeClusterBridge::default()));
    background._ops_realtime_mirror = realtime_bootstrap
        .map(|bootstrap| spawn_ops_realtime_mirror(ops_runtime.clone(), bootstrap));

    router = router.merge(
        sdkwork_routes_im_audit_backend_api::gateway_mount_with_runtime(audit_runtime.clone()),
    );
    router = router.merge(
        sdkwork_routes_im_automation_app_api::gateway_mount_with_runtime(
            automation_runtime.clone(),
        ),
    );
    router = router.merge(sdkwork_routes_im_calls_open_api::gateway_mount());
    router = router.merge(
        sdkwork_routes_im_chat_open_api::gateway_mount_with_state(conversation_state.clone())
            .await?,
    );
    router = router.merge(
        sdkwork_routes_im_knowledgebase_app_api::gateway_mount_with_state(conversation_state)
            .await?,
    );
    router = router.merge(
        sdkwork_routes_im_governance_backend_api::gateway_mount_with_automation_runtime_and_governance_sinks(
            automation_runtime,
            realtime_cluster,
            ops_runtime.clone(),
            audit_runtime,
        ),
    );
    router = router.merge(sdkwork_routes_im_media_app_api::gateway_mount());
    router = router.merge(
        sdkwork_routes_im_notification_app_api::gateway_mount_with_runtime(notification_runtime),
    );
    router = router
        .merge(sdkwork_routes_im_ops_backend_api::gateway_mount_with_runtime(ops_runtime.clone()));
    router =
        router.merge(sdkwork_routes_im_portal_app_api::gateway_mount_with_runtime(portal_runtime));
    router = router.merge(match realtime_bootstrap {
        Some(bootstrap) => {
            sdkwork_routes_im_realtime_open_api::build_public_app_with_realtime_bootstrap_from_env(
                bootstrap,
            )
            .await
        }
        None => sdkwork_routes_im_realtime_open_api::gateway_mount(),
    });
    router = router.merge(
        sdkwork_routes_im_social_backend_api::build_control_embedded_public_app(
            social_runtime.clone(),
        ),
    );
    router = router.merge(sdkwork_routes_im_social_open_api::build_runtime_public_app(
        social_runtime.clone(),
    ));

    let pool = resolve_embedded_social_postgres_pool()?;
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

    router = router.merge(sdkwork_routes_im_stream_app_api::gateway_mount());

    let mut readiness_checks =
        vec![sdkwork_im_service_readiness::resolve_im_service_readiness_check().await];
    if let Some(bootstrap) = realtime_bootstrap {
        readiness_checks.push(Arc::new(RealtimePlaneReadinessCheck::new(
            bootstrap.assembly.readiness(),
        )));
    }
    let contribution = ApiAssemblyContribution::from_manifest(
        "sdkwork-im",
        "SDKWork IM API",
        router,
        build_route_manifest(),
        Vec::new(),
        Arc::new(CompositeReadinessCheck::new(readiness_checks)),
    )?;

    Ok(ApiAssembly {
        contribution,
        runtime: ApiAssemblyRuntime {
            social_runtime,
            ops_runtime,
            _background: background,
        },
    })
}

fn build_route_manifest() -> HttpRouteManifest {
    let manifests = [
        sdkwork_routes_im_audit_backend_api::gateway_route_manifest(),
        sdkwork_routes_im_automation_app_api::gateway_route_manifest(),
        sdkwork_routes_im_calls_open_api::gateway_route_manifest(),
        sdkwork_routes_im_chat_open_api::gateway_route_manifest(),
        sdkwork_routes_im_governance_backend_api::gateway_route_manifest(),
        sdkwork_routes_im_knowledgebase_app_api::gateway_route_manifest(),
        sdkwork_routes_im_media_app_api::gateway_route_manifest(),
        sdkwork_routes_im_notification_app_api::gateway_route_manifest(),
        sdkwork_routes_im_ops_backend_api::gateway_route_manifest(),
        sdkwork_routes_im_portal_app_api::gateway_route_manifest(),
        sdkwork_routes_im_realtime_open_api::gateway_route_manifest(),
        sdkwork_routes_im_social_backend_api::gateway_route_manifest(),
        sdkwork_routes_im_social_open_api::gateway_route_manifest(),
        sdkwork_routes_im_space_open_api::gateway_route_manifest(),
        sdkwork_routes_im_stream_app_api::gateway_route_manifest(),
    ];
    HttpRouteManifest::from_owned_routes(
        manifests
            .into_iter()
            .flat_map(|manifest| manifest.routes().to_vec())
            .collect(),
    )
}

fn resolve_embedded_social_postgres_pool()
-> Result<im_adapters_social_postgres::SocialPostgresPool, String> {
    sdkwork_im_database_pool::ensure_im_process_postgres_r2d2_pool()
        .map(im_adapters_social_postgres::SocialPostgresPool::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_manifest_builds_an_indivisible_contract() {
        let contribution = ApiAssemblyContribution::from_manifest(
            "sdkwork-im",
            "SDKWork IM API",
            Router::new(),
            build_route_manifest(),
            Vec::new(),
            Arc::new(sdkwork_web_bootstrap::AlwaysReady),
        )
        .expect("IM route manifest must produce aligned OpenAPI and permissions");

        assert!(!contribution.route_manifest.routes().is_empty());
        assert_eq!(
            contribution.permission_catalog,
            sdkwork_web_bootstrap::permission_catalog(contribution.route_manifest.routes()),
        );
    }
}
