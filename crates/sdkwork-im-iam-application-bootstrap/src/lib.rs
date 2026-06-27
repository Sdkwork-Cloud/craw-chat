//! Thin Sdkwork IM adapter over the shared embedded IAM tenant application bootstrap.

use std::path::PathBuf;

use sdkwork_iam_embedded_application_bootstrap::{
    ensure_tenant_application_from_app_root_with_env_and_fallback, resolve_application_app_root,
    EmbeddedApplicationBootstrapOptions, EmbeddedApplicationRuntimeBinding,
};
use sqlx::PgPool;

pub const IM_PC_RUNTIME_APP_ID: &str = "sdkwork-im-pc";
pub const IM_H5_RUNTIME_APP_ID: &str = "sdkwork-im-h5";
pub const IM_FLUTTER_MOBILE_RUNTIME_APP_ID: &str = "sdkwork-im-flutter-mobile";

pub fn im_pc_runtime_binding() -> EmbeddedApplicationRuntimeBinding {
    EmbeddedApplicationRuntimeBinding {
        runtime_app_id: IM_PC_RUNTIME_APP_ID.to_owned(),
        display_name: Some("Sdkwork IM".to_owned()),
        app_key_override: None,
        instance_key_override: None,
    }
}

pub fn im_h5_runtime_binding() -> EmbeddedApplicationRuntimeBinding {
    EmbeddedApplicationRuntimeBinding {
        runtime_app_id: IM_H5_RUNTIME_APP_ID.to_owned(),
        display_name: Some("Sdkwork IM H5".to_owned()),
        app_key_override: None,
        instance_key_override: None,
    }
}

pub fn im_flutter_mobile_runtime_binding() -> EmbeddedApplicationRuntimeBinding {
    EmbeddedApplicationRuntimeBinding {
        runtime_app_id: IM_FLUTTER_MOBILE_RUNTIME_APP_ID.to_owned(),
        display_name: Some("Sdkwork IM Flutter Mobile".to_owned()),
        app_key_override: None,
        instance_key_override: None,
    }
}

pub async fn ensure_im_tenant_application_runtime(
    pg: &PgPool,
    environment: &str,
) -> Result<(), String> {
    let app_root = resolve_im_repo_root();
    let manifest = sdkwork_iam_embedded_application_bootstrap::load_manifest_from_app_root(
        app_root.as_path(),
    )?;
    let options = EmbeddedApplicationBootstrapOptions {
        environment: environment.to_owned(),
        ..EmbeddedApplicationBootstrapOptions::default()
    };
    sdkwork_iam_embedded_application_bootstrap::ensure_tenant_applications_on_pool(
        pg,
        &manifest,
        &options,
        Some(&im_pc_runtime_binding()),
        &[im_h5_runtime_binding(), im_flutter_mobile_runtime_binding()],
    )
    .await
}

pub async fn ensure_im_tenant_application_runtime_from_env(
    environment: &str,
) -> Result<(), String> {
    let app_root = resolve_im_repo_root();
    sdkwork_iam_database_host::unified_postgres_env::apply_unified_claw_postgres_env(&app_root);
    ensure_tenant_application_from_app_root_with_env_and_fallback(
        environment,
        app_root,
        Some(&im_pc_runtime_binding()),
        &[im_h5_runtime_binding(), im_flutter_mobile_runtime_binding()],
    )
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
        load_manifest_from_app_root, manifest_to_ensure_command,
        normalize_bootstrap_environment, EmbeddedApplicationBootstrapOptions,
    };

    #[test]
    fn im_repo_root_resolves_repository_manifest() {
        let root = resolve_im_repo_root();
        assert!(root.join("sdkwork.app.config.json").is_file());
    }

    #[test]
    fn im_pc_runtime_binding_uses_shared_instance_key_rules() {
        let manifest =
            load_manifest_from_app_root(resolve_im_repo_root().as_path()).expect("manifest");
        let command = manifest_to_ensure_command(
            &manifest,
            &EmbeddedApplicationBootstrapOptions {
                environment: "development".to_owned(),
                ..EmbeddedApplicationBootstrapOptions::default()
            },
            Some(&im_pc_runtime_binding()),
        )
        .expect("command");
        assert_eq!("chat", command.app_key);
        assert_eq!(IM_PC_RUNTIME_APP_ID, command.runtime_app_id);
        assert_eq!("sdkwork_im_pc_dev", command.instance_key);
        assert_eq!("dev", command.environment);
        assert!(!command.default_access_permissions.is_empty());
    }

    #[test]
    fn im_h5_runtime_binding_uses_shared_instance_key_rules() {
        let manifest =
            load_manifest_from_app_root(resolve_im_repo_root().as_path()).expect("manifest");
        let command = manifest_to_ensure_command(
            &manifest,
            &EmbeddedApplicationBootstrapOptions {
                environment: "production".to_owned(),
                ..EmbeddedApplicationBootstrapOptions::default()
            },
            Some(&im_h5_runtime_binding()),
        )
        .expect("command");
        assert_eq!(IM_H5_RUNTIME_APP_ID, command.runtime_app_id);
        assert_eq!("sdkwork_im_h5_prod", command.instance_key);
        assert_eq!(
            "prod",
            normalize_bootstrap_environment("production")
        );
    }

    #[test]
    fn im_flutter_mobile_runtime_binding_uses_shared_instance_key_rules() {
        let manifest =
            load_manifest_from_app_root(resolve_im_repo_root().as_path()).expect("manifest");
        let command = manifest_to_ensure_command(
            &manifest,
            &EmbeddedApplicationBootstrapOptions {
                environment: "development".to_owned(),
                ..EmbeddedApplicationBootstrapOptions::default()
            },
            Some(&im_flutter_mobile_runtime_binding()),
        )
        .expect("command");
        assert_eq!(IM_FLUTTER_MOBILE_RUNTIME_APP_ID, command.runtime_app_id);
        assert_eq!("sdkwork_im_flutter_mobile_dev", command.instance_key);
        assert_eq!("dev", command.environment);
    }
}
