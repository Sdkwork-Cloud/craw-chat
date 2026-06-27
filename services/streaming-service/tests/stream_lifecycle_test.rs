use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryStreamStateStore;
use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_stream_checkpoint_and_complete_over_http() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_lifecycle",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let checkpoint_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_lifecycle/checkpoint")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("checkpoint request should succeed");
    assert_eq!(checkpoint_response.status(), StatusCode::OK);
    let checkpoint_body = checkpoint_response
        .into_body()
        .collect()
        .await
        .expect("checkpoint body should collect")
        .to_bytes();
    let checkpoint_json: serde_json::Value =
        serde_json::from_slice(&checkpoint_body).expect("checkpoint should be valid json");
    assert_eq!(checkpoint_json["state"], "checkpointed");
    assert_eq!(checkpoint_json["lastCheckpointSeq"], 3);

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_lifecycle/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_demo_5"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete should be valid json");
    assert_eq!(complete_json["state"], "completed");
    assert_eq!(complete_json["lastFrameSeq"], 5);
    assert_eq!(complete_json["resultMessageId"], "msg_demo_5");
}

#[tokio::test]
async fn test_stream_abort_over_http_closes_stream_without_result_message() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let abort_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort request should succeed");
    assert_eq!(abort_response.status(), StatusCode::OK);
    let abort_body = abort_response
        .into_body()
        .collect()
        .await
        .expect("abort body should collect")
        .to_bytes();
    let abort_json: serde_json::Value =
        serde_json::from_slice(&abort_body).expect("abort should be valid json");
    assert_eq!(abort_json["state"], "aborted");
    assert_eq!(abort_json["lastFrameSeq"], 2);
    assert_eq!(abort_json["resultMessageId"], serde_json::Value::Null);
    assert!(abort_json["closedAt"].is_string());

    let complete_after_abort = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3,
                        "resultMessageId": "msg_demo_3"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete after abort request should succeed");
    assert_eq!(complete_after_abort.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_stream_append_and_list_frames_over_http() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_frames",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_frames/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hel\"}",
                        "attributes": {
                            "topic": "llm"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);
    let append_body = append_response
        .into_body()
        .collect()
        .await
        .expect("append body should collect")
        .to_bytes();
    let append_json: serde_json::Value =
        serde_json::from_slice(&append_body).expect("append response should be valid json");
    assert_eq!(append_json["frameSeq"], 1);
    assert_eq!(append_json["frameType"], "delta");
    assert_eq!(append_json["sender"]["id"], "1");
    assert_eq!(append_json["attributes"]["topic"], "llm");

    let second_append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_frames/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"lo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append request should succeed");
    assert_eq!(second_append_response.status(), StatusCode::OK);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_frames/frames?afterFrameSeq=0&limit=10")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    assert_eq!(list_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(list_json["items"][0]["frameSeq"], 1);
    assert_eq!(list_json["items"][1]["frameSeq"], 2);
    assert_eq!(list_json["nextAfterFrameSeq"], 2);
    assert_eq!(list_json["hasMore"], false);
}

#[tokio::test]
async fn test_request_scoped_stream_append_rejects_different_actor_over_http() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_request_scope_owner_only_append",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_request_scope_owner_only_append/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1101")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"intrusion\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor append should return response");
    assert_eq!(append_response.status(), StatusCode::NOT_FOUND);
    let append_body = append_response
        .into_body()
        .collect()
        .await
        .expect("different actor append body should collect")
        .to_bytes();
    let append_json: serde_json::Value =
        serde_json::from_slice(&append_body).expect("different actor append should be valid json");
    assert_eq!(append_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_stream_runtime_timestamps_advance_between_distinct_mutations() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_timestamps",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);
    let open_body = open_response
        .into_body()
        .collect()
        .await
        .expect("open body should collect")
        .to_bytes();
    let open_json: serde_json::Value =
        serde_json::from_slice(&open_body).expect("open response should be valid json");
    let opened_at = open_json["openedAt"]
        .as_str()
        .expect("openedAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let first_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_timestamps/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first append request should succeed");
    assert_eq!(first_append.status(), StatusCode::OK);
    let first_append_body = first_append
        .into_body()
        .collect()
        .await
        .expect("first append body should collect")
        .to_bytes();
    let first_append_json: serde_json::Value =
        serde_json::from_slice(&first_append_body).expect("first append should be valid json");
    let first_occurred_at = first_append_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let second_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_timestamps/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"world\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append request should succeed");
    assert_eq!(second_append.status(), StatusCode::OK);
    let second_append_body = second_append
        .into_body()
        .collect()
        .await
        .expect("second append body should collect")
        .to_bytes();
    let second_append_json: serde_json::Value =
        serde_json::from_slice(&second_append_body).expect("second append should be valid json");
    let second_occurred_at = second_append_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_timestamps/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "resultMessageId": "msg_complete_timestamps"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete should be valid json");
    let closed_at = complete_json["closedAt"]
        .as_str()
        .expect("closedAt should be present")
        .to_owned();

    assert!(opened_at < first_occurred_at);
    assert!(first_occurred_at < second_occurred_at);
    assert!(second_occurred_at < closed_at);
}

#[tokio::test]
async fn test_stream_append_enforces_ordering_and_idempotent_retry_rules() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_rules",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rules/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append first frame request should succeed");
    assert_eq!(append_first.status(), StatusCode::OK);
    let append_first_body = append_first
        .into_body()
        .collect()
        .await
        .expect("append first body should collect")
        .to_bytes();
    let append_first_json: serde_json::Value =
        serde_json::from_slice(&append_first_body).expect("append first should be valid json");
    assert_eq!(append_first_json["frameSeq"], 1);
    assert_eq!(append_first_json["deliveryStatus"], "applied");
    assert_eq!(
        append_first_json["proofVersion"],
        "stream.frame.delivery-proof.v1"
    );

    let idempotent_retry = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rules/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent retry request should succeed");
    assert_eq!(idempotent_retry.status(), StatusCode::OK);
    let idempotent_retry_body = idempotent_retry
        .into_body()
        .collect()
        .await
        .expect("idempotent retry body should collect")
        .to_bytes();
    let idempotent_retry_json: serde_json::Value = serde_json::from_slice(&idempotent_retry_body)
        .expect("idempotent retry should be valid json");
    assert_eq!(idempotent_retry_json["frameSeq"], 1);
    assert_eq!(idempotent_retry_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_retry_json["requestKey"],
        append_first_json["requestKey"]
    );
    assert_eq!(
        idempotent_retry_json["proofVersion"],
        append_first_json["proofVersion"]
    );

    let out_of_order = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rules/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"skip\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("out of order request should return response");
    assert_eq!(out_of_order.status(), StatusCode::BAD_REQUEST);
    let out_of_order_body = out_of_order
        .into_body()
        .collect()
        .await
        .expect("out of order body should collect")
        .to_bytes();
    let out_of_order_json: serde_json::Value =
        serde_json::from_slice(&out_of_order_body).expect("out of order should be valid json");
    assert_eq!(out_of_order_json["code"], "stream_frame_out_of_order");

    let conflicting_retry = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rules/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"changed\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry request should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting retry body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting retry should be valid json");
    assert_eq!(conflicting_retry_json["code"], "stream_frame_conflict");
}

#[tokio::test]
async fn test_stream_append_rejects_closed_stream() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_closed",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_closed/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_closed_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let append_after_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_closed/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"late\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append after complete request should return response");
    assert_eq!(append_after_complete.status(), StatusCode::BAD_REQUEST);
    let append_after_complete_body = append_after_complete
        .into_body()
        .collect()
        .await
        .expect("append after complete body should collect")
        .to_bytes();
    let append_after_complete_json: serde_json::Value =
        serde_json::from_slice(&append_after_complete_body)
            .expect("append after complete should be valid json");
    assert_eq!(append_after_complete_json["code"], "stream_state_invalid");
}

#[tokio::test]
async fn test_duplicate_open_stream_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = streaming_service::build_default_app();

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream request should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert_eq!(
        first_open_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_open_idempotent/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);

    let idempotent_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent open stream request should succeed");
    assert_eq!(idempotent_open.status(), StatusCode::OK);
    let idempotent_open_body = idempotent_open
        .into_body()
        .collect()
        .await
        .expect("idempotent open body should collect")
        .to_bytes();
    let idempotent_open_json: serde_json::Value = serde_json::from_slice(&idempotent_open_body)
        .expect("idempotent open should be valid json");
    assert_eq!(idempotent_open_json["state"], "active");
    assert_eq!(idempotent_open_json["lastFrameSeq"], 1);
    assert_eq!(idempotent_open_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_open_json["requestKey"],
        first_open_json["requestKey"]
    );
    assert_eq!(
        idempotent_open_json["proofVersion"],
        first_open_json["proofVersion"]
    );

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_open_idempotent/frames?afterFrameSeq=0&limit=10")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    assert_eq!(list_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_json["items"][0]["frameSeq"], 1);

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.binary",
                        "scopeKind":"request",
                        "scopeId":"req_other",
                        "durabilityClass":"eventLog",
                        "schemaRef":"custom.delta.binary.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting open stream request should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("conflicting open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("conflicting open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_duplicate_open_stream_with_different_actor_is_conflict() {
    let app = streaming_service::build_default_app();

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert_eq!(
        first_open_json["requestKey"],
        "6#1000014#user1#14#open19#st_actor_scope_open"
    );

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1101")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor open should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("different actor open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("different actor open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_duplicate_complete_stream_request_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_complete_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_idempotent/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_idempotent/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_idempotent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value =
        serde_json::from_slice(&first_complete_body).expect("first complete should be valid json");
    assert_eq!(first_complete_json["state"], "completed");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let duplicate_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_idempotent/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_idempotent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate complete should return response");
    assert_eq!(duplicate_complete.status(), StatusCode::OK);
    let duplicate_complete_body = duplicate_complete
        .into_body()
        .collect()
        .await
        .expect("duplicate complete body should collect")
        .to_bytes();
    let duplicate_complete_json: serde_json::Value =
        serde_json::from_slice(&duplicate_complete_body)
            .expect("duplicate complete should be valid json");
    assert_eq!(duplicate_complete_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_complete_json["requestKey"],
        first_complete_json["requestKey"]
    );
    assert_eq!(
        duplicate_complete_json["proofVersion"],
        first_complete_json["proofVersion"]
    );

    let conflicting_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_idempotent/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting complete should return response");
    assert_eq!(conflicting_complete.status(), StatusCode::CONFLICT);
    let conflicting_complete_body = conflicting_complete
        .into_body()
        .collect()
        .await
        .expect("conflicting complete body should collect")
        .to_bytes();
    let conflicting_complete_json: serde_json::Value =
        serde_json::from_slice(&conflicting_complete_body)
            .expect("conflicting complete should be valid json");
    assert_eq!(conflicting_complete_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_duplicate_complete_stream_request_with_different_actor_is_not_found() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_actor_scope_complete",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_complete/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_complete/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_actor_scope_complete"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value =
        serde_json::from_slice(&first_complete_body).expect("first complete should be valid json");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["requestKey"],
        "6#1000014#user1#18#complete23#st_actor_scope_complete"
    );

    let hidden_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_complete/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1101")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_actor_scope_complete"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor complete should return response");
    assert_eq!(hidden_complete.status(), StatusCode::NOT_FOUND);
    let hidden_complete_body = hidden_complete
        .into_body()
        .collect()
        .await
        .expect("different actor complete body should collect")
        .to_bytes();
    let hidden_complete_json: serde_json::Value = serde_json::from_slice(&hidden_complete_body)
        .expect("different actor complete should be valid json");
    assert_eq!(hidden_complete_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_duplicate_abort_stream_request_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_idempotent/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_idempotent/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first abort should return response");
    assert_eq!(first_abort.status(), StatusCode::OK);
    let first_abort_body = first_abort
        .into_body()
        .collect()
        .await
        .expect("first abort body should collect")
        .to_bytes();
    let first_abort_json: serde_json::Value =
        serde_json::from_slice(&first_abort_body).expect("first abort should be valid json");
    assert_eq!(first_abort_json["state"], "aborted");
    assert_eq!(first_abort_json["deliveryStatus"], "applied");
    assert_eq!(
        first_abort_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );
    assert_eq!(first_abort_json["abortFrameSeq"], 1);
    assert_eq!(first_abort_json["abortReason"], "client_cancelled");

    let duplicate_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_idempotent/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate abort should return response");
    assert_eq!(duplicate_abort.status(), StatusCode::OK);
    let duplicate_abort_body = duplicate_abort
        .into_body()
        .collect()
        .await
        .expect("duplicate abort body should collect")
        .to_bytes();
    let duplicate_abort_json: serde_json::Value = serde_json::from_slice(&duplicate_abort_body)
        .expect("duplicate abort should be valid json");
    assert_eq!(duplicate_abort_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_abort_json["requestKey"],
        first_abort_json["requestKey"]
    );
    assert_eq!(
        duplicate_abort_json["proofVersion"],
        first_abort_json["proofVersion"]
    );
    assert_eq!(
        duplicate_abort_json["abortReason"],
        first_abort_json["abortReason"]
    );

    let conflicting_abort = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_idempotent/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "different_reason"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting abort should return response");
    assert_eq!(conflicting_abort.status(), StatusCode::CONFLICT);
    let conflicting_abort_body = conflicting_abort
        .into_body()
        .collect()
        .await
        .expect("conflicting abort body should collect")
        .to_bytes();
    let conflicting_abort_json: serde_json::Value = serde_json::from_slice(&conflicting_abort_body)
        .expect("conflicting abort should be valid json");
    assert_eq!(conflicting_abort_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_duplicate_abort_stream_request_with_different_actor_is_not_found() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_actor_scope_abort",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_abort/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_abort/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first abort should return response");
    assert_eq!(first_abort.status(), StatusCode::OK);
    let first_abort_body = first_abort
        .into_body()
        .collect()
        .await
        .expect("first abort body should collect")
        .to_bytes();
    let first_abort_json: serde_json::Value =
        serde_json::from_slice(&first_abort_body).expect("first abort should be valid json");
    assert_eq!(first_abort_json["deliveryStatus"], "applied");
    assert_eq!(
        first_abort_json["requestKey"],
        "6#1000014#user1#15#abort20#st_actor_scope_abort"
    );

    let hidden_abort = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_abort/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1101")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor abort should return response");
    assert_eq!(hidden_abort.status(), StatusCode::NOT_FOUND);
    let hidden_abort_body = hidden_abort
        .into_body()
        .collect()
        .await
        .expect("different actor abort body should collect")
        .to_bytes();
    let hidden_abort_json: serde_json::Value = serde_json::from_slice(&hidden_abort_body)
        .expect("different actor abort should be valid json");
    assert_eq!(hidden_abort_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_duplicate_checkpoint_stream_request_replays_after_stream_completes() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_checkpoint_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let first_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_checkpoint_idempotent/checkpoint")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first checkpoint should return response");
    assert_eq!(first_checkpoint.status(), StatusCode::OK);
    let first_checkpoint_body = first_checkpoint
        .into_body()
        .collect()
        .await
        .expect("first checkpoint body should collect")
        .to_bytes();
    let first_checkpoint_json: serde_json::Value = serde_json::from_slice(&first_checkpoint_body)
        .expect("first checkpoint should be valid json");
    assert_eq!(first_checkpoint_json["state"], "checkpointed");
    assert_eq!(first_checkpoint_json["deliveryStatus"], "applied");
    assert_eq!(
        first_checkpoint_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_checkpoint_idempotent/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_checkpoint_complete"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should return response");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let duplicate_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_checkpoint_idempotent/checkpoint")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate checkpoint should return response");
    assert_eq!(duplicate_checkpoint.status(), StatusCode::OK);
    let duplicate_checkpoint_body = duplicate_checkpoint
        .into_body()
        .collect()
        .await
        .expect("duplicate checkpoint body should collect")
        .to_bytes();
    let duplicate_checkpoint_json: serde_json::Value =
        serde_json::from_slice(&duplicate_checkpoint_body)
            .expect("duplicate checkpoint should be valid json");
    assert_eq!(duplicate_checkpoint_json["state"], "completed");
    assert_eq!(duplicate_checkpoint_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_checkpoint_json["requestKey"],
        first_checkpoint_json["requestKey"]
    );
    assert_eq!(
        duplicate_checkpoint_json["proofVersion"],
        first_checkpoint_json["proofVersion"]
    );
}

#[tokio::test]
async fn test_duplicate_checkpoint_stream_request_with_different_actor_is_not_found() {
    let app = streaming_service::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_actor_scope_checkpoint",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let first_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_checkpoint/checkpoint")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first checkpoint should return response");
    assert_eq!(first_checkpoint.status(), StatusCode::OK);
    let first_checkpoint_body = first_checkpoint
        .into_body()
        .collect()
        .await
        .expect("first checkpoint body should collect")
        .to_bytes();
    let first_checkpoint_json: serde_json::Value = serde_json::from_slice(&first_checkpoint_body)
        .expect("first checkpoint should be valid json");
    assert_eq!(first_checkpoint_json["deliveryStatus"], "applied");
    assert_eq!(
        first_checkpoint_json["requestKey"],
        "6#1000014#user1#110#checkpoint25#st_actor_scope_checkpoint1#3"
    );

    let hidden_checkpoint = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_actor_scope_checkpoint/checkpoint")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1101")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor checkpoint should return response");
    assert_eq!(hidden_checkpoint.status(), StatusCode::NOT_FOUND);
    let hidden_checkpoint_body = hidden_checkpoint
        .into_body()
        .collect()
        .await
        .expect("different actor checkpoint body should collect")
        .to_bytes();
    let hidden_checkpoint_json: serde_json::Value = serde_json::from_slice(&hidden_checkpoint_body)
        .expect("different actor checkpoint should be valid json");
    assert_eq!(hidden_checkpoint_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_runtime_restores_stream_state_on_rebuild_with_shared_store() {
    let stream_store = Arc::new(MemoryStreamStateStore::default());
    let app_before = streaming_service::build_app(Arc::new(
        streaming_service::StreamingRuntime::with_store(stream_store.clone()),
    ));

    let open_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_rebuild",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rebuild/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);

    let app_after = streaming_service::build_app(Arc::new(
        streaming_service::StreamingRuntime::with_store(stream_store),
    ));

    let list_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_rebuild/frames?afterFrameSeq=0&limit=10")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request after rebuild should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["frameSeq"], 1);

    let complete_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_rebuild/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "resultMessageId": "msg_rebuild_result"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream request after rebuild should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_stream_append_rejects_oversized_payload_over_http() {
    let app = streaming_service::build_default_app();
    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_oversized_payload",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let oversized_payload = "x".repeat(262145);
    let append_body = serde_json::json!({
        "frameSeq": 1,
        "frameType": "delta",
        "schemaRef": "custom.delta.text.v1",
        "encoding": "json",
        "payload": oversized_payload
    })
    .to_string();
    let append_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_oversized_payload/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(append_body))
                .unwrap(),
        )
        .await
        .expect("oversized append request should return response");
    assert_eq!(append_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_stream_append_rejects_oversized_attributes_over_http() {
    let app = streaming_service::build_default_app();
    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_oversized_attributes",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_body = serde_json::json!({
        "frameSeq": 1,
        "frameType": "delta",
        "schemaRef": "custom.delta.text.v1",
        "encoding": "json",
        "payload": "{\"delta\":\"hello\"}",
        "attributes": {
            "trace": "x".repeat(65537)
        }
    })
    .to_string();
    let append_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_oversized_attributes/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_demo")
                .header("content-type", "application/json")
                .body(Body::from(append_body))
                .unwrap(),
        )
        .await
        .expect("oversized attributes append request should return response");
    assert_eq!(append_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_stream_complete_rejects_oversized_result_message_id_over_http() {
    let app = streaming_service::build_default_app();
    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_oversized_result_message_id",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let complete_body = serde_json::json!({
        "frameSeq": 1,
        "resultMessageId": "m".repeat(257)
    })
    .to_string();
    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_oversized_result_message_id/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(complete_body))
                .unwrap(),
        )
        .await
        .expect("oversized complete request should return response");
    assert_eq!(complete_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("oversized complete rejection should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete rejection should be valid json");
    assert_eq!(complete_json["code"], "payload_too_large");
    assert!(
        complete_json["message"]
            .as_str()
            .expect("complete rejection message should be a string")
            .contains("resultMessageId"),
        "error should point to resultMessageId guard, got: {complete_json:?}"
    );
}

#[tokio::test]
async fn test_stream_abort_rejects_oversized_reason_over_http() {
    let app = streaming_service::build_default_app();
    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_oversized_abort_reason",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let abort_body = serde_json::json!({
        "frameSeq": 1,
        "reason": "x".repeat(8193)
    })
    .to_string();
    let abort_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_oversized_abort_reason/abort")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(abort_body))
                .unwrap(),
        )
        .await
        .expect("oversized abort request should return response");
    assert_eq!(abort_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let abort_body = abort_response
        .into_body()
        .collect()
        .await
        .expect("oversized abort rejection should collect")
        .to_bytes();
    let abort_json: serde_json::Value =
        serde_json::from_slice(&abort_body).expect("abort rejection should be valid json");
    assert_eq!(abort_json["code"], "payload_too_large");
    assert!(
        abort_json["message"]
            .as_str()
            .expect("abort rejection message should be a string")
            .contains("reason"),
        "error should point to reason guard, got: {abort_json:?}"
    );
}

#[tokio::test]
async fn test_stream_list_rejects_limit_above_guardrail_over_http() {
    let app = streaming_service::build_default_app();
    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_limit_guardrail",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_limit_guardrail/frames?afterFrameSeq=0&limit=1001")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("list rejection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("list rejection body should be valid json");
    assert_eq!(json["code"], "invalid_limit");
}

#[tokio::test]
async fn test_stream_list_rejects_oversized_stream_id_over_http() {
    let app = streaming_service::build_default_app();
    let oversized_stream_id = "s".repeat(257);

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/streams/{oversized_stream_id}/frames?afterFrameSeq=0&limit=10"
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized list request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("oversized list rejection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("oversized list rejection body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("oversized list rejection message should be a string")
            .contains("streamId"),
        "error should point to streamId guard, got: {json:?}"
    );
}
