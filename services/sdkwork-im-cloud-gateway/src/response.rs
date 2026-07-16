//! Response shaping, header allow-listing, HTTP method mapping, gateway proxy
//! route wiring, and JSON error rendering shared across gateway handlers.

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, Method, StatusCode, header},
    response::Response,
    routing::get,
};
use futures_util::TryStreamExt;
use sdkwork_im_api_registry::{HttpMethod, RouteDescriptor};
use sdkwork_web_core::{ProblemCorrelation, WebFrameworkError, problem_response};

use crate::client::resolve_max_upstream_response_body_bytes;
use crate::constants::SDKWORK_CONTEXT_PROJECTION_HEADERS;
use crate::proxy::{proxy_get_request, proxy_request};
use crate::state::GatewayState;
use crate::trace_identity::new_server_trace_id;

pub(crate) fn is_sdkwork_context_projection_header(name: &header::HeaderName) -> bool {
    SDKWORK_CONTEXT_PROJECTION_HEADERS
        .iter()
        .any(|candidate| name.as_str().eq_ignore_ascii_case(candidate))
}

pub(crate) async fn build_proxy_response(
    service_id: &str,
    upstream_response: reqwest::Response,
    correlation: ProblemCorrelation<'_>,
    stream_response: bool,
) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    if stream_response {
        let service_id_for_error = service_id.to_owned();
        let body = Body::from_stream(upstream_response.bytes_stream().map_err(move |error| {
            tracing::warn!(
                service = %service_id_for_error,
                error = %error,
                "gateway upstream response stream failed"
            );
            std::io::Error::other("gateway upstream response stream failed")
        }));
        return attach_upstream_service_header(
            build_raw_response(status, &headers, body),
            service_id,
        );
    }

    let max_body_bytes = resolve_max_upstream_response_body_bytes();
    let body = match upstream_response.bytes().await {
        Ok(body) if body.len() <= max_body_bytes => body,
        Ok(body) => {
            return json_error_response_with_correlation(
                StatusCode::BAD_GATEWAY,
                format!(
                    "gateway upstream response from {service_id} exceeded maximum size ({max_body_bytes} bytes, got {} bytes)",
                    body.len()
                )
                .as_str(),
                correlation,
            );
        }
        Err(error) => {
            return json_error_response_with_correlation(
                StatusCode::BAD_GATEWAY,
                format!("gateway failed to read upstream response from {service_id}: {error}")
                    .as_str(),
                correlation,
            );
        }
    };
    attach_upstream_service_header(
        build_raw_response(status, &headers, Body::from(body)),
        service_id,
    )
}

fn attach_upstream_service_header(mut response: Response, service_id: &str) -> Response {
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

/// Legacy entry point for call sites that lack routing context. Prefer
/// [`json_error_response_with_correlation`] so that `instance`, `operationId`,
/// and `traceId` are populated per `API_SPEC.md` §15.2.
pub(crate) fn json_error_response(status: StatusCode, message: &str) -> Response {
    let trace_id = new_gateway_trace_id();
    json_error_response_with_correlation(
        status,
        message,
        ProblemCorrelation::new(None, Some(trace_id.as_str())),
    )
}

/// Renders an RFC 9457 `application/problem+json` response with full routing
/// correlation (`API_SPEC.md` §15.2, `OBSERVABILITY_SPEC.md` §2).
///
/// - `instance` is derived from `correlation.route_template` when available,
///   otherwise the fallback path is redacted so no user, tenant, file, object,
///   token, or provider identifiers leak.
/// - `operationId` is emitted when the gateway resolved the matched route.
/// - `traceId` is generated by the gateway boundary.
pub(crate) fn json_error_response_with_correlation(
    status: StatusCode,
    message: &str,
    correlation: ProblemCorrelation<'_>,
) -> Response {
    let error = match status {
        StatusCode::BAD_GATEWAY | StatusCode::SERVICE_UNAVAILABLE => {
            WebFrameworkError::dependency_unavailable(message)
        }
        StatusCode::PAYLOAD_TOO_LARGE => WebFrameworkError::payload_too_large(message),
        StatusCode::REQUEST_TIMEOUT => WebFrameworkError::request_timeout(message),
        _ => WebFrameworkError::internal_server_error(message),
    };
    problem_response(&error, correlation)
}

/// Builds a `ProblemCorrelation` from request parts and resolved route.
///
/// Per `OBSERVABILITY_SPEC.md` §2, `route_template` takes precedence over the
/// raw fallback path so `instance` reports `{METHOD} {routeTemplate}` instead
/// of a path containing business resource identifiers.
///
/// The returned correlation borrows `method`, `fallback_path`, `trace_id`, and
/// `route` — callers must keep these alive while the correlation is in use.
pub(crate) fn problem_correlation_for_parts<'a>(
    method: &'a str,
    fallback_path: &'a str,
    trace_id: &'a str,
    route: Option<&'a RouteDescriptor>,
) -> ProblemCorrelation<'a> {
    let route_template = route.map(|descriptor| descriptor.path_pattern.as_str());
    let operation_id = route.map(|descriptor| descriptor.operation_group.as_str());

    ProblemCorrelation::new(None, Some(trace_id)).with_routing(
        Some(method),
        route_template,
        Some(fallback_path),
        operation_id,
    )
}

/// Generates the server-owned traceId exposed on gateway-originated problem responses.
pub(crate) fn new_gateway_trace_id() -> String {
    new_server_trace_id()
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
