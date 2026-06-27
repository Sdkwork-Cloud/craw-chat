use std::future::Future;
use std::pin::Pin;

use futures_util::Stream;
use prost::Message;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::{ImRpcError, RpcMetadata, RpcMethodBinding, map_rpc_error_to_status};

pub type ImRpcBoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
pub type ImRpcBoxStream<T> = Pin<Box<dyn Stream<Item = T> + Send + 'static>>;
pub type ImRpcResponseStream<T> = ReceiverStream<Result<T, Status>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcUnaryRequest {
    pub binding: &'static RpcMethodBinding,
    pub metadata: RpcMetadata,
    pub request_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcUnaryResponse {
    pub response_bytes: Vec<u8>,
}

impl ImRpcUnaryResponse {
    pub fn from_message<M>(message: M) -> Result<Self, ImRpcError>
    where
        M: Message,
    {
        let mut response_bytes = Vec::with_capacity(message.encoded_len());
        message.encode(&mut response_bytes)?;
        Ok(Self { response_bytes })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcStreamRequest {
    pub binding: &'static RpcMethodBinding,
    pub metadata: RpcMetadata,
    pub request_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImRpcStreamResponse {
    pub response_bytes: Vec<u8>,
}

impl ImRpcStreamResponse {
    pub fn from_message<M>(message: M) -> Result<Self, ImRpcError>
    where
        M: Message,
    {
        let mut response_bytes = Vec::with_capacity(message.encoded_len());
        message.encode(&mut response_bytes)?;
        Ok(Self { response_bytes })
    }
}

pub trait ImRpcRuntimeDispatcher: Send + Sync + 'static {
    fn dispatch_unary(
        &self,
        request: ImRpcUnaryRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcUnaryResponse, ImRpcError>>;

    fn dispatch_server_stream(
        &self,
        request: ImRpcStreamRequest,
    ) -> ImRpcBoxFuture<Result<ImRpcBoxStream<Result<ImRpcStreamResponse, ImRpcError>>, ImRpcError>>
    {
        let _ = request;
        Box::pin(async {
            Err(ImRpcError::unimplemented(
                "server streaming dispatcher is not implemented",
            ))
        })
    }
}

pub async fn dispatch_unary_rpc<Req, Res>(
    dispatcher: &dyn ImRpcRuntimeDispatcher,
    binding: &'static RpcMethodBinding,
    request: Request<Req>,
) -> Result<Response<Res>, Status>
where
    Req: Message,
    Res: Message + Default,
{
    let metadata =
        RpcMetadata::from_metadata_map(request.metadata()).map_err(map_rpc_error_to_status)?;
    let request_message = request.into_inner();
    let mut request_bytes = Vec::with_capacity(request_message.encoded_len());
    request_message
        .encode(&mut request_bytes)
        .map_err(ImRpcError::from)
        .map_err(map_rpc_error_to_status)?;

    let response = dispatcher
        .dispatch_unary(ImRpcUnaryRequest {
            binding,
            metadata,
            request_bytes,
        })
        .await
        .map_err(map_rpc_error_to_status)?;
    let decoded = Res::decode(response.response_bytes.as_slice())
        .map_err(ImRpcError::from)
        .map_err(map_rpc_error_to_status)?;

    Ok(Response::new(decoded))
}

pub async fn dispatch_server_stream_rpc<Req, Res>(
    dispatcher: &dyn ImRpcRuntimeDispatcher,
    binding: &'static RpcMethodBinding,
    request: Request<Req>,
) -> Result<Response<ImRpcResponseStream<Res>>, Status>
where
    Req: Message,
    Res: Message + Default + Send + 'static,
{
    let metadata =
        RpcMetadata::from_metadata_map(request.metadata()).map_err(map_rpc_error_to_status)?;
    let request_message = request.into_inner();
    let mut request_bytes = Vec::with_capacity(request_message.encoded_len());
    request_message
        .encode(&mut request_bytes)
        .map_err(ImRpcError::from)
        .map_err(map_rpc_error_to_status)?;

    let runtime_stream = dispatcher
        .dispatch_server_stream(ImRpcStreamRequest {
            binding,
            metadata,
            request_bytes,
        })
        .await
        .map_err(map_rpc_error_to_status)?;
    let (sender, receiver) = tokio::sync::mpsc::channel(16);

    tokio::spawn(async move {
        let mut runtime_stream = runtime_stream;
        while let Some(item) = futures_util::StreamExt::next(&mut runtime_stream).await {
            let decoded = item.map_err(map_rpc_error_to_status).and_then(|response| {
                Res::decode(response.response_bytes.as_slice())
                    .map_err(ImRpcError::from)
                    .map_err(map_rpc_error_to_status)
            });

            if sender.send(decoded).await.is_err() {
                break;
            }
        }
    });

    Ok(Response::new(ReceiverStream::new(receiver)))
}
