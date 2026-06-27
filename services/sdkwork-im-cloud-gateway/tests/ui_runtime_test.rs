use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sdkwork_api_config::StandaloneConfig;
use sdkwork_api_product_runtime::{
    ProductSiteDirs, RouterProductRuntimeOptions, build_product_runtime_router,
};
use sdkwork_im_cloud_gateway_config::{GatewayRuntimeMode, WebGatewayConfig};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceExt;

#[tokio::test]
async fn unified_gateway_fallback_serves_portal_and_admin_shells() {
    let temp_root = unique_temp_root("gateway_ui_runtime");
    let admin_site_dir = temp_root.join("admin");
    let portal_site_dir = temp_root.join("portal");
    fs::create_dir_all(admin_site_dir.join("assets")).expect("admin assets dir should be created");
    fs::create_dir_all(portal_site_dir.join("assets"))
        .expect("portal assets dir should be created");
    write_file(
        admin_site_dir.join("index.html").as_path(),
        "<!doctype html><html><head><title>admin-shell</title></head><body>admin-shell</body></html>",
    );
    write_file(
        portal_site_dir.join("index.html").as_path(),
        "<!doctype html><html><head><script type=\"importmap\">{}</script><title>portal-shell</title></head><body>portal-shell</body></html>",
    );

    let product_router = build_product_runtime_router(
        StandaloneConfig {
            runtime_bind_addr: "127.0.0.1:0".into(),
            admin_proxy_target: String::new(),
            portal_api_base_url: "http://127.0.0.1:18079".into(),
            admin_sandbox_enabled: false,
            admin_sandbox_storage_file: None,
        },
        RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
            admin_site_dir.clone(),
            portal_site_dir.clone(),
        )),
    )
    .await
    .expect("product runtime router should build");

    let app = web_gateway::build_app_with_registry_and_product_runtime(
        test_gateway_config(),
        web_gateway::build_gateway_registry().expect("gateway route registry should build"),
        Some(product_router),
    );

    let portal_response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .expect("portal request should succeed");
    assert_eq!(portal_response.status(), StatusCode::OK);
    let portal_body = String::from_utf8(
        portal_response
            .into_body()
            .collect()
            .await
            .expect("portal body should collect")
            .to_bytes()
            .to_vec(),
    )
    .expect("portal body should be utf8");
    assert!(portal_body.contains("portal-shell"));
    assert!(portal_body.contains("__SDKWORK_IM_PORTAL_API_BASE_URL__"));
    assert!(portal_body.contains(r#"type="importmap" nonce=""#));

    let admin_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/admin/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("admin request should succeed");
    assert_eq!(admin_response.status(), StatusCode::OK);
    let admin_body = String::from_utf8(
        admin_response
            .into_body()
            .collect()
            .await
            .expect("admin body should collect")
            .to_bytes()
            .to_vec(),
    )
    .expect("admin body should be utf8");
    assert!(admin_body.contains("admin-shell"));

    let missing_api_response = app
        .oneshot(
            Request::builder()
                .uri("/api/missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("missing api request should succeed");
    assert_eq!(missing_api_response.status(), StatusCode::NOT_FOUND);
}

fn unique_temp_root(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_web_gateway_{prefix}_{unique}"))
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir should be created");
    }

    fs::write(path, contents).expect("test file should be written");
}

fn test_gateway_config() -> WebGatewayConfig {
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::Split,
        strict_startup: true,
        upstreams: Vec::new(),
    }
}
