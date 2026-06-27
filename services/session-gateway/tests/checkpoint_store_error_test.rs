use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;

mod test_env;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use tower::ServiceExt;

#[derive(Clone)]
struct FailingCheckpointStore {
    fail_on_load: bool,
    fail_on_save: bool,
}

impl RealtimeCheckpointStore for FailingCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        if self.fail_on_load {
            return Err(ContractError::Unavailable(
                "synthetic checkpoint load failure".into(),
            ));
        }
        Ok(None)
    }

    fn save_checkpoints(
        &self,
        _records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.fail_on_save {
            return Err(ContractError::Unavailable(
                "synthetic checkpoint save failure".into(),
            ));
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_realtime_events_returns_503_when_checkpoint_store_load_fails() {
    let _env = test_env::dev_test_environment();
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_and_runtime(
        cluster.clone(),
        Arc::new(
            session_gateway::RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
                FailingCheckpointStore {
                    fail_on_load: true,
                    fail_on_save: false,
                },
            )),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_pad")
                .with_dual_token_session("s_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should return a response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["code"], "checkpoint_store_unavailable");
    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .is_none(),
        "failed realtime events request must not leave a new client route"
    );
}

#[tokio::test]
async fn test_realtime_ack_returns_503_when_checkpoint_store_save_fails() {
    let _env = test_env::dev_test_environment();
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_and_runtime(
        cluster.clone(),
        Arc::new(
            session_gateway::RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
                FailingCheckpointStore {
                    fail_on_load: false,
                    fail_on_save: true,
                },
            )),
        ),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_pad")
                .with_dual_token_session("s_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("request should return a response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["code"], "checkpoint_store_unavailable");
    assert!(
        cluster
            .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
            .is_none(),
        "failed realtime ack request must not leave a new client route"
    );
}

#[tokio::test]
async fn test_realtime_ack_preserves_existing_route_when_checkpoint_store_save_fails() {
    let _env = test_env::dev_test_environment();
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let runtime = Arc::new(
        session_gateway::RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
            FailingCheckpointStore {
                fail_on_load: false,
                fail_on_save: true,
            },
        )),
    );
    let app = session_gateway::build_app_with_cluster_and_runtime(cluster.clone(), runtime);

    cluster
        .bind_client_route_for_principal_kind(            "100001",            "default",            "1",
            "user",
            "d_pad",
            "session_gateway_local_1",
            Some("s_demo"),
            "websocket",
        )
        .expect("test setup should bind an existing websocket route");
    let existing_route = cluster
        .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("test setup should expose the existing route");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_pad")
                .with_dual_token_session("s_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("request should return a response");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let current_route = cluster
        .resolve_client_route_for_principal_kind("100001", "default", "1", "user", "d_pad")
        .expect("failed ack must preserve the route that existed before the request");
    assert_eq!(current_route.owner_node_id, existing_route.owner_node_id);
    assert_eq!(current_route.session_id, existing_route.session_id);
    assert_eq!(
        current_route.connection_kind,
        existing_route.connection_kind
    );
    assert!(
        current_route.route_epoch > existing_route.route_epoch,
        "restoring a previous route after a failed ack must still advance the route epoch"
    );
}
