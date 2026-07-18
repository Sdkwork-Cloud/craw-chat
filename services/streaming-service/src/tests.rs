use std::collections::BTreeSet;

use im_app_context::AppContext;

use crate::dto::{AppendStreamFrameRequest, OpenStreamRequest};
use crate::helpers::{stream_append_request_key, stream_open_request_key};
use crate::state::StreamingRuntime;

fn auth(organization_id: &str) -> AppContext {
    AppContext {
        tenant_id: "tenant-a".into(),
        organization_id: organization_id.into(),
        user_id: "user-a".into(),
        session_id: Some("session-a".into()),
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: BTreeSet::new(),
        permission_scope: BTreeSet::new(),
        actor_id: "user-a".into(),
        actor_kind: "user".into(),
        device_id: Some("device-a".into()),
    }
}

fn open_request(stream_id: &str) -> OpenStreamRequest {
    OpenStreamRequest {
        stream_id: stream_id.into(),
        stream_type: "custom.delta.text".into(),
        scope_kind: "request".into(),
        scope_id: "request-a".into(),
        durability_class: "durableSession".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
    }
}

fn append_request(frame_seq: u64, payload: &str) -> AppendStreamFrameRequest {
    AppendStreamFrameRequest {
        frame_seq,
        frame_type: "delta".into(),
        schema_ref: Some("custom.delta.text.v1".into()),
        encoding: "json".into(),
        payload: payload.into(),
        attributes: Default::default(),
    }
}

#[test]
fn stream_runtime_isolates_organizations_within_one_tenant() {
    let runtime = StreamingRuntime::default();
    let org_a = auth("org-a");
    let org_b = auth("org-b");

    runtime
        .open_stream(&org_a, open_request("stream-shared"))
        .expect("organization A stream should open");
    runtime
        .open_stream(&org_b, open_request("stream-shared"))
        .expect("organization B stream should open independently");
    runtime
        .append_frame(&org_a, "stream-shared", append_request(1, "a"))
        .expect("organization A frame should append");

    assert_eq!(
        runtime
            .list_frames(&org_a, "stream-shared", 0, 20)
            .expect("organization A page")
            .items
            .len(),
        1
    );
    assert!(
        runtime
            .list_frames(&org_b, "stream-shared", 0, 20)
            .expect("organization B page")
            .items
            .is_empty()
    );
}

#[test]
fn duplicate_frame_sequence_rejects_changed_payload() {
    let runtime = StreamingRuntime::default();
    let auth = auth("org-a");
    runtime
        .open_stream(&auth, open_request("stream-conflict"))
        .expect("stream should open");
    runtime
        .append_frame(&auth, "stream-conflict", append_request(1, "first"))
        .expect("first frame should append");

    let replay = runtime
        .append_frame_with_outcome(&auth, "stream-conflict", append_request(1, "first"))
        .expect("same payload should replay");
    assert!(!replay.applied);

    let conflict = runtime.append_frame(&auth, "stream-conflict", append_request(1, "different"));
    assert_eq!(
        conflict
            .expect_err("changed payload must conflict")
            .status
            .as_u16(),
        409
    );
}

#[test]
fn stream_request_keys_include_organization_scope() {
    let org_a = auth("org-a");
    let org_b = auth("org-b");
    assert_ne!(
        stream_open_request_key(&org_a, "stream"),
        stream_open_request_key(&org_b, "stream")
    );
    assert_ne!(
        stream_append_request_key(&org_a, "stream", 7),
        stream_append_request_key(&org_b, "stream", 7)
    );
}
