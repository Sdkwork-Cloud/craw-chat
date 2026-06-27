use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{ProviderDomain, RuntimeProviderRegistry, StaticProviderRegistry};
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

#[tokio::test]
async fn test_control_plane_exposes_provider_registry_snapshot_to_control_readers() {
    let app = governance_service::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_registry")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider registry request should return a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider registry body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider registry body should be valid json");

    assert_eq!(json["status"], "registry");
    assert_eq!(json["interfaceVersion"], "provider-registry/v1");
    assert_eq!(
        json["precedence"],
        serde_json::json!(["tenant_override", "deployment_profile", "global_default"])
    );

    let plugins = json["plugins"]
        .as_array()
        .expect("plugins should be returned as an array");
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin["pluginId"] == "rtc-volcengine"),
        "provider registry should surface the default rtc provider"
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin["pluginId"] == "object-storage-google"),
        "provider registry should surface google object storage support"
    );
    assert!(
        plugins
            .iter()
            .any(|plugin| plugin["pluginId"] == "principal-profile-upstream-context"),
        "provider registry should surface the upstream-context principal-profile provider"
    );
    let effective_bindings = json["effectiveBindings"]
        .as_array()
        .expect("effective bindings should be returned as an array");
    let rtc_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "rtc")
        .expect("rtc binding should be present");
    assert_eq!(rtc_binding["selectedPluginId"], "rtc-volcengine");

    let principal_profile_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "principal-profile")
        .expect("principal-profile binding should be present");
    assert_eq!(
        principal_profile_binding["selectedPluginId"],
        "principal-profile-upstream-context"
    );

    let object_storage_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "object-storage")
        .expect("object-storage binding should be present");
    assert_eq!(
        object_storage_binding["selectionSource"],
        serde_json::json!("deployment_required")
    );
    assert_eq!(
        object_storage_binding["selectedPluginId"],
        serde_json::Value::Null
    );
}

#[tokio::test]
async fn test_control_plane_exposes_deployment_profile_provider_bindings_to_control_readers() {
    let app = governance_service::build_app_with_cluster_and_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(
            StaticProviderRegistry::platform_default().with_deployment_profile(
                ProviderDomain::ObjectStorage,
                "object-storage-volcengine",
            ),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider bindings request should return a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider bindings body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider bindings body should be valid json");

    assert_eq!(json["status"], "bindings");
    assert_eq!(json["interfaceVersion"], "provider-registry/v1");
    assert_eq!(json["tenantId"], serde_json::Value::Null);
    assert_eq!(
        json["precedence"],
        serde_json::json!(["tenant_override", "deployment_profile", "global_default"])
    );

    let effective_bindings = json["effectiveBindings"]
        .as_array()
        .expect("effective bindings should be returned as an array");

    let rtc_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "rtc")
        .expect("rtc binding should be present");
    assert_eq!(rtc_binding["selectedPluginId"], "rtc-volcengine");
    assert_eq!(rtc_binding["selectionSource"], "global_default");

    let object_storage_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "object-storage")
        .expect("object-storage binding should be present");
    assert_eq!(
        object_storage_binding["selectedPluginId"],
        "object-storage-volcengine"
    );
    assert_eq!(
        object_storage_binding["selectionSource"],
        "deployment_profile"
    );
}

#[tokio::test]
async fn test_control_plane_exposes_tenant_override_provider_bindings_to_control_readers() {
    let app = governance_service::build_app_with_cluster_and_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(
            StaticProviderRegistry::platform_default()
                .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
                .with_tenant_override("t_provider_combo", ProviderDomain::Rtc, "rtc-aliyun")
                .with_tenant_override(
                    "t_provider_combo",
                    ProviderDomain::ObjectStorage,
                    "object-storage-aws",
                ),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings?tenantId=t_provider_combo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider bindings request should return a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider bindings body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider bindings body should be valid json");

    assert_eq!(json["status"], "bindings");
    assert_eq!(json["tenantId"], "t_provider_combo");

    let effective_bindings = json["effectiveBindings"]
        .as_array()
        .expect("effective bindings should be returned as an array");

    let rtc_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "rtc")
        .expect("rtc binding should be present");
    assert_eq!(rtc_binding["selectedPluginId"], "rtc-aliyun");
    assert_eq!(rtc_binding["selectionSource"], "tenant_override");

    let object_storage_binding = effective_bindings
        .iter()
        .find(|binding| binding["domain"] == "object-storage")
        .expect("object-storage binding should be present");
    assert_eq!(
        object_storage_binding["selectedPluginId"],
        "object-storage-aws"
    );
    assert_eq!(object_storage_binding["selectionSource"], "tenant_override");
}

#[tokio::test]
async fn test_control_plane_allows_control_writers_to_update_provider_policies_and_read_back_effective_bindings()
 {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment profile write should return a response");
    assert_eq!(deployment_write.status(), StatusCode::OK);
    let deployment_body = deployment_write
        .into_body()
        .collect()
        .await
        .expect("deployment body should collect")
        .to_bytes();
    let deployment_json: serde_json::Value =
        serde_json::from_slice(&deployment_body).expect("deployment body should be valid json");
    assert_eq!(deployment_json["status"], "applied");
    assert_eq!(deployment_json["tenantId"], serde_json::Value::Null);
    assert_eq!(deployment_json["currentVersion"], 2);
    assert_eq!(
        deployment_json["committedBinding"]["domain"],
        "object-storage"
    );
    assert_eq!(
        deployment_json["committedBinding"]["selectedPluginId"],
        "object-storage-volcengine"
    );
    assert_eq!(
        deployment_json["committedBinding"]["selectionSource"],
        "deployment_profile"
    );
    assert_eq!(deployment_json["diff"]["fromVersion"], 1);
    assert_eq!(deployment_json["diff"]["toVersion"], 2);
    assert!(
        deployment_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "object-storage"
                && binding["selectedPluginId"] == "object-storage-volcengine"
                && binding["selectionSource"] == "deployment_profile")
    );

    let tenant_preview = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_policies/preview")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant preview should return a response");
    assert_eq!(tenant_preview.status(), StatusCode::OK);
    let tenant_preview_body = tenant_preview
        .into_body()
        .collect()
        .await
        .expect("tenant preview body should collect")
        .to_bytes();
    let tenant_preview_json: serde_json::Value = serde_json::from_slice(&tenant_preview_body)
        .expect("tenant preview body should be valid json");
    assert_eq!(tenant_preview_json["status"], "preview");
    assert_eq!(tenant_preview_json["baseVersion"], 2);

    let tenant_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun","expectedBaseVersion":2}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant override write should return a response");
    assert_eq!(tenant_write.status(), StatusCode::OK);
    let tenant_body = tenant_write
        .into_body()
        .collect()
        .await
        .expect("tenant body should collect")
        .to_bytes();
    let tenant_json: serde_json::Value =
        serde_json::from_slice(&tenant_body).expect("tenant body should be valid json");
    assert_eq!(tenant_json["status"], "applied");
    assert_eq!(tenant_json["tenantId"], "t_provider_combo");
    assert_eq!(tenant_json["currentVersion"], 3);
    assert_eq!(tenant_json["committedBinding"]["domain"], "rtc");
    assert_eq!(
        tenant_json["committedBinding"]["selectedPluginId"],
        "rtc-aliyun"
    );
    assert_eq!(
        tenant_json["committedBinding"]["selectionSource"],
        "tenant_override"
    );
    assert_eq!(tenant_json["diff"]["fromVersion"], 2);
    assert_eq!(tenant_json["diff"]["toVersion"], 3);
    assert_eq!(
        tenant_json["diff"]["tenantOverrideChanges"],
        serde_json::json!([
            {
                "tenantId": "t_provider_combo",
                "domain": "rtc",
                "changeKind": "added",
                "fromPluginId": serde_json::Value::Null,
                "toPluginId": "rtc-aliyun"
            }
        ])
    );
    assert!(
        tenant_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "rtc"
                && binding["selectedPluginId"] == "rtc-aliyun"
                && binding["selectionSource"] == "tenant_override")
    );

    let tenant_read = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings?tenantId=t_provider_combo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tenant read should return a response");
    assert_eq!(tenant_read.status(), StatusCode::OK);
    let tenant_read_body = tenant_read
        .into_body()
        .collect()
        .await
        .expect("tenant read body should collect")
        .to_bytes();
    let tenant_read_json: serde_json::Value =
        serde_json::from_slice(&tenant_read_body).expect("tenant read body should be valid json");
    assert_eq!(tenant_read_json["status"], "bindings");
    assert!(
        tenant_read_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "rtc"
                && binding["selectedPluginId"] == "rtc-aliyun"
                && binding["selectionSource"] == "tenant_override")
    );
}

#[tokio::test]
async fn test_control_plane_rejects_cross_domain_provider_policy_write() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"rtc","pluginId":"object-storage-aws"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("cross-domain write should return a response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("cross-domain body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("cross-domain body should be valid json");
    assert_eq!(json["status"], 400);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "invalid_provider_policy");
}

#[tokio::test]
async fn test_control_plane_returns_explicit_noop_without_advancing_provider_policy_version() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let first_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first write should return a response");
    assert_eq!(first_write.status(), StatusCode::OK);

    let noop_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine","expectedBaseVersion":2}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("noop write should return a response");
    assert_eq!(noop_write.status(), StatusCode::OK);
    let noop_body = noop_write
        .into_body()
        .collect()
        .await
        .expect("noop body should collect")
        .to_bytes();
    let noop_json: serde_json::Value =
        serde_json::from_slice(&noop_body).expect("noop body should be valid json");
    assert_eq!(noop_json["status"], "noop");
    assert_eq!(noop_json["applied"], false);
    assert_eq!(noop_json["currentVersion"], 2);
    assert_eq!(
        noop_json["committedBinding"]["selectedPluginId"],
        "object-storage-volcengine"
    );
    assert_eq!(noop_json["diff"]["fromVersion"], 2);
    assert_eq!(noop_json["diff"]["toVersion"], 2);
    assert_eq!(
        noop_json["diff"]["deploymentProfileChanges"],
        serde_json::json!([])
    );
    assert_eq!(
        noop_json["diff"]["tenantOverrideChanges"],
        serde_json::json!([])
    );

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider policy history should return a response");
    assert_eq!(history_response.status(), StatusCode::OK);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history body should be valid json");
    assert_eq!(history_json["status"], "history");
    assert_eq!(history_json["currentVersion"], 2);
    assert_eq!(history_json["items"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_control_plane_exposes_provider_policy_history_and_supports_rollback() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment write should return a response");
    assert_eq!(deployment_write.status(), StatusCode::OK);

    let tenant_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant override write should return a response");
    assert_eq!(tenant_write.status(), StatusCode::OK);

    let history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider policy history should return a response");
    assert_eq!(history_response.status(), StatusCode::OK);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history body should be valid json");
    assert_eq!(history_json["status"], "history");
    assert_eq!(history_json["currentVersion"], 3);
    assert_eq!(history_json["items"].as_array().unwrap().len(), 3);
    assert!(
        history_json["items"][2]["tenantOverrides"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["tenantId"] == "t_provider_combo"),
        "policy history should expose the tenant override snapshot"
    );

    let rollback_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_policies/rollback")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetVersion":1}"#))
                .unwrap(),
        )
        .await
        .expect("rollback should return a response");
    assert_eq!(rollback_response.status(), StatusCode::OK);
    let rollback_body = rollback_response
        .into_body()
        .collect()
        .await
        .expect("rollback body should collect")
        .to_bytes();
    let rollback_json: serde_json::Value =
        serde_json::from_slice(&rollback_body).expect("rollback body should be valid json");
    assert_eq!(rollback_json["status"], "rolled_back");
    assert_eq!(rollback_json["currentVersion"], 4);
    assert_eq!(rollback_json["items"][3]["rollbackFromVersion"], 1);

    let global_read = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("global provider bindings should return a response after rollback");
    assert_eq!(global_read.status(), StatusCode::OK);
    let global_body = global_read
        .into_body()
        .collect()
        .await
        .expect("global read body should collect")
        .to_bytes();
    let global_json: serde_json::Value =
        serde_json::from_slice(&global_body).expect("global read body should be valid json");
    assert_eq!(global_json["status"], "bindings");
    assert!(
        global_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "object-storage"
                && binding["selectedPluginId"].is_null()
                && binding["selectionSource"] == "deployment_required")
    );

    let tenant_read = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings?tenantId=t_provider_combo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tenant provider bindings should return a response after rollback");
    assert_eq!(tenant_read.status(), StatusCode::OK);
    let tenant_body = tenant_read
        .into_body()
        .collect()
        .await
        .expect("tenant read body should collect")
        .to_bytes();
    let tenant_json: serde_json::Value =
        serde_json::from_slice(&tenant_body).expect("tenant read body should be valid json");
    assert_eq!(tenant_json["status"], "bindings");
    assert!(
        tenant_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "rtc"
                && binding["selectedPluginId"] == "rtc-volcengine"
                && binding["selectionSource"] == "global_default")
    );
}

#[tokio::test]
async fn test_control_plane_exposes_provider_policy_diff_between_committed_versions() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment write should return a response");
    assert_eq!(deployment_write.status(), StatusCode::OK);

    let deployment_update = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-aws"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment update should return a response");
    assert_eq!(deployment_update.status(), StatusCode::OK);

    let tenant_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant override write should return a response");
    assert_eq!(tenant_write.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies/diff?fromVersion=2&toVersion=4")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider policy diff should return a response");
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("diff body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("diff body should be valid json");

    assert_eq!(json["status"], "diff");
    assert_eq!(json["fromVersion"], 2);
    assert_eq!(json["toVersion"], 4);
    assert_eq!(
        json["deploymentProfileChanges"],
        serde_json::json!([
            {
                "domain": "object-storage",
                "changeKind": "changed",
                "fromPluginId": "object-storage-volcengine",
                "toPluginId": "object-storage-aws"
            }
        ])
    );
    assert_eq!(
        json["tenantOverrideChanges"],
        serde_json::json!([
            {
                "tenantId": "t_provider_combo",
                "domain": "rtc",
                "changeKind": "added",
                "fromPluginId": serde_json::Value::Null,
                "toPluginId": "rtc-aliyun"
            }
        ])
    );
}

#[tokio::test]
async fn test_control_plane_exposes_provider_policy_preview_without_mutation() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let preview_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_policies/preview")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("provider policy preview should return a response");
    assert_eq!(preview_response.status(), StatusCode::OK);

    let preview_body = preview_response
        .into_body()
        .collect()
        .await
        .expect("preview body should collect")
        .to_bytes();
    let preview_json: serde_json::Value =
        serde_json::from_slice(&preview_body).expect("preview body should be valid json");
    assert_eq!(preview_json["status"], "preview");
    assert_eq!(preview_json["baseVersion"], 1);
    assert_eq!(preview_json["previewVersion"], 2);
    assert_eq!(preview_json["tenantId"], "t_provider_combo");
    assert_eq!(preview_json["previewBinding"]["domain"], "rtc");
    assert_eq!(
        preview_json["previewBinding"]["selectedPluginId"],
        "rtc-aliyun"
    );
    assert_eq!(
        preview_json["previewBinding"]["selectionSource"],
        "tenant_override"
    );
    assert_eq!(preview_json["diff"]["fromVersion"], 1);
    assert_eq!(preview_json["diff"]["toVersion"], 2);
    assert_eq!(
        preview_json["diff"]["tenantOverrideChanges"],
        serde_json::json!([
            {
                "tenantId": "t_provider_combo",
                "domain": "rtc",
                "changeKind": "added",
                "fromPluginId": serde_json::Value::Null,
                "toPluginId": "rtc-aliyun"
            }
        ])
    );

    let history_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider policy history should return a response");
    assert_eq!(history_response.status(), StatusCode::OK);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history body should be valid json");
    assert_eq!(history_json["status"], "history");
    assert_eq!(history_json["currentVersion"], 1);
    assert_eq!(history_json["items"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_control_plane_rejects_stale_provider_policy_confirm_write_after_preview() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let preview_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_policies/preview")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("provider policy preview should return a response");
    assert_eq!(preview_response.status(), StatusCode::OK);
    let preview_body = preview_response
        .into_body()
        .collect()
        .await
        .expect("preview body should collect")
        .to_bytes();
    let preview_json: serde_json::Value =
        serde_json::from_slice(&preview_body).expect("preview body should be valid json");
    assert_eq!(preview_json["baseVersion"], 1);

    let concurrent_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("concurrent write should return a response");
    assert_eq!(concurrent_write.status(), StatusCode::OK);

    let stale_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun","expectedBaseVersion":1}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("stale write should return a response");
    assert_eq!(stale_write.status(), StatusCode::CONFLICT);
    let stale_body = stale_write
        .into_body()
        .collect()
        .await
        .expect("stale body should collect")
        .to_bytes();
    let stale_json: serde_json::Value =
        serde_json::from_slice(&stale_body).expect("stale body should be valid json");
    assert_eq!(stale_json["status"], 409);
    assert_eq!(stale_json["errorStatus"], "conflict");
    assert_eq!(stale_json["code"], "provider_policy_conflict");
    assert!(
        stale_json["message"]
            .as_str()
            .expect("conflict message should be present")
            .contains("expected 1"),
        "conflict message should expose the stale expected version"
    );

    let history_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider policy history should return a response");
    assert_eq!(history_response.status(), StatusCode::OK);
    let history_body = history_response
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history body should be valid json");
    assert_eq!(history_json["status"], "history");
    assert_eq!(history_json["currentVersion"], 2);
    assert_eq!(history_json["items"].as_array().unwrap().len(), 2);
    assert!(
        history_json["items"][1]["tenantOverrides"]
            .as_array()
            .unwrap()
            .is_empty(),
        "stale write must not append tenant override history"
    );

    let tenant_read = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_bindings?tenantId=t_provider_combo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tenant bindings should return a response");
    assert_eq!(tenant_read.status(), StatusCode::OK);
    let tenant_body = tenant_read
        .into_body()
        .collect()
        .await
        .expect("tenant body should collect")
        .to_bytes();
    let tenant_json: serde_json::Value =
        serde_json::from_slice(&tenant_body).expect("tenant body should be valid json");
    assert_eq!(tenant_json["status"], "bindings");
    assert!(
        tenant_json["effectiveBindings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|binding| binding["domain"] == "rtc"
                && binding["selectedPluginId"] == "rtc-volcengine"
                && binding["selectionSource"] == "global_default")
    );
}

#[tokio::test]
async fn test_control_plane_returns_unavailable_status_when_provider_policy_runtime_is_disabled() {
    let app = governance_service::build_app_with_cluster_and_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(StaticProviderRegistry::platform_default()),
    );

    for (method, uri, body, expected_code) in [
        (
            "POST",
            "/backend/v3/api/control/provider_bindings",
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
            "provider_policy_write_unavailable",
        ),
        (
            "POST",
            "/backend/v3/api/control/provider_policies/preview",
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
            "provider_policy_preview_unavailable",
        ),
        (
            "GET",
            "/backend/v3/api/control/provider_policies",
            None,
            "provider_policy_history_unavailable",
        ),
        (
            "GET",
            "/backend/v3/api/control/provider_policies/diff?fromVersion=1&toVersion=2",
            None,
            "provider_policy_diff_unavailable",
        ),
        (
            "POST",
            "/backend/v3/api/control/provider_policies/rollback",
            Some(r#"{"targetVersion":1}"#),
            "provider_policy_rollback_unavailable",
        ),
    ] {
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .with_dual_token_tenant("100001")
            .with_dual_token_user("1080")
            .with_dual_token_actor_kind("user")
            .with_dual_token_permission_scope("control.write");
        if body.is_some() {
            request = request.header("content-type", "application/json");
        }

        let response = app
            .clone()
            .oneshot(
                request
                    .body(body.map(Body::from).unwrap_or_else(Body::empty))
                    .unwrap(),
            )
            .await
            .expect("unavailable provider policy route should return a response");

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("unavailable body should collect")
            .to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("unavailable body should be valid json");
        assert_eq!(json["status"], 503);
        assert_eq!(json["errorStatus"], "unavailable");
        assert_eq!(json["code"], expected_code);
    }
}

#[tokio::test]
async fn test_control_plane_returns_conflict_status_for_unknown_provider_policy_versions() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    for (method, uri, permission, body) in [
        (
            "GET",
            "/backend/v3/api/control/provider_policies/diff?fromVersion=1&toVersion=9",
            "control.read",
            None,
        ),
        (
            "POST",
            "/backend/v3/api/control/provider_policies/rollback",
            "control.write",
            Some(r#"{"targetVersion":9}"#),
        ),
    ] {
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .with_dual_token_tenant("100001")
            .with_dual_token_user("1080")
            .with_dual_token_actor_kind("user")
            .with_dual_token_permission_scope(permission);
        if body.is_some() {
            request = request.header("content-type", "application/json");
        }

        let response = app
            .clone()
            .oneshot(
                request
                    .body(body.map(Body::from).unwrap_or_else(Body::empty))
                    .unwrap(),
            )
            .await
            .expect("unknown-version provider policy route should return a response");

        assert_eq!(response.status(), StatusCode::CONFLICT);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("unknown-version body should collect")
            .to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("unknown-version body should be valid json");
        assert_eq!(json["status"], 409);
        assert_eq!(json["errorStatus"], "conflict");
        assert_eq!(json["code"], "provider_policy_conflict");
        assert!(
            json["message"]
                .as_str()
                .expect("unknown-version message should be present")
                .contains("unknown provider policy version"),
            "unknown-version conflict should explain the missing policy version"
        );
    }
}

#[tokio::test]
async fn test_control_plane_rejects_provider_policy_diff_with_reversed_version_range() {
    let app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/provider_bindings")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment write should return a response");
    assert_eq!(deployment_write.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/provider_policies/diff?fromVersion=2&toVersion=1")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1080")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("reversed diff request should return a response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("reversed diff body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("reversed diff body should be valid json");
    assert_eq!(json["status"], 400);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "invalid_provider_policy");
    assert!(
        json["message"]
            .as_str()
            .expect("reversed diff error message should be present")
            .contains("fromVersion must not exceed toVersion"),
        "reversed diff should explain the invalid version range"
    );
}
