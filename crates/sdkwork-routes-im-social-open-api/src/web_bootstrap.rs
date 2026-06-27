use axum::Router;
use sdkwork_im_web_bootstrap::wrap_im_open_api_service_router;

pub fn wrap_router(router: Router) -> Router {
    wrap_im_open_api_service_router(router)
}
