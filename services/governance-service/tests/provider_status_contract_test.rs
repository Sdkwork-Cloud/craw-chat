use im_app_context::DualTokenRequestBuilderExt;
use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{RuntimeProviderRegistry, StaticProviderRegistry};
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

struct StatusExpectation<'a> {
    method: &'a str,
    uri: &'a str,
    tenant_id: Option<&'a str>,
    user_id: Option<&'a str>,
    permission: Option<&'a str>,
    body: Option<&'a str>,
    expected_http: StatusCode,
    expected_business_status: &'a str,
}

async fn request_status(
    app: Router,
    expectation: &StatusExpectation<'_>,
) -> (StatusCode, Option<String>, Option<String>) {
    let mut request = Request::builder()
        .method(expectation.method)
        .uri(expectation.uri);
    if let (Some(tenant_id), Some(user_id)) = (expectation.tenant_id, expectation.user_id) {
        request = request.with_dual_token_context(
            tenant_id,
            user_id,
            "user",
            None,
            expectation.permission.into_iter(),
        );
    } else if let Some(tenant_id) = expectation.tenant_id {
        request = request.with_dual_token_tenant(tenant_id);
    } else if let Some(user_id) = expectation.user_id {
        request = request.with_dual_token_user(user_id);
        request = request.with_dual_token_actor_kind("user");
    }
    if expectation.body.is_some() {
        request = request.header("content-type", "application/json");
    }

    let response = app
        .oneshot(
            request
                .body(
                    expectation
                        .body
                        .map(|value| Body::from(value.to_owned()))
                        .unwrap_or_else(Body::empty),
                )
                .unwrap(),
        )
        .await
        .expect("provider control-plane request should return a response");
    let status_code = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider control-plane body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider control-plane body should be valid json");
    let status = if status_code.is_success() {
        json["status"].as_str().map(str::to_owned)
    } else {
        None
    };
    let error_status = json["errorStatus"].as_str().map(str::to_owned);

    (status_code, status, error_status)
}

async fn assert_status(app: Router, expectation: StatusExpectation<'_>) -> String {
    let (status_code, status, error_status) = request_status(app, &expectation).await;
    assert_eq!(
        status_code, expectation.expected_http,
        "{} {} should return the expected HTTP status",
        expectation.method, expectation.uri
    );
    if expectation.expected_http.is_success() {
        let business_status = status.expect("success response should expose top-level status");
        assert_eq!(
            business_status, expectation.expected_business_status,
            "{} {} should return the expected top-level provider status",
            expectation.method, expectation.uri
        );
        business_status
    } else {
        let legacy_error_status = error_status.expect("error response should expose errorStatus");
        assert_eq!(
            legacy_error_status, expectation.expected_business_status,
            "{} {} should return the expected error status mapping",
            expectation.method, expectation.uri
        );
        legacy_error_status
    }
}

#[tokio::test]
async fn test_provider_control_plane_status_contract_covers_read_write_and_error_routes() {
    let runtime_app = governance_service::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );
    let static_app = governance_service::build_app_with_cluster_and_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(StaticProviderRegistry::platform_default()),
    );

    let mut observed = BTreeSet::new();

    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_registry",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_business_status: "registry",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_bindings",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_business_status: "bindings",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_policies/preview",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::OK,
                expected_business_status: "preview",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_bindings",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::OK,
                expected_business_status: "applied",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_bindings",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine","expectedBaseVersion":2}"#,
                ),
                expected_http: StatusCode::OK,
                expected_business_status: "noop",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_policies",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_business_status: "history",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_policies/diff?fromVersion=1&toVersion=2",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_business_status: "diff",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_policies/rollback",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(r#"{"targetVersion":1}"#),
                expected_http: StatusCode::OK,
                expected_business_status: "rolled_back",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_bindings",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"rtc","pluginId":"object-storage-aws"}"#),
                expected_http: StatusCode::BAD_REQUEST,
                expected_business_status: "invalid",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_policies/diff?fromVersion=1&toVersion=9",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::CONFLICT,
                expected_business_status: "conflict",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            static_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/backend/v3/api/control/provider_policies/preview",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::SERVICE_UNAVAILABLE,
                expected_business_status: "unavailable",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_registry",
                tenant_id: Some("100001"),
                user_id: Some("1080"),
                permission: None,
                body: None,
                expected_http: StatusCode::FORBIDDEN,
                expected_business_status: "forbidden",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app,
            StatusExpectation {
                method: "GET",
                uri: "/backend/v3/api/control/provider_registry",
                tenant_id: None,
                user_id: None,
                permission: None,
                body: None,
                expected_http: StatusCode::UNAUTHORIZED,
                expected_business_status: "unauthorized",
            },
        )
        .await,
    );

    assert_eq!(
        observed,
        BTreeSet::from([
            "applied".to_owned(),
            "bindings".to_owned(),
            "conflict".to_owned(),
            "diff".to_owned(),
            "forbidden".to_owned(),
            "history".to_owned(),
            "invalid".to_owned(),
            "noop".to_owned(),
            "preview".to_owned(),
            "registry".to_owned(),
            "rolled_back".to_owned(),
            "unavailable".to_owned(),
            "unauthorized".to_owned(),
        ]),
        "provider control-plane routes should expose the consolidated top-level status vocabulary"
    );
}
