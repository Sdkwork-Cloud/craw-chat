//! Thin adapter preserving the standalone gateway module boundary.

pub use sdkwork_im_gateway_assembly::{
    assemble_application_router as bootstrap_embedded_application_routes,
};
