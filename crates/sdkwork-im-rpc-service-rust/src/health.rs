use tokio_stream::Once;
use tonic::{Request, Response, Status};
use tonic_health::pb::health_check_response::ServingStatus;
use tonic_health::pb::health_server::{Health, HealthServer};
use tonic_health::pb::{HealthCheckRequest, HealthCheckResponse};

#[derive(Clone, Debug, Default)]
pub struct ImRpcHealthService;

impl ImRpcHealthService {
    fn service_status(service_name: &str) -> Result<ServingStatus, Status> {
        if service_name.is_empty()
            || crate::RPC_SERVICE_BINDINGS
                .iter()
                .any(|binding| binding.service_key == service_name)
        {
            Ok(ServingStatus::Serving)
        } else {
            Err(Status::not_found("service not registered"))
        }
    }

    fn response_for(service_name: &str) -> Result<HealthCheckResponse, Status> {
        Ok(HealthCheckResponse {
            status: Self::service_status(service_name)? as i32,
        })
    }
}

#[tonic::async_trait]
impl Health for ImRpcHealthService {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(Self::response_for(
            &request.get_ref().service,
        )?))
    }

    type WatchStream = Once<Result<HealthCheckResponse, Status>>;

    async fn watch(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let response = Self::response_for(&request.get_ref().service)?;
        Ok(Response::new(tokio_stream::once(Ok(response))))
    }
}

pub fn build_im_rpc_health_server() -> HealthServer<ImRpcHealthService> {
    HealthServer::new(ImRpcHealthService)
}
