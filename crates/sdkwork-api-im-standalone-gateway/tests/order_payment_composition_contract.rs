use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::routing::{delete, get, patch, post, put};
use sdkwork_web_bootstrap::{
    ApiAssemblyContribution, ComposedApiAssembly, ReadinessCheck, ReadinessFuture,
};
use sdkwork_web_core::{HttpMethod, HttpRouteManifest};
use tower::ServiceExt;

#[derive(Clone)]
struct DeterministicReadiness;

impl ReadinessCheck for DeterministicReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        Box::pin(async { Ok(()) })
    }
}

async fn executable_route() -> StatusCode {
    StatusCode::NO_CONTENT
}

fn executable_router(manifest: &HttpRouteManifest) -> Router {
    manifest
        .routes()
        .iter()
        .fold(Router::new(), |router, route| {
            let method_router = match route.method {
                HttpMethod::Get => get(executable_route),
                HttpMethod::Post => post(executable_route),
                HttpMethod::Put => put(executable_route),
                HttpMethod::Patch => patch(executable_route),
                HttpMethod::Delete => delete(executable_route),
            };
            router.route(route.path, method_router)
        })
}

fn contribution(
    owner: &'static str,
    title: &str,
    manifest: HttpRouteManifest,
) -> ApiAssemblyContribution {
    let router = executable_router(&manifest);
    ApiAssemblyContribution::from_manifest(
        owner,
        title,
        router,
        manifest,
        Vec::new(),
        Arc::new(DeterministicReadiness),
    )
    .expect("manifest-backed executable contribution")
}

fn request_method(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
        HttpMethod::Put => Method::PUT,
        HttpMethod::Patch => Method::PATCH,
        HttpMethod::Delete => Method::DELETE,
    }
}

fn executable_path(template: &str) -> String {
    template
        .split('/')
        .map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                "fixture"
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[tokio::test]
async fn order_and_federated_payment_contributions_compose_with_router_manifest_parity() {
    let order_manifest = sdkwork_api_order_assembly::OrderAssemblyContract::app_route_manifest();
    let payment_manifest = sdkwork_api_payment_assembly::federated_app_route_manifest();
    let composed = ComposedApiAssembly::try_compose(
        "SDKWork IM Order and Payment Contract",
        vec![
            contribution("sdkwork-order", "SDKWork Order App API", order_manifest),
            contribution(
                "sdkwork-payment",
                "SDKWork Payment Federated App API",
                payment_manifest,
            ),
        ],
    )
    .expect("Order and federated Payment contributions must compose");

    assert!(composed.readiness_check.check().await.is_ok());
    assert!(
        composed
            .route_manifest
            .routes()
            .iter()
            .any(|route| { route.path == "/app/v3/api/orders/payments/webhooks/{providerCode}" })
    );
    assert!(
        composed
            .route_manifest
            .routes()
            .iter()
            .all(|route| { route.operation_id != "payments.webhooks.receiveDeprecated" })
    );

    let router = composed.router;
    for route in composed.route_manifest.routes() {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(request_method(route.method))
                    .uri(executable_path(route.path))
                    .body(Body::empty())
                    .expect("route request"),
            )
            .await
            .expect("route response");
        assert_eq!(
            StatusCode::NO_CONTENT,
            response.status(),
            "executable Router must cover {:?} {}",
            route.method,
            route.path,
        );
    }
}
