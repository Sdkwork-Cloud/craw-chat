use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use social_service::SocialRuntime;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_social_infra_app_exposes_shared_channel_sync_metrics() {
    let app = social_service::build_app(Arc::new(SocialRuntime::default()));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("metrics request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("metrics body should collect")
        .to_bytes();
    let text = String::from_utf8(body.to_vec()).expect("metrics should be utf-8");
    assert!(text.contains("im_shared_channel_sync_stale_reclaim_ticks_total"));
    assert!(text.contains("im_shared_channel_sync_delivery_deduplicated_total"));
    assert!(text.contains("im_health_status"));
}
