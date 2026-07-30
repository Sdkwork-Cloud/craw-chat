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
    ("SDKWORK_ORDER", "sdkwork-order"),
    ("SDKWORK_PAYMENT", "sdkwork-payment"),
    ("SDKWORK_SHOP", "sdkwork-shop"),
    ("SDKWORK_NOTARY", "sdkwork-notary"),
    ("SDKWORK_AGENTS", "sdkwork-agents"),
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
    // Avoid overlapping `/app/v3/api/recharges/*` routes: sdkwork-order owns the surface, while
    // sdkwork-payment only provides a deprecated proxy implementation.
    set_env_var("SDKWORK_PAYMENT_DISABLE_RECHARGE_PROXY", "true");
    // Avoid overlapping `GET /app/v3/api/orders/{orderId}/payments`: sdkwork-order owns the
    // list handler (`payments.orderPayments.list`); payment uses federated mount options.
    set_env_var("SDKWORK_PAYMENT_FEDERATED_COMMERCE", "true");
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

pub async fn bootstrap_embedded_dependency_routes() -> Result<EmbeddedDependencyRoutes, String> {
    let mut contributions = vec![
        bootstrap_embedded_drive_contribution().await?,
        bootstrap_embedded_knowledgebase_contribution().await?,
        bootstrap_embedded_inventory_contribution().await?,
        bootstrap_embedded_invoice_contribution().await?,
        sdkwork_api_membership_assembly::assemble_app_api_contribution().await?,
        sdkwork_api_order_assembly::assemble_app_api_contribution().await?,
        bootstrap_embedded_payment_contribution().await?,
        bootstrap_embedded_shop_contribution().await?,
        sdkwork_api_notary_assembly::assemble_app_api_contribution().await?,
    ];
    let agents_runtime = build_embedded_agents_runtime().await?;
    let agents_session_facade = agents_runtime.session_facade;
    contributions.push(agents_runtime.contribution);
    Ok(EmbeddedDependencyRoutes {
        contributions,
        agents_session_facade,
    })
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

async fn bootstrap_embedded_payment_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_payment_assembly::assemble_app_api_contribution_from_env().await
}

async fn bootstrap_embedded_shop_contribution()
-> Result<sdkwork_web_bootstrap::ApiAssemblyContribution, String> {
    sdkwork_api_shop_assembly::assemble_app_api_contribution_from_env().await
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
