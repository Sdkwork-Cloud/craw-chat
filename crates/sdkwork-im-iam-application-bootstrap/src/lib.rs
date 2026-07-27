//! Thin Sdkwork IM adapter over the shared embedded IAM tenant application bootstrap.

use std::path::PathBuf;

use sdkwork_iam_embedded_application_bootstrap::{
    EmbeddedApplicationBootstrapOptions,
    ensure_tenant_application_from_app_root_with_env_and_fallback,
    ensure_tenant_applications_from_app_root_on_pool, resolve_application_app_root,
};
use sqlx::PgPool;

pub async fn ensure_im_tenant_application_runtime(
    pg: &PgPool,
    environment: &str,
) -> Result<(), String> {
    let app_root = resolve_im_repo_root();
    let options = EmbeddedApplicationBootstrapOptions {
        environment: environment.to_owned(),
        ..EmbeddedApplicationBootstrapOptions::default()
    };
    ensure_tenant_applications_from_app_root_on_pool(pg, app_root.as_path(), &options, None, &[])
        .await
}

pub async fn ensure_im_tenant_application_runtime_from_env(
    environment: &str,
) -> Result<(), String> {
    let app_root = resolve_im_repo_root();
    sdkwork_iam_database_host::unified_postgres_env::apply_unified_claw_postgres_env(&app_root);
    ensure_tenant_application_from_app_root_with_env_and_fallback(environment, app_root, None, &[])
        .await
}

fn resolve_im_repo_root() -> PathBuf {
    resolve_application_app_root().unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_iam_embedded_application_bootstrap::{
        discover_application_manifest_roots, load_manifest_from_app_root,
        resolve_manifest_runtime_app_bindings,
    };

    #[test]
    fn im_repo_root_resolves_repository_manifest() {
        let root = resolve_im_repo_root();
        assert!(root.join("sdkwork.app.config.json").is_file());
    }

    #[test]
    fn im_application_manifests_declare_runtime_identities() {
        let repo_root = resolve_im_repo_root();
        let manifest_roots =
            discover_application_manifest_roots(repo_root.as_path()).expect("manifest roots");
        let declared_runtime_app_ids = manifest_roots
            .into_iter()
            .flat_map(|manifest_root| {
                let manifest =
                    load_manifest_from_app_root(manifest_root.as_path()).expect("manifest");
                resolve_manifest_runtime_app_bindings(&manifest)
                    .into_iter()
                    .map(|binding| binding.runtime_app_id)
            })
            .collect::<Vec<_>>();

        assert!(declared_runtime_app_ids.contains(&"sdkwork-im-pc".to_owned()));
        assert!(declared_runtime_app_ids.contains(&"sdkwork-im-h5".to_owned()));
        assert!(declared_runtime_app_ids.contains(&"sdkwork-im-flutter-mobile".to_owned()));
    }
}
