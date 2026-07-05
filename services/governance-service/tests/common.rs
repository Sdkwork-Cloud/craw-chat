use axum::body::Body;
use axum::http::{request::Builder, Request};
use im_app_context::DualTokenRequestBuilderExt;

pub fn control_plane_request_builder(
    method: &str,
    uri: &str,
    user_id: &str,
    actor_kind: &str,
    permission: &str,
) -> Builder {
    Request::builder()
        .method(method)
        .uri(uri)
        .with_dual_token_context(
            "100001",
            user_id,
            actor_kind,
            Some("d_pad"),
            [permission],
        )
        .with_dual_token_organization("100001")
}

pub fn control_plane_write_request(
    method: &str,
    uri: &str,
    user_id: &str,
    actor_kind: &str,
) -> Builder {
    control_plane_request_builder(method, uri, user_id, actor_kind, "control.write")
}

pub fn control_plane_read_request(
    method: &str,
    uri: &str,
    user_id: &str,
    actor_kind: &str,
) -> Builder {
    control_plane_request_builder(method, uri, user_id, actor_kind, "control.read")
}

pub fn control_plane_json_body(body: &str) -> Body {
    Body::from(body.to_owned())
}
