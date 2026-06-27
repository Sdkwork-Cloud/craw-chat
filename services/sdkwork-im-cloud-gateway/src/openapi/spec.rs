//! OpenAPI service spec builders for the docs UI rendered by `sdkwork-im-openapi`.

use sdkwork_im_openapi::OpenApiServiceSpec;

pub(crate) fn aggregate_gateway_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Unified Gateway API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Aggregate OpenAPI contract served by sdkwork-im-cloud-gateway for the unified Sdkwork IM external HTTP surface.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

pub(crate) fn service_openapi_spec(service_id: &str) -> OpenApiServiceSpec<'static> {
    let title = Box::leak(format!("Sdkwork IM {} Service Contract", service_id).into_boxed_str());
    let description = Box::leak(
        format!("Gateway-hosted service contract view for {service_id}.").into_boxed_str(),
    );
    let openapi_path =
        Box::leak(format!("/openapi/services/{service_id}.openapi.json").into_boxed_str());
    let docs_path = Box::leak(format!("/docs/services/{service_id}").into_boxed_str());
    OpenApiServiceSpec {
        title,
        version: env!("CARGO_PKG_VERSION"),
        description,
        openapi_path,
        docs_path,
    }
}
