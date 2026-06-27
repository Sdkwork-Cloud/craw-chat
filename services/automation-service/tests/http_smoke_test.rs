use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = automation_service::build_public_app();

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
    assert_eq!(value["info"]["title"], "Sdkwork IM Automation Service API");
    assert!(value["paths"]["/app/v3/api/automation/executions"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = automation_service::build_public_app();

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
    assert!(html.contains("Sdkwork IM Automation Service API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_request_and_get_execution_over_http() {
    let app = automation_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_demo",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request execution should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create body should be valid json");
    assert_eq!(create_json["executionId"], "ae_http_demo");
    assert_eq!(create_json["state"], "succeeded");
    assert_eq!(create_json["deliveryStatus"], "applied");
    assert!(
        !create_json["requestKey"]
            .as_str()
            .expect("request key should be present")
            .is_empty()
    );
    assert_eq!(
        create_json["proofVersion"],
        "automation.execution.delivery-proof.v1"
    );

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/automation/executions/ae_http_demo")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get execution should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get body should be valid json");
    assert_eq!(get_json["targetRef"], "wf_http_demo");
    assert_eq!(get_json["triggerType"], "webhook.manual");
}

#[tokio::test]
async fn test_duplicate_execution_id_is_idempotent_and_conflicting_retry_is_rejected_over_http() {
    let app = automation_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first execution request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");
    assert_eq!(first_json["deliveryStatus"], "applied");

    let idempotent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent retry should return response");
    assert_eq!(idempotent_response.status(), StatusCode::OK);
    let idempotent_body = idempotent_response
        .into_body()
        .collect()
        .await
        .expect("idempotent body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent body should be valid json");
    assert_eq!(idempotent_json["deliveryStatus"], "replayed");
    assert_eq!(idempotent_json["requestKey"], first_json["requestKey"]);
    assert_eq!(idempotent_json["executionId"], first_json["executionId"]);
    assert_eq!(idempotent_json["targetRef"], first_json["targetRef"]);
    assert_eq!(idempotent_json["triggerType"], first_json["triggerType"]);
    assert_eq!(idempotent_json["state"], first_json["state"]);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_other",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "automation_execution_conflict");
}

#[tokio::test]
async fn test_execution_requests_are_isolated_by_actor_kind_over_http() {
    let app = automation_service::build_default_app();

    let user_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_kind_isolation",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user execution request should return response");
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_body = user_response
        .into_body()
        .collect()
        .await
        .expect("user body should collect")
        .to_bytes();
    let user_json: serde_json::Value =
        serde_json::from_slice(&user_body).expect("user body should be valid json");
    assert_eq!(
        user_json["requestKey"],
        "6#1000014#user1#322#ae_http_kind_isolation"
    );
    assert_eq!(user_json["principalKind"], "user");

    let system_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_kind_isolation",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system execution request should return response");
    assert_eq!(system_response.status(), StatusCode::OK);
    let system_body = system_response
        .into_body()
        .collect()
        .await
        .expect("system body should collect")
        .to_bytes();
    let system_json: serde_json::Value =
        serde_json::from_slice(&system_body).expect("system body should be valid json");
    assert_eq!(
        system_json["requestKey"],
        "6#1000016#system1#322#ae_http_kind_isolation"
    );
    assert_eq!(system_json["principalKind"], "system");

    let user_get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/automation/executions/ae_http_kind_isolation")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user get should return response");
    assert_eq!(user_get_response.status(), StatusCode::OK);
    let user_get_body = user_get_response
        .into_body()
        .collect()
        .await
        .expect("user get body should collect")
        .to_bytes();
    let user_get_json: serde_json::Value =
        serde_json::from_slice(&user_get_body).expect("user get body should be valid json");
    assert_eq!(user_get_json["principalKind"], "user");

    let system_get_response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/automation/executions/ae_http_kind_isolation")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("system")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("system get should return response");
    assert_eq!(system_get_response.status(), StatusCode::OK);
    let system_get_body = system_get_response
        .into_body()
        .collect()
        .await
        .expect("system get body should collect")
        .to_bytes();
    let system_get_json: serde_json::Value =
        serde_json::from_slice(&system_get_body).expect("system get body should be valid json");
    assert_eq!(system_get_json["principalKind"], "system");
}

#[tokio::test]
async fn test_agent_response_and_tool_call_lifecycle_over_http() {
    let app = automation_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "streamId":"st_http_agent",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);
    let start_body = start_response
        .into_body()
        .collect()
        .await
        .expect("start body should collect")
        .to_bytes();
    let start_json: serde_json::Value =
        serde_json::from_slice(&start_body).expect("start body should be valid json");
    assert_eq!(start_json["streamId"], "st_http_agent");
    assert_eq!(start_json["state"], "opened");

    let delta_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_agent/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta.text",
                        "schemaRef":"schema://agent/response.delta#chunk",
                        "encoding":"json",
                        "payload":"{\"delta\":\"hello\"}",
                        "attributes":{"chunk":"1"}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response delta should return response");
    assert_eq!(delta_response.status(), StatusCode::OK);
    let delta_body = delta_response
        .into_body()
        .collect()
        .await
        .expect("delta body should collect")
        .to_bytes();
    let delta_json: serde_json::Value =
        serde_json::from_slice(&delta_body).expect("delta body should be valid json");
    assert_eq!(delta_json["sender"]["kind"], "agent");
    assert_eq!(delta_json["sender"]["id"], "ag_demo");

    let tool_request_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "toolCallId":"tc_http_lookup",
                        "toolName":"knowledge.search",
                        "argumentsPayload":"{\"query\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool request should return response");
    assert_eq!(tool_request_response.status(), StatusCode::OK);
    let tool_request_body = tool_request_response
        .into_body()
        .collect()
        .await
        .expect("tool request body should collect")
        .to_bytes();
    let tool_request_json: serde_json::Value =
        serde_json::from_slice(&tool_request_body).expect("tool request body should be valid json");
    assert_eq!(tool_request_json["state"], "requested");

    let tool_complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions/ae_http_agent/agent_tool_calls/tc_http_lookup/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "resultPayload":"{\"hits\":[{\"id\":\"doc_1\"}]}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool completion should return response");
    assert_eq!(tool_complete_response.status(), StatusCode::OK);
    let tool_complete_body = tool_complete_response
        .into_body()
        .collect()
        .await
        .expect("tool complete body should collect")
        .to_bytes();
    let tool_complete_json: serde_json::Value = serde_json::from_slice(&tool_complete_body)
        .expect("tool complete body should be valid json");
    assert_eq!(tool_complete_json["state"], "completed");

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_agent/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "resultMessageId":"m_http_agent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response complete should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete body should be valid json");
    assert_eq!(complete_json["state"], "completed");
    assert_eq!(complete_json["resultMessageId"], "m_http_agent");
}

#[tokio::test]
async fn test_automation_governance_surface_and_operator_override_over_http() {
    let app = automation_service::build_default_app();

    let governance_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/automation/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("governance request should return response");
    assert_eq!(governance_response.status(), StatusCode::OK);
    let governance_body = governance_response
        .into_body()
        .collect()
        .await
        .expect("governance body should collect")
        .to_bytes();
    let governance_json: serde_json::Value =
        serde_json::from_slice(&governance_body).expect("governance body should be valid json");
    assert_eq!(governance_json["capabilityProfileId"], "stable-agent");
    assert_eq!(
        governance_json["operatorOverridePermission"],
        "automation.operator_override"
    );
    assert_eq!(governance_json["operatorOverrideActive"], false);
    assert_eq!(governance_json["restrictedToolPrefixes"][0], "ops.");

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"shutdown\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "streamId":"st_http_guardrail",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let denied_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "toolCallId":"tc_http_guardrail_denied",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("restricted tool request should return response");
    assert_eq!(denied_response.status(), StatusCode::FORBIDDEN);
    let denied_body = denied_response
        .into_body()
        .collect()
        .await
        .expect("denied body should collect")
        .to_bytes();
    let denied_json: serde_json::Value =
        serde_json::from_slice(&denied_body).expect("denied body should be valid json");
    assert_eq!(denied_json["code"], "automation_guardrail_denied");

    let override_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope(
                    "automation.execute automation.read automation.operator_override",
                )
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "toolCallId":"tc_http_guardrail_allowed",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("override tool request should return response");
    assert_eq!(override_response.status(), StatusCode::OK);
    let override_body = override_response
        .into_body()
        .collect()
        .await
        .expect("override body should collect")
        .to_bytes();
    let override_json: serde_json::Value =
        serde_json::from_slice(&override_body).expect("override body should be valid json");
    assert_eq!(override_json["state"], "requested");

    let override_governance_response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/automation/governance")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.read automation.operator_override")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("override governance request should return response");
    assert_eq!(override_governance_response.status(), StatusCode::OK);
    let override_governance_body = override_governance_response
        .into_body()
        .collect()
        .await
        .expect("override governance body should collect")
        .to_bytes();
    let override_governance_json: serde_json::Value =
        serde_json::from_slice(&override_governance_body)
            .expect("override governance body should be valid json");
    assert_eq!(override_governance_json["operatorOverrideActive"], true);
}

#[tokio::test]
async fn test_request_execution_rejects_oversized_input_payload_over_http() {
    let app = automation_service::build_default_app();
    let oversized_input = "x".repeat(131073);
    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_input",
        "triggerType": "webhook.manual",
        "targetKind": "workflow",
        "targetRef": "wf_http_demo",
        "inputPayload": oversized_input,
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_request_execution_rejects_oversized_execution_id_over_http() {
    let app = automation_service::build_default_app();

    let oversized_execution_id = "e".repeat(257);
    let request_body = serde_json::json!({
        "executionId": oversized_execution_id,
        "triggerType":"webhook.manual",
        "targetKind":"workflow",
        "targetRef":"wf_demo",
        "inputPayload":"{\"conversationId\":\"c_demo\"}"
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized execution id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_get_execution_rejects_oversized_execution_id_over_http() {
    let app = automation_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/app/v3/api/automation/executions/{}",
                    "e".repeat(257)
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized execution id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_stream_id_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_stream_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_stream_id",
        "streamId": "s".repeat(257),
        "streamType": "agent.response.delta",
        "conversationId": "c_demo",
        "schemaRef": "schema://agent/response.delta",
        "memberId": "cm_agent",
        "agent": {
            "agent_id": "ag_demo",
            "session_id": "s_agent",
            "metadata": {
                "agentMode": "assistant",
                "capabilityProfileId": "stable-agent"
            }
        }
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized stream id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_stream_contract_fields_over_http() {
    let app = automation_service::build_default_app();
    let cases = [
        (
            "streamType",
            "ae_http_oversized_stream_type",
            serde_json::json!({
                "executionId": "ae_http_oversized_stream_type",
                "streamId": "st_http_oversized_stream_type",
                "streamType": "x".repeat(129),
                "conversationId": "c_demo",
                "schemaRef": "schema://agent/response.delta",
                "memberId": "cm_agent",
                "agent": {
                    "agent_id": "ag_demo",
                    "session_id": "s_agent",
                    "metadata": {
                        "agentMode": "assistant",
                        "capabilityProfileId": "stable-agent"
                    }
                }
            }),
        ),
        (
            "conversationId",
            "ae_http_oversized_conversation_id",
            serde_json::json!({
                "executionId": "ae_http_oversized_conversation_id",
                "streamId": "st_http_oversized_conversation_id",
                "streamType": "agent.response.delta",
                "conversationId": "c".repeat(257),
                "schemaRef": "schema://agent/response.delta",
                "memberId": "cm_agent",
                "agent": {
                    "agent_id": "ag_demo",
                    "session_id": "s_agent",
                    "metadata": {
                        "agentMode": "assistant",
                        "capabilityProfileId": "stable-agent"
                    }
                }
            }),
        ),
        (
            "schemaRef",
            "ae_http_oversized_schema_ref",
            serde_json::json!({
                "executionId": "ae_http_oversized_schema_ref",
                "streamId": "st_http_oversized_schema_ref",
                "streamType": "agent.response.delta",
                "conversationId": "c_demo",
                "schemaRef": "s".repeat(257),
                "memberId": "cm_agent",
                "agent": {
                    "agent_id": "ag_demo",
                    "session_id": "s_agent",
                    "metadata": {
                        "agentMode": "assistant",
                        "capabilityProfileId": "stable-agent"
                    }
                }
            }),
        ),
    ];

    for (field, execution_id, start_request_body) in cases {
        let create_execution_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/executions")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "executionId": execution_id,
                            "triggerType":"agent.manual",
                            "targetKind":"conversation",
                            "targetRef":"c_demo",
                            "inputPayload":"{\"prompt\":\"hello\"}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("execution request should return response");
        assert_eq!(create_execution_response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/agent_responses")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(start_request_body.to_string()))
                    .unwrap(),
            )
            .await
            .expect("oversized contract field request should return response");
        assert_eq!(
            response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_member_id_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_member_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_member_id",
        "streamId": "st_http_oversized_member_id",
        "streamType": "agent.response.delta",
        "conversationId": "c_demo",
        "schemaRef": "schema://agent/response.delta",
        "memberId": "m".repeat(257),
        "agent": {
            "agent_id": "ag_demo",
            "session_id": "s_agent",
            "metadata": {
                "agentMode": "assistant",
                "capabilityProfileId": "stable-agent"
            }
        }
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized member id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_execution_id_over_http() {
    let app = automation_service::build_default_app();

    let request_body = serde_json::json!({
        "executionId": "e".repeat(257),
        "streamId": "st_http_oversized_start_execution_id",
        "streamType": "agent.response.delta",
        "conversationId": "c_demo",
        "schemaRef": "schema://agent/response.delta",
        "memberId": "cm_agent",
        "agent": {
            "agent_id": "ag_demo",
            "session_id": "s_agent",
            "metadata": {
                "agentMode": "assistant",
                "capabilityProfileId": "stable-agent"
            }
        }
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized execution id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_append_agent_response_delta_rejects_oversized_stream_id_path_over_http() {
    let app = automation_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/app/v3/api/automation/agent_responses/{}/frames",
                    "s".repeat(257)
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "frameType": "delta.text",
                        "schemaRef": "schema://agent/response.delta#chunk",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}",
                        "attributes": {}
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized stream path request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_agent_metadata_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_agent_metadata",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_agent_metadata",
        "streamId": "st_http_oversized_agent_metadata",
        "streamType": "agent.response.delta",
        "conversationId": "c_demo",
        "schemaRef": "schema://agent/response.delta",
        "memberId": "cm_agent",
        "agent": {
            "agent_id": "ag_demo",
            "session_id": "s_agent",
            "metadata": {
                "trace": "x".repeat(65_537)
            }
        }
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized agent metadata request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_complete_agent_response_rejects_oversized_result_message_id_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_result_message_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_result_message_id",
                        "streamId":"st_http_oversized_result_message_id",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "frameSeq": 1,
        "resultMessageId": "m".repeat(257)
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_oversized_result_message_id/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized result message id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_complete_agent_response_rejects_oversized_stream_id_path_over_http() {
    let app = automation_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/app/v3/api/automation/agent_responses/{}/complete",
                    "s".repeat(257)
                ))
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "resultMessageId": "m_done"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized stream path request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_start_agent_response_rejects_oversized_agent_identity_fields_over_http() {
    for (field, agent_id, session_id) in [
        (
            "agent.agent_id",
            serde_json::Value::String("a".repeat(257)),
            serde_json::Value::String("s_agent".into()),
        ),
        (
            "agent.session_id",
            serde_json::Value::String("ag_demo".into()),
            serde_json::Value::String("s".repeat(257)),
        ),
    ] {
        let app = automation_service::build_default_app();

        let create_execution_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/executions")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "executionId": format!("ae_http_{}", field.replace('.', "_")),
                            "triggerType":"agent.manual",
                            "targetKind":"conversation",
                            "targetRef":"c_demo",
                            "inputPayload":"{\"prompt\":\"hello\"}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("execution request should return response");
        assert_eq!(create_execution_response.status(), StatusCode::OK);

        let request_body = serde_json::json!({
            "executionId": format!("ae_http_{}", field.replace('.', "_")),
            "streamId": format!("st_http_{}", field.replace('.', "_")),
            "streamType": "agent.response.delta",
            "conversationId": "c_demo",
            "schemaRef": "schema://agent/response.delta",
            "memberId": "cm_agent",
            "agent": {
                "agent_id": agent_id,
                "session_id": session_id,
                "metadata": {
                    "agentMode": "assistant",
                    "capabilityProfileId": "stable-agent"
                }
            }
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/agent_responses")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body))
                    .unwrap(),
            )
            .await
            .expect("oversized agent identity request should return response");
        assert_eq!(
            response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_append_agent_response_delta_rejects_oversized_payload_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_delta",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_delta",
                        "streamId":"st_http_oversized_delta",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let oversized_payload = "x".repeat(262145);
    let delta_request_body = serde_json::json!({
        "frameSeq": 1,
        "frameType": "delta.text",
        "schemaRef": "schema://agent/response.delta#chunk",
        "encoding": "json",
        "payload": oversized_payload,
        "attributes": {"chunk": "1"}
    })
    .to_string();
    let oversized_delta_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_oversized_delta/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(delta_request_body))
                .unwrap(),
        )
        .await
        .expect("oversized delta request should return response");
    assert_eq!(
        oversized_delta_response.status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[tokio::test]
async fn test_append_agent_response_delta_rejects_oversized_contract_fields_over_http() {
    let app = automation_service::build_default_app();
    let cases = [
        (
            "frameType",
            serde_json::json!({
                "frameSeq": 1,
                "frameType": "f".repeat(65),
                "schemaRef": "schema://agent/response.delta#chunk",
                "encoding": "json",
                "payload": "{\"delta\":\"hello\"}",
                "attributes": {}
            }),
        ),
        (
            "encoding",
            serde_json::json!({
                "frameSeq": 2,
                "frameType": "delta.text",
                "schemaRef": "schema://agent/response.delta#chunk",
                "encoding": "j".repeat(33),
                "payload": "{\"delta\":\"hello\"}",
                "attributes": {}
            }),
        ),
        (
            "schemaRef",
            serde_json::json!({
                "frameSeq": 3,
                "frameType": "delta.text",
                "schemaRef": "s".repeat(257),
                "encoding": "json",
                "payload": "{\"delta\":\"hello\"}",
                "attributes": {}
            }),
        ),
    ];

    for (field, delta_request_body) in cases {
        let create_execution_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/executions")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "executionId":"ae_http_oversized_delta_contract",
                            "triggerType":"agent.manual",
                            "targetKind":"conversation",
                            "targetRef":"c_demo",
                            "inputPayload":"{\"prompt\":\"hello\"}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("execution request should return response");
        assert_eq!(create_execution_response.status(), StatusCode::OK);

        let start_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/agent_responses")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{
                            "executionId":"ae_http_oversized_delta_contract",
                            "streamId":"st_http_oversized_delta_contract",
                            "streamType":"agent.response.delta",
                            "conversationId":"c_demo",
                            "schemaRef":"schema://agent/response.delta",
                            "memberId":"cm_agent",
                            "agent":{
                                "agent_id":"ag_demo",
                                "session_id":"s_agent",
                                "metadata":{
                                    "agentMode":"assistant",
                                    "capabilityProfileId":"stable-agent"
                                }
                            }
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .expect("agent response start should return response");
        assert_eq!(start_response.status(), StatusCode::OK);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/app/v3/api/automation/agent_responses/st_http_oversized_delta_contract/frames")
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(delta_request_body.to_string()))
                    .unwrap(),
            )
            .await
            .expect("oversized delta contract field request should return response");
        assert_eq!(
            response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_request_agent_tool_call_rejects_oversized_tool_call_id_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_tool_call_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_tool_call_id",
                        "streamId":"st_http_oversized_tool_call_id",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_tool_call_id",
        "toolCallId": "t".repeat(257),
        "toolName": "knowledge.search",
        "argumentsPayload": "{\"query\":\"hello\"}"
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized tool call id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_request_agent_tool_call_rejects_oversized_execution_id_over_http() {
    let app = automation_service::build_default_app();

    let request_body = serde_json::json!({
        "executionId": "e".repeat(257),
        "toolCallId": "tc_http_oversized_execution_id",
        "toolName": "knowledge.search",
        "argumentsPayload": "{\"query\":\"hello\"}"
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized execution id request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_complete_agent_tool_call_rejects_oversized_path_ids_over_http() {
    let app = automation_service::build_default_app();

    for (field, execution_id, tool_call_id) in [
        ("executionId", "e".repeat(257), "tc_http_demo".to_string()),
        ("toolCallId", "ae_http_demo".to_string(), "t".repeat(257)),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/app/v3/api/automation/executions/{}/agent_tool_calls/{}/complete",
                        execution_id, tool_call_id
                    ))
                    .with_dual_token_tenant("100001")
                    .with_dual_token_user("1")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_permission_scope("automation.execute automation.read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "resultPayload": "{\"hits\":[{\"id\":\"doc_1\"}]}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("oversized path id request should return response");
        assert_eq!(
            response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_request_agent_tool_call_rejects_oversized_tool_name_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_tool_name",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_tool_name",
                        "streamId":"st_http_oversized_tool_name",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let request_body = serde_json::json!({
        "executionId": "ae_http_oversized_tool_name",
        "toolCallId": "tc_http_oversized_name",
        "toolName": "t".repeat(257),
        "argumentsPayload": "{\"query\":\"hello\"}"
    })
    .to_string();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized tool name request should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_append_agent_response_delta_rejects_oversized_attributes_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_delta_attrs",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_oversized_delta_attrs",
                        "streamId":"st_http_oversized_delta_attrs",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let oversized_attributes = serde_json::json!({
        "trace": "x".repeat(65537)
    });
    let delta_request_body = serde_json::json!({
        "frameSeq": 1,
        "frameType": "delta.text",
        "schemaRef": "schema://agent/response.delta#chunk",
        "encoding": "json",
        "payload": "{\"delta\":\"hello\"}",
        "attributes": oversized_attributes
    })
    .to_string();
    let oversized_delta_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_oversized_delta_attrs/frames")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(delta_request_body))
                .unwrap(),
        )
        .await
        .expect("oversized delta attributes request should return response");
    assert_eq!(
        oversized_delta_response.status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[tokio::test]
async fn test_request_agent_tool_call_rejects_after_agent_response_completed_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_tool_after_complete",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_tool_after_complete",
                        "streamId":"st_http_tool_after_complete",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_tool_after_complete/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":0,
                        "resultMessageId":"m_done"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response complete should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let tool_call_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_tool_after_complete",
                        "toolCallId":"tc_http_after_complete",
                        "toolName":"knowledge.search",
                        "argumentsPayload":"{\"query\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool call request should return response");
    assert_eq!(tool_call_response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_complete_agent_response_rejects_when_tool_call_pending_over_http() {
    let app = automation_service::build_default_app();

    let create_execution_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/executions")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_pending_tool_guard",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should return response");
    assert_eq!(create_execution_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_pending_tool_guard",
                        "streamId":"st_http_pending_tool_guard",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let request_tool_call_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_tool_calls")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_pending_tool_guard",
                        "toolCallId":"tc_http_pending",
                        "toolName":"knowledge.search",
                        "argumentsPayload":"{\"query\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool call request should return response");
    assert_eq!(request_tool_call_response.status(), StatusCode::OK);

    let blocked_complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_pending_tool_guard/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":0,
                        "resultMessageId":"m_done"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("pending tool call completion should return response");
    assert_eq!(blocked_complete_response.status(), StatusCode::BAD_REQUEST);
    let blocked_complete_body = blocked_complete_response
        .into_body()
        .collect()
        .await
        .expect("blocked completion body should collect")
        .to_bytes();
    let blocked_complete_json: serde_json::Value = serde_json::from_slice(&blocked_complete_body)
        .expect("blocked completion body should be valid json");
    assert_eq!(
        blocked_complete_json["code"],
        "agent_response_pending_tool_calls"
    );

    let complete_tool_call_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/app/v3/api/automation/executions/ae_http_pending_tool_guard/agent_tool_calls/tc_http_pending/complete",
                )
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "resultPayload":"{\"hits\":[{\"id\":\"doc_1\"}]}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool call completion should return response");
    assert_eq!(complete_tool_call_response.status(), StatusCode::OK);

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/app/v3/api/automation/agent_responses/st_http_pending_tool_guard/complete")
                .with_dual_token_tenant("100001")
                .with_dual_token_user("1")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":0,
                        "resultMessageId":"m_done"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response completion should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);
}
