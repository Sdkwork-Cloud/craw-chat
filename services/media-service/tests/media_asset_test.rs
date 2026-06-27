use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_media_service_does_not_expose_app_local_upload_lifecycle_routes() {
    let app = media_service::build_default_app();

    for (method, path) in [
        ("POST", "/im/v3/api/media/uploads"),
        ("POST", "/im/v3/api/media/uploads/ma_legacy/complete"),
        ("GET", "/im/v3/api/media/ma_legacy"),
        ("GET", "/im/v3/api/media/ma_legacy/download_url"),
        ("POST", "/im/v3/api/media/ma_legacy/attach"),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should return response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must be removed; Drive owns file upload/download lifecycle"
        );
    }
}
