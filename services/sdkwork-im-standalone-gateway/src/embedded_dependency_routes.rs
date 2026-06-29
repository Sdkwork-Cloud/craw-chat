//! Standalone unified-process dependency API surfaces (Drive, Knowledgebase, Commerce, Mail, Notary, Course).
//!
//! Sibling domain route crates are mounted in-process per `APPLICATION_GATEWAY_SPEC.md`
//! platform consumer linking and `DEPENDENCY_MANAGEMENT_SPEC.md` §5 — not HTTP-proxied
//! to split-service ports when IM standalone gateway collapses platform ingress.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use sdkwork_drive_workspace_service::application::download_service::ensure_production_download_token_signing_configured;
use sdkwork_drive_workspace_service::bootstrap::bootstrap_drive_database_from_env;
use sdkwork_drive_workspace_service::infrastructure::outbox_dispatch::ensure_domain_outbox_dispatcher;
use sdkwork_drive_workspace_service::infrastructure::sql::connect_any_database_and_install_schema;
use sdkwork_iam_embedded_application_bootstrap::ensure_tenant_application_from_app_root_with_env_and_fallback;
use sdkwork_knowledgebase_gateway_assembly::{
    resolve_database_url, validate_process_config, KnowledgebaseRuntime,
};

pub struct EmbeddedDependencyRoutes {
    pub router: Router,
}

struct CommerceT1Module {
    env_prefix: &'static str,
    repo_dir: &'static str,
    sqlite_file: &'static str,
}

const COMMERCE_T1_MODULES: &[CommerceT1Module] = &[
    CommerceT1Module {
        env_prefix: "SDKWORK_ACCOUNT",
        repo_dir: "sdkwork-account",
        sqlite_file: "account.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_CATALOG",
        repo_dir: "sdkwork-catalog",
        sqlite_file: "catalog.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_INVENTORY",
        repo_dir: "sdkwork-inventory",
        sqlite_file: "inventory.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_INVOICE",
        repo_dir: "sdkwork-invoice",
        sqlite_file: "invoice.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_MEMBERSHIP",
        repo_dir: "sdkwork-membership",
        sqlite_file: "membership.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_MERCHANDISE",
        repo_dir: "sdkwork-merchandise",
        sqlite_file: "merchandise.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_ORDER",
        repo_dir: "sdkwork-order",
        sqlite_file: "order.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_PAYMENT",
        repo_dir: "sdkwork-payment",
        sqlite_file: "payment.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_PROMOTION",
        repo_dir: "sdkwork-promotion",
        sqlite_file: "promotion.sqlite",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_SHOP",
        repo_dir: "sdkwork-shop",
        sqlite_file: "shop.sqlite",
    },
];

/// Apply all embedded dependency environment variables synchronously.
///
/// This must be called from the main thread BEFORE the Tokio runtime is created
/// to avoid data races on the process environment. After this returns, all
/// `SDKWORK_*_DATABASE_URL`, `SDKWORK_KNOWLEDGEBASE_*`, commerce T1 `SDKWORK_*_APP_ROOT`,
/// and related env vars are resolved and the async bootstrap functions can
/// safely read them.
///
/// # Safety
///
/// See `set_env_var` safety contract — callers must ensure no other threads exist.
pub fn apply_embedded_dependency_env() {
    let _ = apply_drive_database_env_from_im_shared_profile();
    apply_commerce_t1_database_env_from_im_shared_profile();
    apply_mail_database_env_from_im_shared_profile();
    apply_knowledgebase_runtime_env_from_im_shared_profile();
    apply_agents_runtime_env_from_im_shared_profile();
    let _ = apply_notary_database_env_from_im_shared_profile();
    apply_course_runtime_env_from_im_shared_profile();
    apply_iam_database_env_from_im_shared_profile();
    apply_web_store_database_env_from_im_shared_profile();
    apply_commerce_t1_app_roots_from_im_shared_profile();
    apply_embedded_dependency_app_roots();
    normalize_embedded_dependency_database_urls();
    set_env_var(
        "SDKWORK_NOTARY_APP_ROOT",
        resolve_notary_app_root().to_string_lossy().as_ref(),
    );
    set_env_var(
        "SDKWORK_COURSE_APP_ROOT",
        resolve_course_app_root().to_string_lossy().as_ref(),
    );
}

/// Run sdkwork-database lifecycle init/migrate for every embedded dependency that owns a database module.
///
/// This mirrors IM/IAM startup in `main.rs` and satisfies `DATABASE_FRAMEWORK_SPEC.md` §4.3 for
/// unified-process standalone gateways that mount sibling platform APIs in-process.
pub async fn bootstrap_embedded_dependency_databases() -> Result<(), String> {
    sync_embedded_dependency_database("drive", sync_drive_embedded_database).await?;
    sync_embedded_dependency_database("knowledgebase", sync_knowledgebase_embedded_database).await?;
    sync_optional_embedded_dependency_database("web_store", sync_webstore_embedded_database).await;
    sync_embedded_dependency_database("mail", sync_mail_embedded_database).await?;
    sync_embedded_dependency_database("notary", sync_notary_embedded_database).await?;
    sync_embedded_dependency_database("course", sync_course_embedded_database).await?;
    bootstrap_embedded_commerce_databases().await?;
    sync_optional_embedded_dependency_database("agents", sync_agents_embedded_database).await;
    Ok(())
}

async fn sync_optional_embedded_dependency_database<F, Fut>(
    dependency: &'static str,
    bootstrap: F,
) where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    match bootstrap().await {
        Ok(()) => {
            tracing::info!(
                target: "sdkwork.im",
                event = "im.standalone_gateway.dependency_database_synced",
                dependency,
                "embedded dependency database lifecycle synchronized"
            );
        }
        Err(error) => {
            tracing::warn!(
                target: "sdkwork.im",
                event = "im.standalone_gateway.dependency_database_skipped",
                dependency,
                error = %error,
                "optional embedded dependency database lifecycle sync skipped"
            );
        }
    }
}

async fn sync_embedded_dependency_database<F, Fut>(
    dependency: &'static str,
    bootstrap: F,
) -> Result<(), String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<(), String>>,
{
    match bootstrap().await {
        Ok(()) => {
            tracing::info!(
                target: "sdkwork.im",
                event = "im.standalone_gateway.dependency_database_synced",
                dependency,
                "embedded dependency database lifecycle synchronized"
            );
            Ok(())
        }
        Err(error) => {
            if embedded_dependency_database_env_is_configured(dependency) {
                Err(format!("{dependency} database lifecycle sync failed: {error}"))
            } else {
                tracing::info!(
                    target: "sdkwork.im",
                    event = "im.standalone_gateway.dependency_database_skipped",
                    dependency,
                    error = %error,
                    "embedded dependency database sync skipped"
                );
                Ok(())
            }
        }
    }
}

async fn sync_drive_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_DRIVE") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("drive", "sdkwork-drive")?;
    ensure_embedded_dependency_app_root("SDKWORK_DRIVE", "sdkwork-drive");
    bootstrap_drive_database_from_env().await?;
    Ok(())
}

async fn sync_knowledgebase_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_KNOWLEDGEBASE") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("knowledgebase", "sdkwork-knowledgebase")?;
    ensure_embedded_dependency_app_root("SDKWORK_KNOWLEDGEBASE", "sdkwork-knowledgebase");
    sdkwork_knowledgebase_database_host::bootstrap_knowledgebase_database_from_env().await?;
    Ok(())
}

async fn sync_webstore_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_WEB_STORE") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("web_store", "sdkwork-web-framework")?;
    set_env_var(
        "SDKWORK_WEB_STORE_APP_ROOT",
        resolve_sibling_app_root("sdkwork-web-framework")
            .to_string_lossy()
            .as_ref(),
    );
    sdkwork_webstore_database_host::bootstrap_webstore_database_from_env().await?;
    Ok(())
}

async fn sync_mail_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_MAIL") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("mail", "sdkwork-mail")?;
    ensure_embedded_dependency_app_root("SDKWORK_MAIL", "sdkwork-mail");
    sdkwork_mail_database_host::bootstrap_mail_database_from_env().await?;
    Ok(())
}

async fn sync_notary_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_NOTARY") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("notary", "sdkwork-notary")?;
    ensure_embedded_dependency_app_root("SDKWORK_NOTARY", "sdkwork-notary");
    sdkwork_notary_database_host::bootstrap_notary_database_from_env().await?;
    Ok(())
}

async fn sync_course_embedded_database() -> Result<(), String> {
    if !database_env_is_configured("SDKWORK_COURSE") {
        return Ok(());
    }
    ensure_embedded_database_module_ready("course", "sdkwork-course")?;
    ensure_embedded_dependency_app_root("SDKWORK_COURSE", "sdkwork-course");
    sdkwork_course_database_host::bootstrap_course_database_from_env().await?;
    Ok(())
}

async fn bootstrap_embedded_commerce_databases() -> Result<(), String> {
    for module in COMMERCE_T1_MODULES {
        sync_optional_embedded_dependency_database(commerce_t1_dependency_id(module), || {
            sync_commerce_t1_module_database(module)
        })
        .await;
    }
    Ok(())
}

fn commerce_t1_dependency_id(module: &CommerceT1Module) -> &'static str {
    match module.env_prefix {
        "SDKWORK_ACCOUNT" => "account",
        "SDKWORK_CATALOG" => "catalog",
        "SDKWORK_INVENTORY" => "inventory",
        "SDKWORK_INVOICE" => "invoice",
        "SDKWORK_MEMBERSHIP" => "membership",
        "SDKWORK_MERCHANDISE" => "merchandise",
        "SDKWORK_ORDER" => "order",
        "SDKWORK_PAYMENT" => "payment",
        "SDKWORK_PROMOTION" => "promotion",
        "SDKWORK_SHOP" => "shop",
        other => other,
    }
}

async fn sync_commerce_t1_module_database(module: &CommerceT1Module) -> Result<(), String> {
    if !database_env_is_configured(module.env_prefix) {
        return Ok(());
    }
    if !embedded_database_manifest_available(module.repo_dir) {
        tracing::info!(
            target: "sdkwork.im",
            event = "im.standalone_gateway.dependency_database_module_unavailable",
            dependency = commerce_t1_dependency_id(module),
            repo_dir = module.repo_dir,
            "embedded commerce database module unavailable; skipping lifecycle sync"
        );
        return Ok(());
    }
    ensure_embedded_dependency_app_root(module.env_prefix, module.repo_dir);
    match module.env_prefix {
        "SDKWORK_ACCOUNT" => {
            sdkwork_account_service_host::AccountServiceHost::from_env().await?;
        }
        "SDKWORK_CATALOG" => {
            sdkwork_catalog_service_host::CatalogServiceHost::from_env().await?;
        }
        "SDKWORK_INVENTORY" => {
            sdkwork_inventory_service_host::InventoryServiceHost::from_env().await?;
        }
        "SDKWORK_INVOICE" => {
            sdkwork_invoice_service_host::InvoiceServiceHost::from_env().await?;
        }
        "SDKWORK_MEMBERSHIP" => {
            sdkwork_membership_service_host::MembershipServiceHost::from_env().await?;
        }
        "SDKWORK_MERCHANDISE" => {
            sdkwork_merchandise_service_host::ShopServiceHost::from_env().await?;
        }
        "SDKWORK_ORDER" => {
            sdkwork_order_service_host::OrderServiceHost::from_env().await?;
        }
        "SDKWORK_PAYMENT" => {
            sdkwork_payment_service_host::PaymentServiceHost::from_env().await?;
        }
        "SDKWORK_PROMOTION" => {
            sdkwork_promotion_service_host::PromotionServiceHost::from_env().await?;
        }
        "SDKWORK_SHOP" => {
            sdkwork_shop_service_host::ShopServiceHost::from_env().await?;
        }
        other => {
            return Err(format!(
                "unsupported commerce database env prefix for embedded sync: {other}"
            ));
        }
    }
    Ok(())
}

async fn sync_agents_embedded_database() -> Result<(), String> {
    if !agents_database_env_is_configured("SDKWORK_AGENTS")
        && !agents_database_env_is_configured("SDKWORK_AGENTS_STORE")
        && !agents_database_env_is_configured("SDKWORK_AGENT_SERVER")
    {
        return Ok(());
    }
    if !embedded_database_manifest_available("sdkwork-agents") {
        tracing::info!(
            target: "sdkwork.im",
            event = "im.standalone_gateway.dependency_database_module_unavailable",
            dependency = "agents",
            repo_dir = "sdkwork-agents",
            "embedded agents database module unavailable; skipping lifecycle sync"
        );
        return Ok(());
    }
    ensure_embedded_dependency_app_root("SDKWORK_AGENTS", "sdkwork-agents");
    tokio::task::spawn_blocking(|| {
        use sdkwork_intelligence_agents_service::SyncPostgresAdapter;
        let adapter = SyncPostgresAdapter::connect_from_agents_managed_store_env()
            .map_err(|error| format!("connect agents managed store postgres adapter failed: {error}"))?;
        adapter
            .apply_managed_store_schema()
            .map_err(|error| format!("apply agents managed store postgres schema failed: {error}"))
    })
    .await
    .map_err(|error| format!("agents database bootstrap worker failed: {error}"))??;
    Ok(())
}

fn embedded_dependency_database_env_is_configured(dependency: &str) -> bool {
    match dependency {
        "drive" => database_env_is_configured("SDKWORK_DRIVE"),
        "knowledgebase" => database_env_is_configured("SDKWORK_KNOWLEDGEBASE"),
        "mail" => database_env_is_configured("SDKWORK_MAIL"),
        "notary" => database_env_is_configured("SDKWORK_NOTARY"),
        "course" => database_env_is_configured("SDKWORK_COURSE"),
        "account" => database_env_is_configured("SDKWORK_ACCOUNT"),
        "catalog" => database_env_is_configured("SDKWORK_CATALOG"),
        "inventory" => database_env_is_configured("SDKWORK_INVENTORY"),
        "invoice" => database_env_is_configured("SDKWORK_INVOICE"),
        "membership" => database_env_is_configured("SDKWORK_MEMBERSHIP"),
        "merchandise" => database_env_is_configured("SDKWORK_MERCHANDISE"),
        "order" => database_env_is_configured("SDKWORK_ORDER"),
        "payment" => database_env_is_configured("SDKWORK_PAYMENT"),
        "promotion" => database_env_is_configured("SDKWORK_PROMOTION"),
        "shop" => database_env_is_configured("SDKWORK_SHOP"),
        "agents" => agents_database_env_is_configured("SDKWORK_AGENTS")
            || agents_database_env_is_configured("SDKWORK_AGENTS_STORE")
            || agents_database_env_is_configured("SDKWORK_AGENT_SERVER"),
        _ => false,
    }
}

fn apply_embedded_dependency_app_roots() {
    for (prefix, repo_dir) in [
        ("SDKWORK_DRIVE", "sdkwork-drive"),
        ("SDKWORK_KNOWLEDGEBASE", "sdkwork-knowledgebase"),
        ("SDKWORK_MAIL", "sdkwork-mail"),
    ] {
        if !embedded_database_manifest_available(repo_dir) {
            continue;
        }
        ensure_embedded_dependency_app_root(prefix, repo_dir);
    }
}

fn embedded_database_manifest_available(repo_dir: &str) -> bool {
    resolve_sibling_app_root(repo_dir)
        .join("database/database.manifest.json")
        .is_file()
}

fn ensure_embedded_database_module_ready(dependency: &str, repo_dir: &str) -> Result<(), String> {
    if embedded_database_manifest_available(repo_dir) {
        return Ok(());
    }
    Err(format!(
        "{dependency} database module not found at sibling repo `{repo_dir}`; ensure the repository is checked out next to sdkwork-im"
    ))
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

fn normalize_embedded_dependency_database_urls() {
    normalize_postgres_database_env("SDKWORK_DRIVE");
    normalize_postgres_database_env("SDKWORK_KNOWLEDGEBASE");
    normalize_postgres_database_env("SDKWORK_WEB_STORE");
    normalize_postgres_database_env("SDKWORK_MAIL");
    normalize_postgres_database_env("SDKWORK_NOTARY");
    normalize_postgres_database_env("SDKWORK_COURSE");
    for module in COMMERCE_T1_MODULES {
        normalize_postgres_database_env(module.env_prefix);
    }
    for prefix in ["SDKWORK_AGENTS", "SDKWORK_AGENTS_STORE", "SDKWORK_AGENT_SERVER"] {
        normalize_postgres_database_env(prefix);
    }
}

fn normalize_postgres_database_env(prefix: &str) {
    let url_key = format!("{prefix}_DATABASE_URL");
    let Ok(url) = std::env::var(&url_key) else {
        return;
    };
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return;
    }
    let normalized_scheme = trimmed.to_ascii_lowercase();
    if !normalized_scheme.starts_with("postgres://") && !normalized_scheme.starts_with("postgresql://")
    {
        return;
    }
    let normalized =
        sdkwork_database_config::claw_database::postgres_url_with_search_path(trimmed, prefix);
    if normalized != trimmed {
        set_env_var(url_key.as_str(), normalized.as_str());
    }
}

pub async fn bootstrap_embedded_dependency_routes() -> EmbeddedDependencyRoutes {
    let mut router = Router::new();
    router = merge_embedded_dependency(router, "drive", bootstrap_embedded_drive_routes).await;
    router = merge_embedded_dependency(router, "knowledgebase", bootstrap_embedded_knowledgebase_routes).await;
    router = merge_embedded_dependency(router, "commerce", bootstrap_embedded_commerce_routes).await;
    router = merge_embedded_dependency(router, "mail", bootstrap_embedded_mail_routes).await;
    router = merge_embedded_dependency(router, "notary", bootstrap_embedded_notary_routes).await;
    router = merge_embedded_dependency(router, "course", bootstrap_embedded_course_routes).await;
    router = merge_embedded_dependency(router, "agents", bootstrap_embedded_agents_routes).await;
    EmbeddedDependencyRoutes { router }
}

async fn merge_embedded_dependency<F, Fut>(
    router: Router,
    dependency: &'static str,
    bootstrap: F,
) -> Router
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Router, String>>,
{
    match bootstrap().await {
        Ok(dependency_router) => router.merge(dependency_router),
        Err(error) => {
            tracing::warn!(
                target: "sdkwork.im",
                event = "im.standalone_gateway.dependency_bootstrap_skipped",
                dependency,
                error = %error,
                "embedded dependency bootstrap skipped"
            );
            router
        }
    }
}

async fn bootstrap_embedded_drive_routes() -> Result<Router, String> {
    ensure_production_download_token_signing_configured()
        .map_err(|error| format!("drive download token signing config invalid: {error}"))?;
    sdkwork_drive_security::ensure_drive_auth_policy_refresh_task();

    let database_config = sdkwork_drive_config::DatabaseConfig::from_env()
        .map_err(|error| format!("resolve drive database config failed: {error}"))?;
    let pool = connect_any_database_and_install_schema(&database_config)
        .await
        .map_err(|error| format!("create drive database pool failed: {error}"))?;
    ensure_domain_outbox_dispatcher(pool.clone());

    ensure_drive_tenant_application_bootstrap_from_env().await?;

    let assembly =
        sdkwork_drive_gateway_assembly::assemble_application_business_router(pool.clone()).await;
    Ok(assembly.router)
}

async fn bootstrap_embedded_knowledgebase_routes() -> Result<Router, String> {
    validate_process_config();

    let database_url = resolve_database_url();
    let tenant_id = resolve_embedded_knowledgebase_tenant_id();

    let runtime = Arc::new(
        KnowledgebaseRuntime::connect(database_url.as_str(), tenant_id)
            .await
            .map_err(|error| format!("initialize knowledgebase runtime failed: {error}"))?,
    );
    runtime
        .readiness_check()
        .await
        .map_err(|error| format!("knowledgebase database readiness check failed: {error}"))?;

    Ok(
        sdkwork_knowledgebase_gateway_assembly::assemble_application_business_router(runtime)
            .await
            .router,
    )
}

async fn bootstrap_embedded_mail_routes() -> Result<Router, String> {
    sdkwork_mail_gateway_assembly::assemble_application_router()
        .await
        .map(|assembly| assembly.router)
        .map_err(|error| format!("compose embedded mail router failed: {error}"))
}

async fn bootstrap_embedded_agents_routes() -> Result<Router, String> {
    let state = tokio::task::spawn_blocking(build_embedded_agents_http_state)
        .await
        .map_err(|error| format!("agents bootstrap worker failed: {error}"))??;
    Ok(sdkwork_routes_agents_app_api::build_served_router(state).await)
}

fn build_embedded_agents_http_state(
) -> Result<sdkwork_intelligence_agents_service::AgentHttpState, String> {
    use sdkwork_intelligence_agents_service::{
        AgentBusinessIdGenerator, AgentHttpState, IamGatedPolicyProvider, PostgresAgentAuditSink,
        PostgresAgentRepository, SyncPostgresAdapter, AUDIT_SINK_NODE_ID,
    };

    let repository_adapter = SyncPostgresAdapter::connect_from_agents_managed_store_env()
        .map_err(|error| format!("connect agents managed store postgres adapter failed: {error}"))?;
    repository_adapter
        .apply_managed_store_schema()
        .map_err(|error| format!("apply agents managed store postgres schema failed: {error}"))?;

    let audit_adapter = {
        let audit_pool = repository_adapter.pool().clone();
        let audit_id_generator = AgentBusinessIdGenerator::with_node_id(AUDIT_SINK_NODE_ID)
            .map_err(|error| format!("build agents audit sink snowflake id generator failed: {error}"))?;
        SyncPostgresAdapter::with_pool_and_id_generator(audit_pool, audit_id_generator)
    };

    Ok(AgentHttpState::new(
        PostgresAgentRepository::new(repository_adapter),
        PostgresAgentAuditSink::new_global(audit_adapter),
        IamGatedPolicyProvider::default(),
    ))
}

async fn bootstrap_embedded_commerce_routes() -> Result<Router, String> {
    let mut router = Router::new();
    router = merge_embedded_dependency(router, "account", bootstrap_embedded_account_routes).await;
    router = merge_embedded_dependency(router, "catalog", bootstrap_embedded_catalog_routes).await;
    router =
        merge_embedded_dependency(router, "inventory", bootstrap_embedded_inventory_routes).await;
    router = merge_embedded_dependency(router, "invoice", bootstrap_embedded_invoice_routes).await;
    router =
        merge_embedded_dependency(router, "membership", bootstrap_embedded_membership_routes).await;
    router = merge_embedded_dependency(
        router,
        "merchandise",
        bootstrap_embedded_merchandise_routes,
    )
    .await;
    router = merge_embedded_dependency(router, "order", bootstrap_embedded_order_routes).await;
    router = merge_embedded_dependency(router, "payment", bootstrap_embedded_payment_routes).await;
    router =
        merge_embedded_dependency(router, "promotion", bootstrap_embedded_promotion_routes).await;
    router = merge_embedded_dependency(router, "shop", bootstrap_embedded_shop_routes).await;
    Ok(router)
}

async fn bootstrap_embedded_account_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_account_service_host::AccountServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap account service host failed: {error}"))?,
    );
    Ok(
        sdkwork_account_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_catalog_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_catalog_service_host::CatalogServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap catalog service host failed: {error}"))?,
    );
    Ok(
        sdkwork_catalog_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_inventory_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_inventory_service_host::InventoryServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap inventory service host failed: {error}"))?,
    );
    Ok(
        sdkwork_inventory_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_invoice_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_invoice_service_host::InvoiceServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap invoice service host failed: {error}"))?,
    );
    Ok(
        sdkwork_invoice_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_membership_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_membership_service_host::MembershipServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap membership service host failed: {error}"))?,
    );
    Ok(
        sdkwork_membership_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_merchandise_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_merchandise_service_host::ShopServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap merchandise service host failed: {error}"))?,
    );
    Ok(
        sdkwork_merchandise_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_order_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_order_service_host::OrderServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap order service host failed: {error}"))?,
    );
    Ok(
        sdkwork_order_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_payment_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_payment_service_host::PaymentServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap payment service host failed: {error}"))?,
    );
    Ok(
        sdkwork_payment_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_promotion_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_promotion_service_host::PromotionServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap promotion service host failed: {error}"))?,
    );
    Ok(
        sdkwork_promotion_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_shop_routes() -> Result<Router, String> {
    let host = Arc::new(
        sdkwork_shop_service_host::ShopServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap shop service host failed: {error}"))?,
    );
    Ok(
        sdkwork_shop_gateway_assembly::assemble_application_router(host)
            .await
            .router,
    )
}

async fn bootstrap_embedded_notary_routes() -> Result<Router, String> {
    let assembly =
        sdkwork_notary_gateway_assembly::assemble_application_business_router().await?;
    Ok(assembly.router)
}

async fn bootstrap_embedded_course_routes() -> Result<Router, String> {
    let assembly =
        sdkwork_course_gateway_assembly::assemble_application_business_router().await?;
    Ok(assembly.router)
}

fn apply_notary_database_env_from_im_shared_profile() -> Result<(), String> {
    if notary_database_env_is_configured() {
        return Ok(());
    }

    let url = sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_NOTARY")
        .map_err(|error| format!("resolve notary database URL from IM shared profile failed: {error}"))?;
    bridge_database_env_from_im_shared_profile("SDKWORK_NOTARY", url.as_str(), Some("notary.sqlite"))
}

fn apply_iam_database_env_from_im_shared_profile() {
    if std::env::var("SDKWORK_IAM_DATABASE_URL")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
    {
        return;
    }
    if let Ok(url) =
        sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_IAM")
    {
        set_env_var("SDKWORK_IAM_DATABASE_URL", url.as_str());
    }
}

fn apply_web_store_database_env_from_im_shared_profile() {
    set_env_var(
        "SDKWORK_WEB_STORE_APP_ROOT",
        resolve_sibling_app_root("sdkwork-web-framework")
            .to_string_lossy()
            .as_ref(),
    );
    if database_env_is_configured("SDKWORK_WEB_STORE") {
        return;
    }
    if let Ok(url) =
        sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_WEB_STORE")
    {
        let normalized = url.to_ascii_lowercase();
        let resolved = if normalized.starts_with("postgres://")
            || normalized.starts_with("postgresql://")
        {
            sdkwork_database_config::claw_database::postgres_url_with_search_path(
                url.as_str(),
                "SDKWORK_WEB_STORE",
            )
        } else if normalized.starts_with("sqlite:") {
            sibling_sqlite_database_url(url.as_str(), "web-store.sqlite").unwrap_or(url)
        } else {
            url
        };
        set_env_var("SDKWORK_WEB_STORE_DATABASE_URL", resolved.as_str());
    }
}

fn notary_database_env_is_configured() -> bool {
    std::env::var("SDKWORK_NOTARY_DATABASE_URL")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn resolve_notary_app_root() -> PathBuf {
    resolve_sibling_app_root("sdkwork-notary")
}

fn resolve_course_app_root() -> PathBuf {
    resolve_sibling_app_root("sdkwork-course")
}

fn apply_drive_database_env_from_im_shared_profile() -> Result<(), String> {
    if drive_database_env_is_configured() {
        return Ok(());
    }

    let url = sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_DRIVE")
        .map_err(|error| format!("resolve drive database URL from IM shared profile failed: {error}"))?;
    bridge_database_env_from_im_shared_profile("SDKWORK_DRIVE", url.as_str(), Some("drive.sqlite"))
}

fn apply_commerce_t1_database_env_from_im_shared_profile() {
    for module in COMMERCE_T1_MODULES {
        if t1_database_env_is_configured(module.env_prefix) {
            continue;
        }
        if let Ok(url) =
            sdkwork_database_config::claw_database::resolve_unified_database_url(module.env_prefix)
        {
            let _ = bridge_database_env_from_im_shared_profile(
                module.env_prefix,
                url.as_str(),
                Some(module.sqlite_file),
            );
        }
    }
}

fn apply_commerce_t1_app_roots_from_im_shared_profile() {
    for module in COMMERCE_T1_MODULES {
        if !embedded_database_manifest_available(module.repo_dir) {
            continue;
        }
        ensure_embedded_dependency_app_root(module.env_prefix, module.repo_dir);
    }
}

fn t1_database_env_is_configured(prefix: &str) -> bool {
    database_env_is_configured(prefix)
}

fn apply_mail_database_env_from_im_shared_profile() {
    if mail_database_env_is_configured() {
        return;
    }
    if let Ok(url) =
        sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_MAIL")
    {
        let _ = bridge_database_env_from_im_shared_profile("SDKWORK_MAIL", url.as_str(), Some("mail.sqlite"));
    }
}

fn bridge_database_env_from_im_shared_profile(
    prefix: &str,
    url: &str,
    sqlite_filename: Option<&str>,
) -> Result<(), String> {
    let normalized = url.to_ascii_lowercase();
    if normalized.starts_with("postgres://") || normalized.starts_with("postgresql://") {
        let url = sdkwork_database_config::claw_database::postgres_url_with_search_path(
            url,
            prefix,
        );
        set_env_var(&format!("{prefix}_DATABASE_URL"), url.as_str());
        return Ok(());
    }
    if normalized.starts_with("sqlite:") {
        let sibling_filename = match sqlite_filename {
            Some(filename) => filename,
            None => match prefix {
                "SDKWORK_DRIVE" => "drive.sqlite",
                "SDKWORK_MAIL" => "mail.sqlite",
                "SDKWORK_NOTARY" => "notary.sqlite",
                "SDKWORK_COURSE" => "course.sqlite",
                other => {
                    return Err(format!(
                        "unsupported sqlite bridge prefix without filename: {other}"
                    ));
                }
            },
        };
        let sibling_url = sibling_sqlite_database_url(url, sibling_filename)?;
        if prefix == "SDKWORK_DRIVE" {
            set_env_var("SDKWORK_DRIVE_DATABASE_ENGINE", "sqlite");
            set_env_var("SDKWORK_DRIVE_DATABASE_SQLITE_URL", sibling_url.as_str());
            set_env_var("SDKWORK_DRIVE_DATABASE_URL", sibling_url.as_str());
        } else {
            set_env_var(&format!("{prefix}_DATABASE_URL"), sibling_url.as_str());
        }
        return Ok(());
    }

    Err(format!(
        "unsupported database URL scheme from IM shared profile for {prefix}: {url}"
    ))
}

fn apply_agents_runtime_env_from_im_shared_profile() {
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
    set_env_var(
        "SDKWORK_AGENTS_APP_ROOT",
        resolve_sibling_app_root("sdkwork-agents")
            .to_string_lossy()
            .as_ref(),
    );
    apply_agents_database_env_from_im_shared_profile();
}

fn apply_agents_database_env_from_im_shared_profile() {
    for prefix in ["SDKWORK_AGENTS", "SDKWORK_AGENTS_STORE", "SDKWORK_AGENT_SERVER"] {
        if agents_database_env_is_configured(prefix) {
            continue;
        }
        if let Ok(url) =
            sdkwork_database_config::claw_database::resolve_unified_database_url(prefix)
        {
            let _ = bridge_database_env_from_im_shared_profile(prefix, url.as_str(), None);
        }
    }
}

fn agents_database_env_is_configured(prefix: &str) -> bool {
    database_env_is_configured(prefix)
}

fn apply_course_runtime_env_from_im_shared_profile() {
    if std::env::var("SDKWORK_COURSE_ENVIRONMENT")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var(
            "SDKWORK_COURSE_ENVIRONMENT",
            normalize_course_environment(
                std::env::var("SDKWORK_IM_ENVIRONMENT")
                    .ok()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| "development".to_owned())
                    .as_str(),
            ),
        );
    }
    if std::env::var("SDKWORK_COURSE_ORGANIZATION_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_COURSE_ORGANIZATION_ID", "0");
    }
    if std::env::var("SDKWORK_COURSE_TENANT_ID")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_COURSE_TENANT_ID", "100001");
    }
    if course_database_env_is_configured() {
        return;
    }
    if let Ok(url) =
        sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_COURSE")
    {
        let normalized = url.to_ascii_lowercase();
        let resolved = if normalized.starts_with("postgres://") || normalized.starts_with("postgresql://")
        {
            sdkwork_database_config::claw_database::postgres_url_with_search_path(
                url.as_str(),
                "SDKWORK_COURSE",
            )
        } else if normalized.starts_with("sqlite:") {
            sibling_sqlite_database_url(url.as_str(), "course.sqlite").unwrap_or(url)
        } else {
            url
        };
        set_env_var("SDKWORK_COURSE_DATABASE_URL", resolved.as_str());
    }
    bridge_course_integration_upstream_env(
        "SDKWORK_COURSE_AUDIT_URL",
        "SDKWORK_IM_AUDIT_SERVICE_UPSTREAM",
        "http://127.0.0.1:18089",
    );
    bridge_course_integration_upstream_env(
        "SDKWORK_COURSE_NOTIFICATION_URL",
        "SDKWORK_IM_NOTIFICATION_SERVICE_UPSTREAM",
        "http://127.0.0.1:18087",
    );
}

fn bridge_course_integration_upstream_env(
    target_env: &str,
    fallback_env: &str,
    development_default: &str,
) {
    if std::env::var(target_env)
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
    {
        return;
    }

    if let Ok(upstream) = std::env::var(fallback_env) {
        if !upstream.trim().is_empty() {
            set_env_var(target_env, upstream.trim());
            return;
        }
    }

    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .unwrap_or_else(|| "development".to_owned());
    if matches!(
        environment.trim().to_ascii_lowercase().as_str(),
        "dev" | "development" | "test" | "testing"
    ) {
        set_env_var(target_env, development_default);
    }
}

fn apply_knowledgebase_runtime_env_from_im_shared_profile() {
    if std::env::var("SDKWORK_KNOWLEDGEBASE_SERVICE_LAYOUT")
        .ok()
        .map(|value| value.trim().is_empty())
        .unwrap_or(true)
    {
        set_env_var("SDKWORK_KNOWLEDGEBASE_SERVICE_LAYOUT", "unified-process");
    }
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
    if knowledgebase_database_env_is_configured() {
        return;
    }
    if let Ok(url) =
        sdkwork_database_config::claw_database::resolve_unified_database_url("SDKWORK_KNOWLEDGEBASE")
    {
        let normalized = url.to_ascii_lowercase();
        let resolved = if normalized.starts_with("postgres://") || normalized.starts_with("postgresql://")
        {
            sdkwork_database_config::claw_database::postgres_url_with_search_path(
                url.as_str(),
                "SDKWORK_KNOWLEDGEBASE",
            )
        } else if normalized.starts_with("sqlite:") {
            sibling_sqlite_database_url(url.as_str(), "knowledgebase.db").unwrap_or(url)
        } else {
            url
        };
        set_env_var("SDKWORK_KNOWLEDGEBASE_DATABASE_URL", resolved.as_str());
    }
}

async fn ensure_drive_tenant_application_bootstrap_from_env() -> Result<(), String> {
    let environment = std::env::var("SDKWORK_IM_ENVIRONMENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "development".to_owned());
    let app_root = resolve_drive_app_root();
    sdkwork_iam_database_host::unified_postgres_env::apply_unified_claw_postgres_env(&app_root);
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

fn drive_database_env_is_configured() -> bool {
    database_env_is_configured("SDKWORK_DRIVE")
}

fn mail_database_env_is_configured() -> bool {
    database_env_is_configured("SDKWORK_MAIL")
}

fn course_database_env_is_configured() -> bool {
    std::env::var("SDKWORK_COURSE_DATABASE_URL")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn knowledgebase_database_env_is_configured() -> bool {
    std::env::var("SDKWORK_KNOWLEDGEBASE_DATABASE_URL")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn database_env_is_configured(prefix: &str) -> bool {
    std::env::var(format!("{prefix}_DATABASE_URL"))
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || std::env::var(format!("{prefix}_DATABASE_SQLITE_URL"))
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
        || std::env::var(format!("{prefix}_DATABASE_HOST"))
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
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

fn normalize_course_environment(raw: &str) -> &'static str {
    normalize_knowledgebase_environment(raw)
}

fn resolve_embedded_knowledgebase_tenant_id() -> u64 {
    std::env::var("SDKWORK_KNOWLEDGEBASE_TENANT_ID")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(100_001)
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

fn sibling_sqlite_database_url(base_sqlite_url: &str, sibling_filename: &str) -> Result<String, String> {
    let without_prefix = base_sqlite_url
        .trim()
        .strip_prefix("sqlite://")
        .ok_or_else(|| format!("invalid sqlite database URL: {base_sqlite_url}"))?;
    let (path_part, query) = without_prefix
        .split_once('?')
        .map(|(path, query)| (path, Some(query)))
        .unwrap_or((without_prefix, None));
    let parent = Path::new(path_part)
        .parent()
        .ok_or_else(|| format!("sqlite database URL has no parent directory: {base_sqlite_url}"))?;
    let sibling_path = parent.join(sibling_filename);
    let mut url = format!(
        "sqlite://{}",
        sibling_path.to_string_lossy().replace('\\', "/")
    );
    if let Some(query) = query.filter(|value| !value.is_empty()) {
        url.push('?');
        url.push_str(query);
    } else if sibling_filename == "knowledgebase.db" {
        url.push_str("?mode=rwc");
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::{
        apply_course_runtime_env_from_im_shared_profile,
        apply_knowledgebase_runtime_env_from_im_shared_profile, normalize_knowledgebase_environment,
        sibling_sqlite_database_url,
    };

    #[test]
    fn apply_course_runtime_env_defaults_align_with_iam_bootstrap_ids() {
        unsafe {
            std::env::remove_var("SDKWORK_COURSE_TENANT_ID");
            std::env::remove_var("SDKWORK_COURSE_ORGANIZATION_ID");
        }
        apply_course_runtime_env_from_im_shared_profile();
        assert_eq!(
            std::env::var("SDKWORK_COURSE_TENANT_ID").expect("tenant id"),
            "100001"
        );
        assert_eq!(
            std::env::var("SDKWORK_COURSE_ORGANIZATION_ID").expect("organization id"),
            "0"
        );
    }

    #[test]
    fn apply_knowledgebase_runtime_env_defaults_align_with_iam_bootstrap_ids() {
        unsafe {
            std::env::remove_var("SDKWORK_KNOWLEDGEBASE_TENANT_ID");
            std::env::remove_var("SDKWORK_KNOWLEDGEBASE_ORGANIZATION_ID");
        }
        apply_knowledgebase_runtime_env_from_im_shared_profile();
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
        assert_eq!(normalize_knowledgebase_environment("development"), "development");
    }

    #[test]
    fn sibling_sqlite_database_url_uses_data_dir_sibling_file() {
        let url = sibling_sqlite_database_url("sqlite:///tmp/chat/data/chat.sqlite", "drive.sqlite")
            .expect("sqlite sibling url should resolve");
        assert_eq!(url, "sqlite:///tmp/chat/data/drive.sqlite");

        let commerce = sibling_sqlite_database_url(
            "sqlite:///tmp/chat/data/chat.sqlite",
            "commerce.sqlite",
        )
        .expect("commerce sqlite sibling url should resolve");
        assert_eq!(commerce, "sqlite:///tmp/chat/data/commerce.sqlite");

        let mail = sibling_sqlite_database_url("sqlite:///tmp/chat/data/chat.sqlite", "mail.sqlite")
            .expect("mail sqlite sibling url should resolve");
        assert_eq!(mail, "sqlite:///tmp/chat/data/mail.sqlite");
    }
}
