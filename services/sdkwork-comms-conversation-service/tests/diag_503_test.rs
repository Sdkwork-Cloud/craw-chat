use im_app_context::DualTokenRequestBuilderExt;
use conversation_runtime;
use tower::ServiceExt;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;

#[tokio::test]
async fn diag_503_actual_error() {
    let app = conversation_runtime::build_default_app();

    // Create conversation
    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"conversationId":"c_diag_503","conversationType":"group"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    println!("CREATE status: {}", create_resp.status());
    let create_body = create_resp.into_body().collect().await.unwrap().to_bytes();
    println!("CREATE body: {}", String::from_utf8_lossy(&create_body));

    // Post message
    let post_resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_diag_503/messages")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"clientMsgId":"client_diag","summary":"hello","text":"hello"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    println!("POST status: {}", post_resp.status());
    let post_body = post_resp.into_body().collect().await.unwrap().to_bytes();
    println!("POST body: {}", String::from_utf8_lossy(&post_body));

    let _ = StatusCode::OK;
}
