use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = media_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Sdkwork IM Media Reference API");
    let paths = value["paths"]
        .as_object()
        .expect("openapi paths should be an object");
    for forbidden in [
        "/im/v3/api/media/uploads",
        "/im/v3/api/media/uploads/{mediaReferenceId}/complete",
        "/im/v3/api/media/{mediaReferenceId}",
        "/im/v3/api/media/{mediaReferenceId}/download_url",
    ] {
        assert!(
            !paths.contains_key(forbidden),
            "media-service OpenAPI must not expose app-local storage lifecycle path {forbidden}"
        );
    }
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = media_service::build_public_app();

    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let html = String::from_utf8(body.to_vec()).expect("docs should be valid utf-8");

    assert!(html.contains("OpenAPI 3.1"));
    assert!(html.contains("Sdkwork IM Media Reference API"));
    assert!(html.contains("/openapi.json"));
}
