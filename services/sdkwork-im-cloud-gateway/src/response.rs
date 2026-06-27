//! Response shaping, header allow-listing, HTTP method mapping, gateway proxy
//! route wiring, and JSON error rendering shared across gateway handlers.

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, Method, StatusCode, header},
    response::Response,
    routing::get,
};
use sdkwork_im_api_registry::HttpMethod;

use crate::client::resolve_max_upstream_response_body_bytes;
use crate::constants::SDKWORK_CONTEXT_PROJECTION_HEADERS;
use crate::proxy::{proxy_get_request, proxy_request};
use crate::state::GatewayState;

pub(crate) fn is_sdkwork_context_projection_header(name: &header::HeaderName) -> bool {
    SDKWORK_CONTEXT_PROJECTION_HEADERS
        .iter()
        .any(|candidate| name.as_str().eq_ignore_ascii_case(candidate))
}

pub(crate) async fn build_proxy_response(
    service_id: &str,
    upstream_response: reqwest::Response,
) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let max_body_bytes = resolve_max_upstream_response_body_bytes();
    let body = match upstream_response.bytes().await {
        Ok(body) if body.len() <= max_body_bytes => body,
        Ok(body) => {
            return json_error_response(
                StatusCode::BAD_GATEWAY,
                format!(
                    "gateway upstream response from {service_id} exceeded maximum size ({max_body_bytes} bytes, got {} bytes)",
                    body.len()
                )
                .as_str(),
            );
        }
        Err(error) => {
            return json_error_response(
                StatusCode::BAD_GATEWAY,
                format!("gateway failed to read upstream response from {service_id}: {error}")
                    .as_str(),
            );
        }
    };
    let mut response = build_raw_response(status, &headers, Body::from(body));
    response.headers_mut().insert(
        "x-sdkwork-im-upstream-service",
        axum::http::HeaderValue::from_str(service_id)
            .expect("static gateway upstream service id should be a valid header value"),
    );
    response
}

fn build_raw_response(status: StatusCode, headers: &HeaderMap, body: Body) -> Response {
    let mut response_builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        if *name == header::TRANSFER_ENCODING || *name == header::CONNECTION {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(body)
        .expect("proxied gateway response should build")
}

pub(crate) fn map_http_method(method: &Method) -> Option<HttpMethod> {
    match *method {
        Method::DELETE => Some(HttpMethod::Delete),
        Method::GET => Some(HttpMethod::Get),
        Method::HEAD => Some(HttpMethod::Head),
        Method::OPTIONS => Some(HttpMethod::Options),
        Method::PATCH => Some(HttpMethod::Patch),
        Method::POST => Some(HttpMethod::Post),
        Method::PUT => Some(HttpMethod::Put),
        _ => None,
    }
}

pub(crate) fn gateway_proxy_routes() -> axum::routing::MethodRouter<GatewayState> {
    get(proxy_get_request)
        .post(proxy_request)
        .put(proxy_request)
        .patch(proxy_request)
        .delete(proxy_request)
        .options(proxy_request)
}

pub(crate) fn json_error_response(status: StatusCode, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            serde_json::json!({
                "code": "gateway_proxy_error",
                "message": message
            })
            .to_string(),
        ))
        .expect("gateway json error response should build")
}

pub(crate) fn request_base_url(request: &Request) -> String {
    let scheme = forwarded_header_value(
        request.headers(),
        header::HeaderName::from_static("x-forwarded-proto"),
    )
    .or_else(|| request.uri().scheme_str().map(str::to_owned))
    .unwrap_or_else(|| "http".to_owned());
    let authority = forwarded_header_value(
        request.headers(),
        header::HeaderName::from_static("x-forwarded-host"),
    )
    .or_else(|| {
        request
            .headers()
            .get(header::HOST)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    })
    .or_else(|| {
        request
            .uri()
            .authority()
            .map(|value| value.as_str().to_owned())
    })
    .unwrap_or_else(|| "localhost".to_owned());
    format!("{scheme}://{authority}")
}

fn forwarded_header_value(headers: &header::HeaderMap, name: header::HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}
