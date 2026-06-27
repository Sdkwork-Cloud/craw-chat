use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::context::resolve_app_context_for_request;
use crate::error::app_context_error_response;
use crate::headers::has_any_dual_token_header;

pub async fn inject_app_request_context_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if request.method() == axum::http::Method::OPTIONS {
        return next.run(request).await;
    }

    if has_any_dual_token_header(request.headers()) {
        match resolve_app_context_for_request(
            request.headers(),
            request.uri().path(),
            request.method().as_str(),
        ) {
            Ok(resolved) => {
                request
                    .extensions_mut()
                    .insert(resolved.app_request_context);
                request.extensions_mut().insert(resolved.app_context);
            }
            Err(error) => return app_context_error_response(error),
        }
    }

    next.run(request).await
}
