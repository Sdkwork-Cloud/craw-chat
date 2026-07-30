//! Standalone single-ingress dependency API surfaces (Drive, Knowledgebase, Commerce, Mail, Notary, Course).
//!
//! Sibling domain route crates are mounted in-process per `APPLICATION_GATEWAY_SPEC.md`
//! platform consumer linking and `DEPENDENCY_MANAGEMENT_SPEC.md` §5 — not HTTP-proxied
//! to internal HTTP service ports when IM standalone gateway collapses platform ingress.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use axum::Router;
use sdkwork_drive_workspace_service::application::download_service::ensure_production_download_token_signing_configured;
use sdkwork_drive_workspace_service::infrastructure::outbox_dispatch::ensure_domain_outbox_dispatcher;
use sdkwork_drive_workspace_service::infrastructure::sql::connect_any_database_and_install_schema;
use sdkwork_iam_embedded_application_bootstrap::ensure_tenant_application_from_app_root_with_env_and_fallback;

pub struct EmbeddedDependencyRoutes {
    pub router: Router,
    pub agents_session_facade: Option<Arc<dyn sdkwork_agents_runtime_facade::AgentsSessionFacade>>,
    pub agents_readiness_check: Option<Arc<dyn sdkwork_web_bootstrap::ReadinessCheck>>,
}

struct EmbeddedAgentsRuntime {
    router: Router,
    session_facade: Arc<dyn sdkwork_agents_runtime_facade::AgentsSessionFacade>,
    readiness_check: Arc<dyn sdkwork_web_bootstrap::ReadinessCheck>,
}

struct CommerceT1Module {
    env_prefix: &'static str,
    repo_dir: &'static str,
}

const COMMERCE_T1_MODULES: &[CommerceT1Module] = &[
    CommerceT1Module {
        env_prefix: "SDKWORK_ACCOUNT",
        repo_dir: "sdkwork-account",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_CATALOG",
        repo_dir: "sdkwork-catalog",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_INVENTORY",
        repo_dir: "sdkwork-inventory",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_INVOICE",
        repo_dir: "sdkwork-invoice",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_MEMBERSHIP",
        repo_dir: "sdkwork-membership",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_MERCHANDISE",
        repo_dir: "sdkwork-merchandise",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_ORDER",
        repo_dir: "sdkwork-order",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_PAYMENT",
        repo_dir: "sdkwork-payment",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_PROMOTION",
        repo_dir: "sdkwork-promotion",
    },
    CommerceT1Module {
        env_prefix: "SDKWORK_SHOP",
        repo_dir: "sdkwork-shop",
    },
];

static EMBEDDED_ACCOUNT_HOST: OnceLock<Arc<sdkwork_account_service_host::AccountServiceHost>> =
    OnceLock::new();
static EMBEDDED_CATALOG_HOST: OnceLock<Arc<sdkwork_catalog_service_host::CatalogServiceHost>> =
    OnceLock::new();
static EMBEDDED_INVENTORY_HOST: OnceLock<
    Arc<sdkwork_inventory_service_host::InventoryServiceHost>,
> = OnceLock::new();
static EMBEDDED_INVOICE_HOST: OnceLock<Arc<sdkwork_invoice_service_host::InvoiceServiceHost>> =
    OnceLock::new();
static EMBEDDED_MEMBERSHIP_HOST: OnceLock<
    Arc<sdkwork_membership_service_host::MembershipServiceHost>,
> = OnceLock::new();
static EMBEDDED_MERCHANDISE_HOST: OnceLock<
    Arc<sdkwork_merchandise_service_host::MerchandiseServiceHost>,
> = OnceLock::new();
static EMBEDDED_ORDER_HOST: OnceLock<Arc<sdkwork_order_service_host::OrderServiceHost>> =
    OnceLock::new();
static EMBEDDED_PAYMENT_HOST: OnceLock<Arc<sdkwork_payment_service_host::PaymentServiceHost>> =
    OnceLock::new();
static EMBEDDED_PROMOTION_HOST: OnceLock<
    Arc<sdkwork_promotion_service_host::PromotionServiceHost>,
> = OnceLock::new();
static EMBEDDED_SHOP_HOST: OnceLock<Arc<sdkwork_shop_service_host::ShopServiceHost>> =
    OnceLock::new();

async fn embedded_account_service_host()
-> Result<Arc<sdkwork_account_service_host::AccountServiceHost>, String> {
    if let Some(host) = EMBEDDED_ACCOUNT_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_account_service_host::AccountServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap account service host failed: {error}"))?,
    );
    let _ = EMBEDDED_ACCOUNT_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_catalog_service_host()
-> Result<Arc<sdkwork_catalog_service_host::CatalogServiceHost>, String> {
    if let Some(host) = EMBEDDED_CATALOG_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_catalog_service_host::CatalogServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap catalog service host failed: {error}"))?,
    );
    let _ = EMBEDDED_CATALOG_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_inventory_service_host()
-> Result<Arc<sdkwork_inventory_service_host::InventoryServiceHost>, String> {
    if let Some(host) = EMBEDDED_INVENTORY_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_inventory_service_host::InventoryServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap inventory service host failed: {error}"))?,
    );
    let _ = EMBEDDED_INVENTORY_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_invoice_service_host()
-> Result<Arc<sdkwork_invoice_service_host::InvoiceServiceHost>, String> {
    if let Some(host) = EMBEDDED_INVOICE_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_invoice_service_host::InvoiceServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap invoice service host failed: {error}"))?,
    );
    let _ = EMBEDDED_INVOICE_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_membership_service_host()
-> Result<Arc<sdkwork_membership_service_host::MembershipServiceHost>, String> {
    if let Some(host) = EMBEDDED_MEMBERSHIP_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_membership_service_host::MembershipServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap membership service host failed: {error}"))?,
    );
    let _ = EMBEDDED_MEMBERSHIP_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_merchandise_service_host()
-> Result<Arc<sdkwork_merchandise_service_host::MerchandiseServiceHost>, String> {
    if let Some(host) = EMBEDDED_MERCHANDISE_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_merchandise_service_host::MerchandiseServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap merchandise service host failed: {error}"))?,
    );
    let _ = EMBEDDED_MERCHANDISE_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_order_service_host()
-> Result<Arc<sdkwork_order_service_host::OrderServiceHost>, String> {
    if let Some(host) = EMBEDDED_ORDER_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_order_service_host::OrderServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap order service host failed: {error}"))?,
    );
    let _ = EMBEDDED_ORDER_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_payment_service_host()
-> Result<Arc<sdkwork_payment_service_host::PaymentServiceHost>, String> {
    if let Some(host) = EMBEDDED_PAYMENT_HOST.get() {
        return Ok(host.clone());
    }
    set_env_var("SDKWORK_PAYMENT_DISABLE_RECHARGE_PROXY", "true");
    let host = Arc::new(
        sdkwork_payment_service_host::PaymentServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap payment service host failed: {error}"))?,
    );
    let _ = EMBEDDED_PAYMENT_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_promotion_service_host()
-> Result<Arc<sdkwork_promotion_service_host::PromotionServiceHost>, String> {
    if let Some(host) = EMBEDDED_PROMOTION_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_promotion_service_host::PromotionServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap promotion service host failed: {error}"))?,
    );
    let _ = EMBEDDED_PROMOTION_HOST.set(host.clone());
    Ok(host)
}

async fn embedded_shop_service_host()
-> Result<Arc<sdkwork_shop_service_host::ShopServiceHost>, String> {
    if let Some(host) = EMBEDDED_SHOP_HOST.get() {
        return Ok(host.clone());
    }
    let host = Arc::new(
        sdkwork_shop_service_host::ShopServiceHost::from_env()
            .await
            .map_err(|error| format!("bootstrap shop service host failed: {error}"))?,
    );
    let _ = EMBEDDED_SHOP_HOST.set(host.clone());
    Ok(host)
}

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
    apply_course_runtime_env_from_im_shared_profile()?;
    apply_web_store_app_root();
    apply_commerce_t1_app_roots_from_im_shared_profile();
    apply_embedded_dependency_app_roots();
    // Avoid overlapping `/app/v3/api/recharges/*` routes: sdkwork-order owns the surface, while
    // sdkwork-payment only provides a deprecated proxy implementation.
    set_env_var("SDKWORK_PAYMENT_DISABLE_RECHARGE_PROXY", "true");
    // Avoid overlapping `GET /app/v3/api/orders/{orderId}/payments`: sdkwork-order owns the
    // list handler (`payments.orderPayments.list`); payment uses federated mount options.
    set_env_var("SDKWORK_PAYMENT_FEDERATED_COMMERCE", "true");
    set_env_var(
        "SDKWORK_NOTARY_APP_ROOT",
        resolve_notary_app_root().to_string_lossy().as_ref(),
    );
    set_env_var(
        "SDKWORK_COURSE_APP_ROOT",
        resolve_course_app_root().to_string_lossy().as_ref(),
    );
    Ok(())
}

fn validate_workspace_server_database_env() -> Result<(), String> {
    let config = sdkwork_database_config::DatabaseConfig::from_env("IM")
        .map_err(|error| format!("resolve workspace database profile failed: {error}"))?;
    if config.engine != sdkwork_database_config::DatabaseEngine::Postgres {
        return Err("IM standalone gateway requires SDKWORK_DATABASE_ENGINE=postgresql".to_owned());
    }
    Ok(())
}

/// Run sdkwork-database lifecycle init/migrate for every embedded dependency that owns a database module.
///
/// This mirrors IM/IAM startup in `main.rs` and satisfies `DATABASE_FRAMEWORK_SPEC.md` §4.3 for
/// standalone gateways that mount sibling platform APIs in-process.
pub async fn bootstrap_embedded_dependency_databases() -> Result<(), String> {
    sync_embedded_dependency_database("drive", sync_drive_embedded_database).await?;
    sync_embedded_dependency_database("knowledgebase", sync_knowledgebase_embedded_database)
        .await?;
    sync_embedded_dependency_database("web_store", sync_webstore_embedded_database).await?;
    sync_embedded_dependency_database("mail", sync_mail_embedded_database).await?;
    sync_embedded_dependency_database("notary", sync_notary_embedded_database).await?;
    sync_embedded_dependency_database("course", sync_course_embedded_database).await?;
    bootstrap_embedded_commerce_databases().await?;
    sync_embedded_dependency_database("agents", sync_agents_embedded_database).await?;
    Ok(())
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
            if workspace_database_env_is_configured() {
                Err(format!(
                    "{dependency} database lifecycle sync failed: {error}"
                ))
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
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    ensure_embedded_database_module_ready("drive", "sdkwork-drive")?;
    ensure_embedded_dependency_app_root("SDKWORK_DRIVE", "sdkwork-drive");
    let database_config = sdkwork_drive_config::DatabaseConfig::from_env()
        .map_err(|error| format!("resolve drive database config failed: {error}"))?;
    connect_any_database_and_install_schema(&database_config)
        .await
        .map_err(|error| format!("drive database lifecycle sync failed: {error}"))?;
    Ok(())
}

async fn sync_knowledgebase_embedded_database() -> Result<(), String> {
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    ensure_embedded_database_module_ready("knowledgebase", "sdkwork-knowledgebase")?;
    ensure_embedded_dependency_app_root("SDKWORK_KNOWLEDGEBASE", "sdkwork-knowledgebase");
    sdkwork_knowledgebase_database_host::bootstrap_knowledgebase_database_from_env().await?;
    Ok(())
}

async fn sync_webstore_embedded_database() -> Result<(), String> {
    if !workspace_database_env_is_configured() {
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
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    ensure_embedded_database_module_ready("mail", "sdkwork-mail")?;
    ensure_embedded_dependency_app_root("SDKWORK_MAIL", "sdkwork-mail");
    sdkwork_mail_database_host::bootstrap_mail_database_from_env().await?;
    Ok(())
}

async fn sync_notary_embedded_database() -> Result<(), String> {
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    ensure_embedded_database_module_ready("notary", "sdkwork-notary")?;
    ensure_embedded_dependency_app_root("SDKWORK_NOTARY", "sdkwork-notary");
    sdkwork_notary_database_host::bootstrap_notary_database_from_env().await?;
    Ok(())
}

async fn sync_course_embedded_database() -> Result<(), String> {
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    ensure_embedded_database_module_ready("course", "sdkwork-course")?;
    ensure_embedded_dependency_app_root("SDKWORK_COURSE", "sdkwork-course");
    sdkwork_course_database_host::bootstrap_course_database_from_env().await?;
    Ok(())
}

async fn bootstrap_embedded_commerce_databases() -> Result<(), String> {
    for module in COMMERCE_T1_MODULES {
        sync_embedded_dependency_database(commerce_t1_dependency_id(module), || {
            sync_commerce_t1_module_database(module)
        })
        .await?;
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
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    if !embedded_database_manifest_available(module.repo_dir) {
        return ensure_embedded_database_module_ready(
            commerce_t1_dependency_id(module),
            module.repo_dir,
        );
    }
    ensure_embedded_dependency_app_root(module.env_prefix, module.repo_dir);
    match module.env_prefix {
        "SDKWORK_ACCOUNT" => {
            embedded_account_service_host().await?;
        }
        "SDKWORK_CATALOG" => {
            embedded_catalog_service_host().await?;
        }
        "SDKWORK_INVENTORY" => {
            embedded_inventory_service_host().await?;
        }
        "SDKWORK_INVOICE" => {
            embedded_invoice_service_host().await?;
        }
        "SDKWORK_MEMBERSHIP" => {
            embedded_membership_service_host().await?;
        }
        "SDKWORK_MERCHANDISE" => {
            embedded_merchandise_service_host().await?;
        }
        "SDKWORK_ORDER" => {
            embedded_order_service_host().await?;
        }
        "SDKWORK_PAYMENT" => {
            embedded_payment_service_host().await?;
        }
        "SDKWORK_PROMOTION" => {
            embedded_promotion_service_host().await?;
        }
        "SDKWORK_SHOP" => {
            embedded_shop_service_host().await?;
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
    if !workspace_database_env_is_configured() {
        return Ok(());
    }
    if !embedded_database_manifest_available("sdkwork-agents") {
        return ensure_embedded_database_module_ready("agents", "sdkwork-agents");
    }
    ensure_embedded_dependency_app_root("SDKWORK_AGENTS", "sdkwork-agents");
    sdkwork_api_agents_assembly::bootstrap_application_database_from_env()
        .await
        .map_err(|error| format!("agents database bootstrap failed: {error}"))?;
    Ok(())
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

pub async fn bootstrap_embedded_dependency_routes() -> Result<EmbeddedDependencyRoutes, String> {
    let mut router = Router::new();
    router = merge_embedded_dependency(router, "drive", bootstrap_embedded_drive_routes).await?;
    router = merge_embedded_dependency(
        router,
        "knowledgebase",
        bootstrap_embedded_knowledgebase_routes,
    )
    .await?;
    router =
        merge_embedded_dependency(router, "commerce", bootstrap_embedded_commerce_routes).await?;
    router = merge_embedded_dependency(router, "mail", bootstrap_embedded_mail_routes).await?;
    router = merge_embedded_dependency(router, "notary", bootstrap_embedded_notary_routes).await?;
    router = merge_embedded_dependency(router, "course", bootstrap_embedded_course_routes).await?;
    let agents_runtime = match build_embedded_agents_runtime().await {
        Ok(runtime) => Some(runtime),
        Err(error) if is_development_environment() => {
            eprintln!(
                "[sdkwork-api-im-standalone-gateway] optional dependency agents is unavailable in development; continuing without its routes: {error}"
            );
            None
        }
        Err(error) => {
            return Err(format!(
                "embedded dependency agents failed readiness and cannot be mounted: {error}"
            ));
        }
    };
    let agents_session_facade = agents_runtime
        .as_ref()
        .map(|runtime| runtime.session_facade.clone());
    let agents_readiness_check = agents_runtime
        .as_ref()
        .map(|runtime| runtime.readiness_check.clone());
    if let Some(runtime) = agents_runtime {
        router = router.merge(runtime.router);
    }
    Ok(EmbeddedDependencyRoutes {
        router,
        agents_session_facade,
        agents_readiness_check,
    })
}

async fn merge_embedded_dependency<F, Fut>(
    router: Router,
    dependency: &'static str,
    bootstrap: F,
) -> Result<Router, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Router, String>>,
{
    match bootstrap().await {
        Ok(dependency_router) => Ok(router.merge(dependency_router)),
        Err(error) if is_development_environment() => {
            eprintln!(
                "[sdkwork-api-im-standalone-gateway] optional dependency {dependency} is unavailable in development; continuing without its routes: {error}"
            );
            Ok(router)
        }
        Err(error) => Err(format!(
            "embedded dependency {dependency} failed readiness and cannot be mounted: {error}"
        )),
    }
}

fn is_development_environment() -> bool {
    std::env::var("SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT")
        .or_else(|_| std::env::var("SDKWORK_IM_ENVIRONMENT"))
        .or_else(|_| std::env::var("SDKWORK_ENV"))
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "dev" | "development" | "test" | "testing"
            )
        })
        .unwrap_or(false)
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

    let assembly = sdkwork_api_drive_assembly::assemble_business_routes(pool).await;
    Ok(assembly.router)
}

async fn bootstrap_embedded_knowledgebase_routes() -> Result<Router, String> {
    sdkwork_api_knowledgebase_assembly::assemble_business_routes_from_environment()
        .await
        .map(|assembly| assembly.router)
        .map_err(|error| format!("compose embedded knowledgebase router failed: {error}"))
}

async fn bootstrap_embedded_mail_routes() -> Result<Router, String> {
    sdkwork_api_mail_assembly::assemble_api_router()
        .await
        .map(|assembly| assembly.router)
        .map_err(|error| format!("compose embedded mail router failed: {error}"))
}

async fn build_embedded_agents_runtime() -> Result<EmbeddedAgentsRuntime, String> {
    let runtime = sdkwork_api_agents_assembly::assemble_app_runtime_contribution()
        .await
        .map_err(|error| format!("compose embedded agents app routes failed: {error}"))?;
    let contribution = runtime.api;
    let resolver = sdkwork_im_web_bootstrap::cached_iam_web_request_context_resolver()
        .unwrap_or_else(|| sdkwork_iam_web_adapter::IamWebRequestContextResolver::new(None));
    let mut layer = sdkwork_iam_web_adapter::build_iam_app_web_framework_layer(
        resolver,
        contribution.route_manifest,
    );
    for injector in contribution.domain_context_injectors {
        layer = layer.with_domain_injector(injector);
    }
    let router = sdkwork_web_axum::with_web_request_context(contribution.router, layer);

    Ok(EmbeddedAgentsRuntime {
        router,
        session_facade: runtime.session_facade,
        readiness_check: contribution.readiness_check,
    })
}

async fn bootstrap_embedded_commerce_routes() -> Result<Router, String> {
    let mut router = Router::new();
    router =
        merge_embedded_dependency(router, "account", bootstrap_embedded_account_routes).await?;
    router =
        merge_embedded_dependency(router, "catalog", bootstrap_embedded_catalog_routes).await?;
    router =
        merge_embedded_dependency(router, "inventory", bootstrap_embedded_inventory_routes).await?;
    router =
        merge_embedded_dependency(router, "invoice", bootstrap_embedded_invoice_routes).await?;
    router = merge_embedded_dependency(router, "membership", bootstrap_embedded_membership_routes)
        .await?;
    router =
        merge_embedded_dependency(router, "merchandise", bootstrap_embedded_merchandise_routes)
            .await?;
    router = merge_embedded_dependency(router, "order", bootstrap_embedded_order_routes).await?;
    router =
        merge_embedded_dependency(router, "payment", bootstrap_embedded_payment_routes).await?;
    router =
        merge_embedded_dependency(router, "promotion", bootstrap_embedded_promotion_routes).await?;
    router = merge_embedded_dependency(router, "shop", bootstrap_embedded_shop_routes).await?;
    Ok(router)
}

async fn bootstrap_embedded_account_routes() -> Result<Router, String> {
    let host = embedded_account_service_host().await?;
    Ok(sdkwork_api_account_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_catalog_routes() -> Result<Router, String> {
    let host = embedded_catalog_service_host().await?;
    Ok(sdkwork_api_catalog_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_inventory_routes() -> Result<Router, String> {
    let host = embedded_inventory_service_host().await?;
    Ok(sdkwork_api_inventory_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_invoice_routes() -> Result<Router, String> {
    let host = embedded_invoice_service_host().await?;
    Ok(sdkwork_api_invoice_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_membership_routes() -> Result<Router, String> {
    let host = embedded_membership_service_host().await?;
    Ok(sdkwork_api_membership_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_merchandise_routes() -> Result<Router, String> {
    let host = embedded_merchandise_service_host().await?;
    Ok(sdkwork_api_merchandise_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_order_routes() -> Result<Router, String> {
    let host = embedded_order_service_host().await?;
    Ok(sdkwork_api_order_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_payment_routes() -> Result<Router, String> {
    set_env_var("SDKWORK_PAYMENT_DISABLE_RECHARGE_PROXY", "true");
    let host = embedded_payment_service_host().await?;
    Ok(sdkwork_api_payment_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_promotion_routes() -> Result<Router, String> {
    let host = embedded_promotion_service_host().await?;
    Ok(sdkwork_api_promotion_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_shop_routes() -> Result<Router, String> {
    let host = embedded_shop_service_host().await?;
    Ok(sdkwork_api_shop_assembly::assemble_api_router(host)
        .await
        .router)
}

async fn bootstrap_embedded_notary_routes() -> Result<Router, String> {
    let assembly = sdkwork_api_notary_assembly::assemble_api_router().await?;
    Ok(assembly.router)
}

async fn bootstrap_embedded_course_routes() -> Result<Router, String> {
    let assembly = sdkwork_api_course_assembly::assemble_api_router().await?;
    Ok(assembly.router)
}

fn apply_web_store_app_root() {
    set_env_var(
        "SDKWORK_WEB_STORE_APP_ROOT",
        resolve_sibling_app_root("sdkwork-web-framework")
            .to_string_lossy()
            .as_ref(),
    );
}

fn resolve_notary_app_root() -> PathBuf {
    resolve_sibling_app_root("sdkwork-notary")
}

fn resolve_course_app_root() -> PathBuf {
    resolve_sibling_app_root("sdkwork-course")
}

fn apply_commerce_t1_app_roots_from_im_shared_profile() {
    for module in COMMERCE_T1_MODULES {
        if !embedded_database_manifest_available(module.repo_dir) {
            continue;
        }
        ensure_embedded_dependency_app_root(module.env_prefix, module.repo_dir);
    }
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
    set_env_var(
        "SDKWORK_AGENTS_APP_ROOT",
        resolve_sibling_app_root("sdkwork-agents")
            .to_string_lossy()
            .as_ref(),
    );
    Ok(())
}

fn apply_course_runtime_env_from_im_shared_profile() -> Result<(), String> {
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
    Ok(())
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

    if let Ok(upstream) = std::env::var(fallback_env)
        && !upstream.trim().is_empty()
    {
        set_env_var(target_env, upstream.trim());
        return;
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

fn workspace_database_env_is_configured() -> bool {
    sdkwork_database_config::workspace_database::workspace_database_env_is_configured()
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
        apply_course_runtime_env_from_im_shared_profile,
        apply_knowledgebase_runtime_env_from_im_shared_profile, is_development_environment,
        normalize_knowledgebase_environment,
    };

    #[test]
    fn apply_course_runtime_env_defaults_align_with_iam_bootstrap_ids() {
        unsafe {
            std::env::remove_var("SDKWORK_COURSE_TENANT_ID");
            std::env::remove_var("SDKWORK_COURSE_ORGANIZATION_ID");
        }
        apply_course_runtime_env_from_im_shared_profile()
            .expect("course runtime defaults must resolve");
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

    #[test]
    fn development_environment_allows_optional_dependency_degradation() {
        unsafe {
            std::env::set_var("SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT", "development");
        }
        assert!(is_development_environment());
        unsafe {
            std::env::set_var("SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT", "production");
        }
        assert!(!is_development_environment());
    }
}
