//! Standalone single-ingress dependency App API contributions.
//!
//! Sibling domain route crates are mounted in-process per `APPLICATION_GATEWAY_SPEC.md`
//! platform consumer linking and `DEPENDENCY_MANAGEMENT_SPEC.md` §5 — not HTTP-proxied
//! to internal HTTP service ports when IM standalone gateway collapses platform ingress.

use std::path::PathBuf;
use std::sync::Arc;

use sdkwork_iam_embedded_application_bootstrap::ensure_tenant_application_from_app_root_with_env_and_fallback;

pub struct EmbeddedDependencyRoutes {
    pub contributions: Vec<sdkwork_web_bootstrap::ApiAssemblyContribution>,
    pub agents_session_facade: Arc<dyn sdkwork_agents_runtime_facade::AgentsSessionFacade>,
}

struct EmbeddedAgentsRuntime {
    contribution: sdkwork_web_bootstrap::ApiAssemblyContribution,
    session_facade: Arc<dyn sdkwork_agents_runtime_facade::AgentsSessionFacade>,
}

const EMBEDDED_DEPENDENCY_APP_ROOTS: &[(&str, &str)] = &[
    ("SDKWORK_DRIVE", "sdkwork-drive"),
    ("SDKWORK_KNOWLEDGEBASE", "sdkwork-knowledgebase"),
    ("SDKWORK_INVENTORY", "sdkwork-inventory"),
    ("SDKWORK_INVOICE", "sdkwork-invoice"),
    ("SDKWORK_MEMBERSHIP", "sdkwork-membership"),
    ("SDKWORK_MERCHANDISE", "sdkwork-merchandise"),
    ("SDKWORK_PROMOTION", "sdkwork-promotion"),
    ("SDKWORK_ORDER", "sdkwork-order"),
    ("SDKWORK_PAYMENT", "sdkwork-payment"),
    ("SDKWORK_SHOP", "sdkwork-shop"),
    ("SDKWORK_NOTARY", "sdkwork-notary"),
    ("SDKWORK_AGENTS", "sdkwork-agents"),
    ("SDKWORK_COURSE", "sdkwork-course"),
    ("SDKWORK_COMMUNITY", "sdkwork-community"),
    ("SDKWORK_FEEDS", "sdkwork-feeds"),
    ("SDKWORK_COMPANY", "sdkwork-company"),
    ("SDKWORK_CATALOG", "sdkwork-catalog"),
    ("SDKWORK_MAIL", "sdkwork-mail"),
];

/// Apply all embedded dependency environment variables synchronously.
///
/// This must be called from the main thread BEFORE the Tokio runtime is created
/// to avoid data races on the process environment. After this returns, all
/// embedded modules share the validated `SDKWORK_DATABASE_*` profile, while
/// module runtime settings and `SDKWORK_*_APP_ROOT` values are resolved before
/// async bootstrap functions read them.
///
/// # Safety
///
/// See `set_env_var` safety contract — callers must ensure no other threads exist.
pub fn apply_embedded_dependency_env() -> Result<(), String> {
    validate_workspace_server_database_env()?;
    apply_knowledgebase_runtime_env_from_im_shared_profile()?;
    apply_agents_runtime_env_from_im_shared_profile()?;
    apply_embedded_dependency_app_roots();
    apply_embedded_commerce_backend_env();
    apply_embedded_feeds_env();
    Ok(())
}

/// Point the feeds community source adapter and the anonymous community/feeds
/// read surfaces at this collapsed single-ingress with the seeded demo tenant
/// when the topology does not configure them explicitly.
fn apply_embedded_feeds_env() {
    let gateway_public_url = std::env::var("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL")
        .ok()
        .filter(|value| !value.trim().is_empty());
    if let Some(url) = gateway_public_url.as_deref() {
        // The feeds community adapter pulls circle content from the community
        // open surface served in-process by this gateway.
        if std::env::var("SDKWORK_FEEDS_COMMUNITY_OPEN_API_BASE_URL")
            .ok()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        {
            set_env_var("SDKWORK_FEEDS_COMMUNITY_OPEN_API_BASE_URL", url);
        }
    }
    // Anonymous public reads (community feed.list, feeds streams) resolve the
    // seeded demo tenant when no IAM context is present.
    if std::env::var("COMMUNITY_DEFAULT_TENANT_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("COMMUNITY_DEFAULT_TENANT_ID", "100001");
    }
    if std::env::var("SDKWORK_FEEDS_DEFAULT_TENANT_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_FEEDS_DEFAULT_TENANT_ID", "100001");
    }
}

/// Point the community commerce integration (tier publishing + order payment
/// verification) at this collapsed single-ingress when no external
/// membership/order backend is configured. The standalone gateway serves the
/// membership/order backend business surfaces in-process, so the community
/// service reaches them through the gateway's own public origin.
fn apply_embedded_commerce_backend_env() {
    let Some(gateway_public_url) = std::env::var("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        return;
    };
    for key in [
        "SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL",
        "SDKWORK_ORDER_BACKEND_API_BASE_URL",
    ] {
        if std::env::var(key)
            .ok()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
        {
            set_env_var(key, gateway_public_url.as_str());
        }
    }
}

fn validate_workspace_server_database_env() -> Result<(), String> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM")
        .map_err(|error| format!("resolve workspace database profile failed: {error}"))?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return Err("IM standalone gateway requires SDKWORK_DATABASE_ENGINE=postgresql".to_owned());
    }
    Ok(())
}

fn apply_embedded_dependency_app_roots() {
    for (prefix, repo_dir) in EMBEDDED_DEPENDENCY_APP_ROOTS {
        ensure_embedded_dependency_app_root(prefix, repo_dir);
    }
}

fn ensure_embedded_dependency_app_root(env_prefix: &str, repo_dir: &str) {
    let app_root_key = format!("{env_prefix}_APP_ROOT");
    if std::env::var(&app_root_key)
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var(
            app_root_key.as_str(),
            resolve_sibling_app_root(repo_dir)
                .to_string_lossy()
                .as_ref(),
        );
    }
}

/// Synchronize embedded dependency database schemas before mounting routes.
///
/// Every embedded dependency (drive, knowledgebase, mail, merchandise,
/// promotion, course, community) installs its baseline tables into the single
/// process-shared workspace PostgreSQL profile. Lifecycle syncs are idempotent
/// (`CREATE TABLE IF NOT EXISTS`) and run once at gateway startup so the
/// collapsed single-ingress assembly serves real data.
pub async fn bootstrap_embedded_dependency_databases() -> Result<(), String> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM")
        .map_err(|error| format!("resolve workspace database profile failed: {error}"))?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return Err("IM standalone gateway requires SDKWORK_DATABASE_ENGINE=postgresql".to_owned());
    }
    let drive_pool =
        sdkwork_drive_workspace_service::infrastructure::sql::installer::connect_any_database_and_install_schema(
            &config.url,
        )
        .await
        .map_err(|error| format!("sync embedded drive database schema failed: {error}"))?;
    drop(drive_pool);
    sdkwork_knowledgebase_database_host::bootstrap_knowledgebase_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded knowledgebase database failed: {error}"))?;
    sdkwork_mail_database_host::bootstrap_mail_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded mail database failed: {error}"))?;
    sdkwork_catalog_database_host::bootstrap_catalog_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded catalog database failed: {error}"))?;
    sdkwork_company_database_host::bootstrap_company_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded company database failed: {error}"))?;
    // Community baseline plus the official circle seed data (idempotent) so
    // circles are usable out of the box; explicit seed does not depend on the
    // global SDKWORK_DATABASE_SEED_ON_BOOT switch (which would also seed every
    // other embedded dependency).
    sdkwork_community_database_host::bootstrap_community_database_with_seed_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded community database failed: {error}"))?;
    // Feeds baseline (`feeds_stream`, `feeds_item`, ...): official streams are
    // ensured by the embedded feeds runtime bootstrap.
    sdkwork_feeds_database_host::bootstrap_feeds_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("sync embedded feeds database failed: {error}"))?;
    bootstrap_embedded_merchandise_database().await?;
    bootstrap_embedded_promotion_database().await?;
    Ok(())
}

pub async fn bootstrap_embedded_dependency_routes() -> Result<EmbeddedDependencyRoutes, String> {
    bootstrap_embedded_merchandise_database().await?;
    bootstrap_embedded_promotion_database().await?;
    // One community host serves both the App surface (circle read/write) and
    // the open surface (feed.public.list — the data source for the feeds
    // community adapter) from the same process database pool.
    let community_host = Arc::new(
        sdkwork_community_service_host::CommunityServiceHost::from_env()
            .await
            .map_err(|error| format!("compose embedded community host failed: {error}"))?,
    );
    let mut contributions = vec![
        bootstrap_embedded_account_contribution().await?,
        bootstrap_embedded_drive_contribution().await?,
        bootstrap_embedded_knowledgebase_contribution().await?,
        bootstrap_embedded_inventory_contribution().await?,
        bootstrap_embedded_invoice_contribution().await?,
        sdkwork_api_membership_assembly::assemble_app_api_contribution().await?,
        sdkwork_api_order_assembly::assemble_app_api_contribution().await?,
        bootstrap_embedded_payment_contribution().await?,
        bootstrap_embedded_shop_contribution().await?,
        bootstrap_embedded_notary_contribution().await?,
        bootstrap_embedded_course_routes().await?,
        sdkwork_api_community_assembly::assemble_app_api_contribution_with_host(
            community_host.clone(),
        )
        .map_err(|error| format!("compose embedded community App API failed: {error}"))?,
        sdkwork_api_community_assembly::assemble_open_api_contribution_with_host(
            community_host.clone(),
        )
        .map_err(|error| format!("compose embedded community Open API failed: {error}"))?,
        sdkwork_api_company_assembly::assemble_app_api_contribution()
            .await
            .map_err(|error| format!("compose embedded company App API failed: {error}"))?,
        bootstrap_embedded_catalog_contribution().await?,
        bootstrap_embedded_mail_contribution().await?,
    ];
    contributions.push(
        bootstrap_embedded_feeds_contribution(community_host.clone())
            .await
            .map_err(|error| format!("compose embedded feeds Open API failed: {error}"))?,
    );
    let agents_runtime = build_embedded_agents_runtime().await?;
    let agents_session_facade = agents_runtime.session_facade;
    contributions.push(agents_runtime.contribution);
    // Service-to-service backend surfaces: the embedded community service
    // registers membership packages and verifies paid orders through the
    // membership/order backend business APIs, both served by this ingress.
    contributions.push(bootstrap_embedded_membership_backend_contribution().await?);
    contributions.push(bootstrap_embedded_order_backend_contribution().await?);
    Ok(EmbeddedDependencyRoutes {
        contributions,
        agents_session_facade,
    })
}

/// Embed the feeds open surface (`/feeds/v3/api/*`) from the sibling
/// sdkwork-feeds workspace: streams and items are served by this gateway, the
/// community source adapter pulls circle content from the embedded community
/// open surface, and a background task keeps streams incrementally synced.
async fn bootstrap_embedded_feeds_contribution(
    community_host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    use sdkwork_feeds_service_host::FeedsServiceHost;

    let database = sdkwork_feeds_database_host::bootstrap_feeds_database_from_env()
        .await
        .map_err(|error| format!("bootstrap embedded feeds database failed: {error}"))?;
    let host = Arc::new(
        FeedsServiceHost::from_database_pool(database.pool().clone())
            .map_err(|error| format!("compose embedded feeds service host failed: {error}"))?,
    );

    if let Some(adapter) = sdkwork_feeds_source_community::CommunitySourceAdapter::from_env() {
        host.register_source_adapter(Box::new(adapter));
        tracing::info!("embedded feeds community source adapter registered (community.entry)");
    }
    if let Some(adapter) = sdkwork_feeds_source_news::NewsSourceAdapter::from_env() {
        host.register_source_adapter(Box::new(adapter));
        tracing::info!("embedded feeds news source adapter registered (news.item)");
    }

    // Ensure the standard streams exist (idempotent) so circle feeds and
    // moments never 404: one stream per circle (posts + resources) plus the
    // global moments stream.
    let tenant_id = std::env::var("SDKWORK_FEEDS_DEFAULT_TENANT_ID")
        .unwrap_or_else(|_| "100001".to_owned());
    ensure_feeds_streams(&host, &community_host, &tenant_id).await;

    // Background incremental sync (fallback to adapter-driven sync; 60s tick
    // mirrors the standalone feeds worker).
    spawn_feeds_sync_task(host.clone());

    let router = sdkwork_routes_feeds_open_api::build_open_router(
        sdkwork_routes_feeds_open_api::FeedsOpenHost {
            service: host.service(),
        },
    );
    sdkwork_web_bootstrap::ApiAssemblyContribution::from_manifest(
        "sdkwork-feeds",
        "SDKWork Feeds Open API",
        router,
        sdkwork_routes_feeds_open_api::gateway_route_manifest(),
        Vec::new(),
        Arc::new(sdkwork_web_bootstrap::AlwaysReady),
    )
    .map_err(|error| format!("compose embedded feeds Open API failed: {error}"))
}

/// Idempotently ensures the standard feed streams exist for the embedded
/// community domain: `community-{circleId}`, `community-{circleId}-resources`
/// and the global `moments-global` stream.
async fn ensure_feeds_streams(
    host: &Arc<sdkwork_feeds_service_host::FeedsServiceHost>,
    community_host: &Arc<sdkwork_community_service_host::CommunityServiceHost>,
    tenant_id: &str,
) {
    use sdkwork_content_feeds_service::{CreateStreamCommand, FeedType};
    let feeds = host.service();

    async fn ensure_stream(
        feeds: &sdkwork_content_feeds_service::FeedsService,
        tenant_id: &str,
        stream_key: &str,
        feed_type: FeedType,
        title: &str,
    ) {
        if feeds.retrieve_stream_by_key(tenant_id, stream_key).await.is_ok() {
            return;
        }
        match feeds
            .create_stream(
                tenant_id,
                &CreateStreamCommand {
                    stream_key: stream_key.to_owned(),
                    feed_type,
                    title: title.to_owned(),
                    description: None,
                    visibility: "public".to_owned(),
                    sort_policy: "ranked".to_owned(),
                    config: None,
                },
            )
            .await
        {
            Ok(_) => tracing::info!(stream_key, "embedded feeds stream ensured"),
            Err(error) => tracing::warn!(stream_key, %error, "embedded feeds stream ensure failed"),
        }
    }

    ensure_stream(&feeds, tenant_id, "moments-global", FeedType::Moments, "朋友圈").await;
    match community_host.service().list_categories(tenant_id).await {
        Ok(circles) => {
            for circle in circles {
                ensure_stream(
                    &feeds,
                    tenant_id,
                    &format!("community-{}", circle.id),
                    FeedType::Community,
                    &circle.title,
                )
                .await;
                ensure_stream(
                    &feeds,
                    tenant_id,
                    &format!("community-{}-resources", circle.id),
                    FeedType::Community,
                    &format!("{} · 资源", circle.title),
                )
                .await;
            }
        }
        Err(error) => tracing::warn!(
            %error,
            "embedded feeds stream bootstrap: list community circles failed"
        ),
    }
}

/// Spawns the periodic feeds stream sync loop inside the gateway process
/// (mirrors `sdkwork-content-feeds-worker`; 60s interval).
fn spawn_feeds_sync_task(host: Arc<sdkwork_feeds_service_host::FeedsServiceHost>) {
    use sdkwork_content_feeds_service::{ListStreamsCommand, SyncStreamCommand};
    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(60);
        loop {
            tokio::time::sleep(interval).await;
            let tenant_id = std::env::var("SDKWORK_FEEDS_DEFAULT_TENANT_ID")
                .unwrap_or_else(|_| "100001".to_owned());
            let (streams, _total) = match host
                .service()
                .list_streams(&ListStreamsCommand {
                    tenant_id: tenant_id.clone(),
                    page: 1,
                    page_size: 100,
                })
                .await
            {
                Ok(page) => page,
                Err(error) => {
                    tracing::warn!(%error, "embedded feeds sync: list streams failed");
                    continue;
                }
            };
            let mut synced = 0i64;
            for stream in streams {
                match host
                    .service()
                    .sync_stream(
                        &tenant_id,
                        &SyncStreamCommand {
                            stream_id: stream.id.clone(),
                            source_type: None,
                        },
                    )
                    .await
                {
                    Ok(count) => {
                        synced += count;
                        tracing::info!(
                            stream_key = %stream.stream_key,
                            synced = count,
                            "embedded feeds stream synced"
                        );
                    }
                    Err(error) => tracing::warn!(
                        stream_key = %stream.stream_key,
                        %error,
                        "embedded feeds stream sync failed"
                    ),
                }
            }
            tracing::info!(synced, "embedded feeds sync tick completed");
        }
    });
}

/// Embed the membership backend business surface so circle tier publishing
/// (`memberships/packages` registration) works through the collapsed ingress.
async fn bootstrap_embedded_membership_backend_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    let assembly = sdkwork_api_membership_assembly::assemble_backend_business_router_from_env()
        .await
        .map_err(|error| format!("compose embedded membership Backend API failed: {error}"))?;
    sdkwork_web_bootstrap::ApiAssemblyContribution::from_manifest(
        "sdkwork-membership-backend",
        "SDKWork Membership Backend API",
        assembly.router,
        sdkwork_routes_membership_backend_api::gateway_route_manifest(),
        Vec::new(),
        Arc::new(sdkwork_web_bootstrap::AlwaysReady),
    )
    .map_err(|error| format!("compose embedded membership Backend API failed: {error}"))
}

/// Embed the order backend business surface so the community membership
/// activation can verify the paid order through the collapsed ingress.
async fn bootstrap_embedded_order_backend_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    let assembly = sdkwork_api_order_assembly::assemble_backend_business_router_from_env()
        .await
        .map_err(|error| format!("compose embedded order Backend API failed: {error}"))?;
    sdkwork_web_bootstrap::ApiAssemblyContribution::from_manifest(
        "sdkwork-order-backend",
        "SDKWork Order Backend API",
        assembly.router,
        sdkwork_routes_order_backend_api::gateway_route_manifest(),
        Vec::new(),
        Arc::new(sdkwork_web_bootstrap::AlwaysReady),
    )
    .map_err(|error| format!("compose embedded order Backend API failed: {error}"))
}

async fn bootstrap_embedded_course_routes()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_course_assembly::assemble_api_router()
        .await
        .map_err(|error| format!("compose embedded course App API failed: {error}"))
}

async fn bootstrap_embedded_catalog_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_catalog_assembly::assemble_api_router_from_env()
        .await
        .map_err(|error| format!("compose embedded catalog App API failed: {error}"))
}

async fn bootstrap_embedded_mail_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_mail_assembly::assemble_api_router()
        .await
        .map_err(|error| format!("compose embedded mail App API failed: {error}"))
}

async fn bootstrap_embedded_account_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_account_assembly::assemble_app_api_contribution_from_env()
        .await
        .map_err(|error| format!("compose embedded account App API failed: {error}"))
}

async fn bootstrap_embedded_drive_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    ensure_drive_tenant_application_bootstrap_from_env().await?;
    sdkwork_api_drive_assembly::assemble_app_api_contribution().await
}

async fn bootstrap_embedded_knowledgebase_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_knowledgebase_assembly::assemble_app_api_contribution_from_environment()
        .await
        .map_err(|error| format!("compose embedded knowledgebase App API failed: {error}"))
}

async fn bootstrap_embedded_inventory_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_inventory_assembly::assemble_app_api_contribution_from_env().await
}

async fn bootstrap_embedded_invoice_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_invoice_assembly::assemble_app_api_contribution_from_env().await
}

/// Bootstrap the merchandise database module only.
///
/// sdkwork-merchandise exposes no public App API surface of its own, but its
/// baseline owns tables that sibling domains join at query time (e.g.
/// membership reads `commerce_product_sku`); the standalone gateway runs its
/// database lifecycle so those tables exist in the shared PostgreSQL profile.
async fn bootstrap_embedded_merchandise_database() -> Result<(), String> {
    sdkwork_merchandise_database_host::bootstrap_merchandise_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("bootstrap embedded merchandise database failed: {error}"))
}

/// Bootstrap the promotion database module only.
///
/// sdkwork-promotion exposes no public App API surface of its own, but its
/// baseline owns the coupon tables (`promotion_code`, `promotion_user_coupon`,
/// `promotion_offer_version`) that the order coupon redemption port reads; the
/// standalone gateway runs its database lifecycle so coupon redemption works.
async fn bootstrap_embedded_promotion_database() -> Result<(), String> {
    sdkwork_promotion_database_host::bootstrap_promotion_database_from_env()
        .await
        .map(|_| ())
        .map_err(|error| format!("bootstrap embedded promotion database failed: {error}"))
}

async fn bootstrap_embedded_payment_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_payment_assembly::assemble_federated_app_api_contribution_from_env().await
}

async fn bootstrap_embedded_shop_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_shop_assembly::assemble_app_api_contribution_from_env().await
}

async fn bootstrap_embedded_notary_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_notary_assembly::assemble_app_api_contribution()
        .await
        .map_err(|error| format!("compose embedded notary App API failed: {error}"))
}

async fn build_embedded_agents_runtime() -> Result<EmbeddedAgentsRuntime, String> {
    let runtime = sdkwork_api_agents_assembly::assemble_app_runtime_contribution()
        .await
        .map_err(|error| format!("compose embedded agents app routes failed: {error}"))?;
    Ok(EmbeddedAgentsRuntime {
        contribution: runtime.api,
        session_facade: runtime.session_facade,
    })
}

fn apply_agents_runtime_env_from_im_shared_profile() -> Result<(), String> {
    if std::env::var("SDKWORK_AGENTS_ENVIRONMENT")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var(
            "SDKWORK_AGENTS_ENVIRONMENT",
            normalize_knowledgebase_environment(
                std::env::var("SDKWORK_IM_ENVIRONMENT")
                    .ok()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| "development".to_owned())
                    .as_str(),
            ),
        );
    }
    if std::env::var("SDKWORK_AGENTS_TENANT_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_AGENTS_TENANT_ID", "100001");
    }
    if std::env::var("SDKWORK_AGENTS_ORGANIZATION_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_AGENTS_ORGANIZATION_ID", "0");
    }
    Ok(())
}

fn apply_knowledgebase_runtime_env_from_im_shared_profile() -> Result<(), String> {
    if std::env::var("SDKWORK_KNOWLEDGEBASE_ENVIRONMENT")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var(
            "SDKWORK_KNOWLEDGEBASE_ENVIRONMENT",
            normalize_knowledgebase_environment(
                std::env::var("SDKWORK_IM_ENVIRONMENT")
                    .ok()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| "development".to_owned())
                    .as_str(),
            ),
        );
    }
    if std::env::var("SDKWORK_KNOWLEDGEBASE_ORGANIZATION_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_KNOWLEDGEBASE_ORGANIZATION_ID", "0");
    }
    if std::env::var("SDKWORK_KNOWLEDGEBASE_TENANT_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_KNOWLEDGEBASE_TENANT_ID", "100001");
    }
    Ok(())
}

async fn ensure_drive_tenant_application_bootstrap_from_env() -> Result<(), String> {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "development".to_owned());
    let app_root = resolve_drive_app_root();
    ensure_tenant_application_from_app_root_with_env_and_fallback(
        environment.as_str(),
        app_root,
        None,
        &[],
    )
    .await
}

fn resolve_drive_app_root() -> PathBuf {
    resolve_sibling_app_root("sdkwork-drive")
}

fn resolve_sibling_app_root(directory: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(directory)
        .canonicalize()
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../..")
                .join(directory)
        })
}

/// Set an environment variable.
///
/// # Safety
///
/// `std::env::set_var` is unsafe because it is not thread-safe. This function
/// must only be called from `fn main()` before the Tokio runtime is created
/// (i.e., before any other threads exist). The `apply_embedded_dependency_env`
/// entry point enforces this contract.
fn set_env_var(key: &str, value: &str) {
    // SAFETY: Called from apply_embedded_dependency_env which is invoked
    // synchronously from fn main() before tokio::runtime::Builder::build().
    unsafe {
        std::env::set_var(key, value);
    }
}

fn normalize_knowledgebase_environment(raw: &str) -> &'static str {
    match raw.trim().to_ascii_lowercase().as_str() {
        "dev" | "development" => "development",
        "test" | "testing" => "test",
        "prod" | "production" => "production",
        "staging" => "staging",
        _ => "development",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_knowledgebase_runtime_env_from_im_shared_profile, normalize_knowledgebase_environment,
    };

    #[test]
    fn apply_knowledgebase_runtime_env_defaults_align_with_iam_bootstrap_ids() {
        unsafe {
            std::env::remove_var("SDKWORK_KNOWLEDGEBASE_TENANT_ID");
            std::env::remove_var("SDKWORK_KNOWLEDGEBASE_ORGANIZATION_ID");
        }
        apply_knowledgebase_runtime_env_from_im_shared_profile()
            .expect("knowledgebase runtime defaults must resolve");
        assert_eq!(
            std::env::var("SDKWORK_KNOWLEDGEBASE_TENANT_ID").expect("tenant id"),
            "100001"
        );
        assert_eq!(
            std::env::var("SDKWORK_KNOWLEDGEBASE_ORGANIZATION_ID").expect("organization id"),
            "0"
        );
    }

    #[test]
    fn normalize_knowledgebase_environment_maps_dev_aliases() {
        assert_eq!(normalize_knowledgebase_environment("dev"), "development");
        assert_eq!(
            normalize_knowledgebase_environment("development"),
            "development"
        );
    }
}
