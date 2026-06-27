// @generated
/// Generated client implementations.
pub mod communication_ops_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CommunicationOpsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommunicationOpsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CommunicationOpsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CommunicationOpsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CommunicationOpsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn retrieve_health(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveHealthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveHealthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveHealth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveHealth",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveClusterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_lag(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveLagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveLagResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveLag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveLag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_replay_status(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveReplayStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveReplayStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveReplayStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveReplayStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_commercial_readiness(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveCommercialReadinessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveCommercialReadinessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveCommercialReadiness",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveCommercialReadiness",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_runtime_dir(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveRuntimeDirRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveRuntimeDirResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveRuntimeDir",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveRuntimeDir",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_ops_provider_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOpsProviderBindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOpsProviderBindingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/ListOpsProviderBindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "ListOpsProviderBindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_provider_binding_drift(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveProviderBindingDriftRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProviderBindingDriftResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveProviderBindingDrift",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveProviderBindingDrift",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_diagnostics(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveDiagnosticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveDiagnosticsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveDiagnostics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationOpsService",
                        "RetrieveDiagnostics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod communication_ops_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CommunicationOpsServiceServer.
    #[async_trait]
    pub trait CommunicationOpsService: std::marker::Send + std::marker::Sync + 'static {
        async fn retrieve_health(
            &self,
            request: tonic::Request<super::RetrieveHealthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveHealthResponse>,
            tonic::Status,
        >;
        async fn retrieve_cluster(
            &self,
            request: tonic::Request<super::RetrieveClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveClusterResponse>,
            tonic::Status,
        >;
        async fn retrieve_lag(
            &self,
            request: tonic::Request<super::RetrieveLagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveLagResponse>,
            tonic::Status,
        >;
        async fn retrieve_replay_status(
            &self,
            request: tonic::Request<super::RetrieveReplayStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveReplayStatusResponse>,
            tonic::Status,
        >;
        async fn retrieve_commercial_readiness(
            &self,
            request: tonic::Request<super::RetrieveCommercialReadinessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveCommercialReadinessResponse>,
            tonic::Status,
        >;
        async fn retrieve_runtime_dir(
            &self,
            request: tonic::Request<super::RetrieveRuntimeDirRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveRuntimeDirResponse>,
            tonic::Status,
        >;
        async fn list_ops_provider_bindings(
            &self,
            request: tonic::Request<super::ListOpsProviderBindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOpsProviderBindingsResponse>,
            tonic::Status,
        >;
        async fn retrieve_provider_binding_drift(
            &self,
            request: tonic::Request<super::RetrieveProviderBindingDriftRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProviderBindingDriftResponse>,
            tonic::Status,
        >;
        async fn retrieve_diagnostics(
            &self,
            request: tonic::Request<super::RetrieveDiagnosticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveDiagnosticsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CommunicationOpsServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CommunicationOpsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for CommunicationOpsServiceServer<T>
    where
        T: CommunicationOpsService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveHealth" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveHealthSvc<T: CommunicationOpsService>(pub Arc<T>);
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveHealthRequest>
                    for RetrieveHealthSvc<T> {
                        type Response = super::RetrieveHealthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveHealthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_health(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveHealthSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveCluster" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveClusterSvc<T: CommunicationOpsService>(pub Arc<T>);
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveClusterRequest>
                    for RetrieveClusterSvc<T> {
                        type Response = super::RetrieveClusterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveClusterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_cluster(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveClusterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveLag" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveLagSvc<T: CommunicationOpsService>(pub Arc<T>);
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveLagRequest>
                    for RetrieveLagSvc<T> {
                        type Response = super::RetrieveLagResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveLagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_lag(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveLagSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveReplayStatus" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveReplayStatusSvc<T: CommunicationOpsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveReplayStatusRequest>
                    for RetrieveReplayStatusSvc<T> {
                        type Response = super::RetrieveReplayStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveReplayStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_replay_status(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveReplayStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveCommercialReadiness" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveCommercialReadinessSvc<T: CommunicationOpsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<
                        super::RetrieveCommercialReadinessRequest,
                    > for RetrieveCommercialReadinessSvc<T> {
                        type Response = super::RetrieveCommercialReadinessResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveCommercialReadinessRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_commercial_readiness(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveCommercialReadinessSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveRuntimeDir" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveRuntimeDirSvc<T: CommunicationOpsService>(pub Arc<T>);
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveRuntimeDirRequest>
                    for RetrieveRuntimeDirSvc<T> {
                        type Response = super::RetrieveRuntimeDirResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveRuntimeDirRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_runtime_dir(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveRuntimeDirSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/ListOpsProviderBindings" => {
                    #[allow(non_camel_case_types)]
                    struct ListOpsProviderBindingsSvc<T: CommunicationOpsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::ListOpsProviderBindingsRequest>
                    for ListOpsProviderBindingsSvc<T> {
                        type Response = super::ListOpsProviderBindingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListOpsProviderBindingsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::list_ops_provider_bindings(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListOpsProviderBindingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveProviderBindingDrift" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveProviderBindingDriftSvc<T: CommunicationOpsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<
                        super::RetrieveProviderBindingDriftRequest,
                    > for RetrieveProviderBindingDriftSvc<T> {
                        type Response = super::RetrieveProviderBindingDriftResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveProviderBindingDriftRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_provider_binding_drift(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveProviderBindingDriftSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationOpsService/RetrieveDiagnostics" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveDiagnosticsSvc<T: CommunicationOpsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationOpsService,
                    > tonic::server::UnaryService<super::RetrieveDiagnosticsRequest>
                    for RetrieveDiagnosticsSvc<T> {
                        type Response = super::RetrieveDiagnosticsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveDiagnosticsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationOpsService>::retrieve_diagnostics(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveDiagnosticsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CommunicationOpsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.CommunicationOpsService";
    impl<T> tonic::server::NamedService for CommunicationOpsServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod realtime_node_admin_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RealtimeNodeAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RealtimeNodeAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RealtimeNodeAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RealtimeNodeAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RealtimeNodeAdminServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn activate_realtime_node(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateRealtimeNodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateRealtimeNodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/ActivateRealtimeNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.RealtimeNodeAdminService",
                        "ActivateRealtimeNode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn drain_realtime_node(
            &mut self,
            request: impl tonic::IntoRequest<super::DrainRealtimeNodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DrainRealtimeNodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/DrainRealtimeNode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.RealtimeNodeAdminService",
                        "DrainRealtimeNode",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn migrate_realtime_node_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::MigrateRealtimeNodeRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MigrateRealtimeNodeRoutesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/MigrateRealtimeNodeRoutes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.RealtimeNodeAdminService",
                        "MigrateRealtimeNodeRoutes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod realtime_node_admin_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RealtimeNodeAdminServiceServer.
    #[async_trait]
    pub trait RealtimeNodeAdminService: std::marker::Send + std::marker::Sync + 'static {
        async fn activate_realtime_node(
            &self,
            request: tonic::Request<super::ActivateRealtimeNodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateRealtimeNodeResponse>,
            tonic::Status,
        >;
        async fn drain_realtime_node(
            &self,
            request: tonic::Request<super::DrainRealtimeNodeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DrainRealtimeNodeResponse>,
            tonic::Status,
        >;
        async fn migrate_realtime_node_routes(
            &self,
            request: tonic::Request<super::MigrateRealtimeNodeRoutesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MigrateRealtimeNodeRoutesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RealtimeNodeAdminServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RealtimeNodeAdminServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for RealtimeNodeAdminServiceServer<T>
    where
        T: RealtimeNodeAdminService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/ActivateRealtimeNode" => {
                    #[allow(non_camel_case_types)]
                    struct ActivateRealtimeNodeSvc<T: RealtimeNodeAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RealtimeNodeAdminService,
                    > tonic::server::UnaryService<super::ActivateRealtimeNodeRequest>
                    for ActivateRealtimeNodeSvc<T> {
                        type Response = super::ActivateRealtimeNodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ActivateRealtimeNodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeNodeAdminService>::activate_realtime_node(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ActivateRealtimeNodeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/DrainRealtimeNode" => {
                    #[allow(non_camel_case_types)]
                    struct DrainRealtimeNodeSvc<T: RealtimeNodeAdminService>(pub Arc<T>);
                    impl<
                        T: RealtimeNodeAdminService,
                    > tonic::server::UnaryService<super::DrainRealtimeNodeRequest>
                    for DrainRealtimeNodeSvc<T> {
                        type Response = super::DrainRealtimeNodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DrainRealtimeNodeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeNodeAdminService>::drain_realtime_node(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DrainRealtimeNodeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.RealtimeNodeAdminService/MigrateRealtimeNodeRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct MigrateRealtimeNodeRoutesSvc<T: RealtimeNodeAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: RealtimeNodeAdminService,
                    > tonic::server::UnaryService<
                        super::MigrateRealtimeNodeRoutesRequest,
                    > for MigrateRealtimeNodeRoutesSvc<T> {
                        type Response = super::MigrateRealtimeNodeRoutesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MigrateRealtimeNodeRoutesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeNodeAdminService>::migrate_realtime_node_routes(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MigrateRealtimeNodeRoutesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for RealtimeNodeAdminServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.RealtimeNodeAdminService";
    impl<T> tonic::server::NamedService for RealtimeNodeAdminServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod communication_control_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CommunicationControlServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CommunicationControlServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CommunicationControlServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CommunicationControlServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CommunicationControlServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn retrieve_protocol_governance(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveProtocolGovernanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProtocolGovernanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProtocolGovernance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "RetrieveProtocolGovernance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_protocol_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveProtocolRegistryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProtocolRegistryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProtocolRegistry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "RetrieveProtocolRegistry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_provider_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProviderPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProviderPoliciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/ListProviderPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "ListProviderPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn preview_provider_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewProviderPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreviewProviderPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/PreviewProviderPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "PreviewProviderPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rollback_provider_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackProviderPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RollbackProviderPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/RollbackProviderPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "RollbackProviderPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_provider_registry(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveProviderRegistryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProviderRegistryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProviderRegistry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "RetrieveProviderRegistry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_control_provider_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListControlProviderBindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListControlProviderBindingsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/ListControlProviderBindings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "ListControlProviderBindings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_control_provider_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateControlProviderBindingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateControlProviderBindingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.CommunicationControlService/CreateControlProviderBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.CommunicationControlService",
                        "CreateControlProviderBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod communication_control_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CommunicationControlServiceServer.
    #[async_trait]
    pub trait CommunicationControlService: std::marker::Send + std::marker::Sync + 'static {
        async fn retrieve_protocol_governance(
            &self,
            request: tonic::Request<super::RetrieveProtocolGovernanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProtocolGovernanceResponse>,
            tonic::Status,
        >;
        async fn retrieve_protocol_registry(
            &self,
            request: tonic::Request<super::RetrieveProtocolRegistryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProtocolRegistryResponse>,
            tonic::Status,
        >;
        async fn list_provider_policies(
            &self,
            request: tonic::Request<super::ListProviderPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProviderPoliciesResponse>,
            tonic::Status,
        >;
        async fn preview_provider_policy(
            &self,
            request: tonic::Request<super::PreviewProviderPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PreviewProviderPolicyResponse>,
            tonic::Status,
        >;
        async fn rollback_provider_policy(
            &self,
            request: tonic::Request<super::RollbackProviderPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RollbackProviderPolicyResponse>,
            tonic::Status,
        >;
        async fn retrieve_provider_registry(
            &self,
            request: tonic::Request<super::RetrieveProviderRegistryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveProviderRegistryResponse>,
            tonic::Status,
        >;
        async fn list_control_provider_bindings(
            &self,
            request: tonic::Request<super::ListControlProviderBindingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListControlProviderBindingsResponse>,
            tonic::Status,
        >;
        async fn create_control_provider_binding(
            &self,
            request: tonic::Request<super::CreateControlProviderBindingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateControlProviderBindingResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CommunicationControlServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CommunicationControlServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for CommunicationControlServiceServer<T>
    where
        T: CommunicationControlService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProtocolGovernance" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveProtocolGovernanceSvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<
                        super::RetrieveProtocolGovernanceRequest,
                    > for RetrieveProtocolGovernanceSvc<T> {
                        type Response = super::RetrieveProtocolGovernanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveProtocolGovernanceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::retrieve_protocol_governance(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveProtocolGovernanceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProtocolRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveProtocolRegistrySvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<super::RetrieveProtocolRegistryRequest>
                    for RetrieveProtocolRegistrySvc<T> {
                        type Response = super::RetrieveProtocolRegistryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveProtocolRegistryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::retrieve_protocol_registry(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveProtocolRegistrySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/ListProviderPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct ListProviderPoliciesSvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<super::ListProviderPoliciesRequest>
                    for ListProviderPoliciesSvc<T> {
                        type Response = super::ListProviderPoliciesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProviderPoliciesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::list_provider_policies(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListProviderPoliciesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/PreviewProviderPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct PreviewProviderPolicySvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<super::PreviewProviderPolicyRequest>
                    for PreviewProviderPolicySvc<T> {
                        type Response = super::PreviewProviderPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PreviewProviderPolicyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::preview_provider_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PreviewProviderPolicySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/RollbackProviderPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RollbackProviderPolicySvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<super::RollbackProviderPolicyRequest>
                    for RollbackProviderPolicySvc<T> {
                        type Response = super::RollbackProviderPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RollbackProviderPolicyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::rollback_provider_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RollbackProviderPolicySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/RetrieveProviderRegistry" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveProviderRegistrySvc<T: CommunicationControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<super::RetrieveProviderRegistryRequest>
                    for RetrieveProviderRegistrySvc<T> {
                        type Response = super::RetrieveProviderRegistryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveProviderRegistryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::retrieve_provider_registry(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveProviderRegistrySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/ListControlProviderBindings" => {
                    #[allow(non_camel_case_types)]
                    struct ListControlProviderBindingsSvc<
                        T: CommunicationControlService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<
                        super::ListControlProviderBindingsRequest,
                    > for ListControlProviderBindingsSvc<T> {
                        type Response = super::ListControlProviderBindingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListControlProviderBindingsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::list_control_provider_bindings(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListControlProviderBindingsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.CommunicationControlService/CreateControlProviderBinding" => {
                    #[allow(non_camel_case_types)]
                    struct CreateControlProviderBindingSvc<
                        T: CommunicationControlService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: CommunicationControlService,
                    > tonic::server::UnaryService<
                        super::CreateControlProviderBindingRequest,
                    > for CreateControlProviderBindingSvc<T> {
                        type Response = super::CreateControlProviderBindingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateControlProviderBindingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CommunicationControlService>::create_control_provider_binding(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateControlProviderBindingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CommunicationControlServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.CommunicationControlService";
    impl<T> tonic::server::NamedService for CommunicationControlServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod social_admin_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SocialAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SocialAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SocialAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SocialAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SocialAdminServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_direct_chat_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDirectChatBindingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDirectChatBindingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateDirectChatBinding",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateDirectChatBinding",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_direct_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveDirectChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveDirectChatResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveDirectChat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveDirectChat",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_external_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateExternalConnectionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateExternalConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateExternalConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_external_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveExternalConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveExternalConnectionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveExternalConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveExternalConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_external_member_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExternalMemberLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateExternalMemberLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateExternalMemberLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateExternalMemberLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_external_member_link(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveExternalMemberLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveExternalMemberLinkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveExternalMemberLink",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveExternalMemberLink",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_managed_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateManagedFriendRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateManagedFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateManagedFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_managed_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveManagedFriendRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveManagedFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveManagedFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_managed_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptManagedFriendRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/AcceptManagedFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "AcceptManagedFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn decline_managed_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::DeclineManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeclineManagedFriendRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/DeclineManagedFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "DeclineManagedFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_managed_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelManagedFriendRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CancelManagedFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CancelManagedFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_managed_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateManagedFriendshipResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateManagedFriendship",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateManagedFriendship",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_managed_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveManagedFriendshipResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveManagedFriendship",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveManagedFriendship",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_managed_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveManagedFriendshipResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RemoveManagedFriendship",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RemoveManagedFriendship",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shared_channel_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSharedChannelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSharedChannelPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateSharedChannelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateSharedChannelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_shared_channel_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveSharedChannelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveSharedChannelPolicyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveSharedChannelPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveSharedChannelPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_user_block(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateUserBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "CreateUserBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_user_block(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveUserBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveUserBlockResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveUserBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialAdminService",
                        "RetrieveUserBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod social_admin_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SocialAdminServiceServer.
    #[async_trait]
    pub trait SocialAdminService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_direct_chat_binding(
            &self,
            request: tonic::Request<super::CreateDirectChatBindingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDirectChatBindingResponse>,
            tonic::Status,
        >;
        async fn retrieve_direct_chat(
            &self,
            request: tonic::Request<super::RetrieveDirectChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveDirectChatResponse>,
            tonic::Status,
        >;
        async fn create_external_connection(
            &self,
            request: tonic::Request<super::CreateExternalConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateExternalConnectionResponse>,
            tonic::Status,
        >;
        async fn retrieve_external_connection(
            &self,
            request: tonic::Request<super::RetrieveExternalConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveExternalConnectionResponse>,
            tonic::Status,
        >;
        async fn create_external_member_link(
            &self,
            request: tonic::Request<super::CreateExternalMemberLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateExternalMemberLinkResponse>,
            tonic::Status,
        >;
        async fn retrieve_external_member_link(
            &self,
            request: tonic::Request<super::RetrieveExternalMemberLinkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveExternalMemberLinkResponse>,
            tonic::Status,
        >;
        async fn create_managed_friend_request(
            &self,
            request: tonic::Request<super::CreateManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateManagedFriendRequestResponse>,
            tonic::Status,
        >;
        async fn retrieve_managed_friend_request(
            &self,
            request: tonic::Request<super::RetrieveManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveManagedFriendRequestResponse>,
            tonic::Status,
        >;
        async fn accept_managed_friend_request(
            &self,
            request: tonic::Request<super::AcceptManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptManagedFriendRequestResponse>,
            tonic::Status,
        >;
        async fn decline_managed_friend_request(
            &self,
            request: tonic::Request<super::DeclineManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeclineManagedFriendRequestResponse>,
            tonic::Status,
        >;
        async fn cancel_managed_friend_request(
            &self,
            request: tonic::Request<super::CancelManagedFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelManagedFriendRequestResponse>,
            tonic::Status,
        >;
        async fn create_managed_friendship(
            &self,
            request: tonic::Request<super::CreateManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateManagedFriendshipResponse>,
            tonic::Status,
        >;
        async fn retrieve_managed_friendship(
            &self,
            request: tonic::Request<super::RetrieveManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveManagedFriendshipResponse>,
            tonic::Status,
        >;
        async fn remove_managed_friendship(
            &self,
            request: tonic::Request<super::RemoveManagedFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveManagedFriendshipResponse>,
            tonic::Status,
        >;
        async fn create_shared_channel_policy(
            &self,
            request: tonic::Request<super::CreateSharedChannelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSharedChannelPolicyResponse>,
            tonic::Status,
        >;
        async fn retrieve_shared_channel_policy(
            &self,
            request: tonic::Request<super::RetrieveSharedChannelPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveSharedChannelPolicyResponse>,
            tonic::Status,
        >;
        async fn create_user_block(
            &self,
            request: tonic::Request<super::CreateUserBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserBlockResponse>,
            tonic::Status,
        >;
        async fn retrieve_user_block(
            &self,
            request: tonic::Request<super::RetrieveUserBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveUserBlockResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SocialAdminServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SocialAdminServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SocialAdminServiceServer<T>
    where
        T: SocialAdminService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateDirectChatBinding" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDirectChatBindingSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::CreateDirectChatBindingRequest>
                    for CreateDirectChatBindingSvc<T> {
                        type Response = super::CreateDirectChatBindingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDirectChatBindingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_direct_chat_binding(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateDirectChatBindingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveDirectChat" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveDirectChatSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::RetrieveDirectChatRequest>
                    for RetrieveDirectChatSvc<T> {
                        type Response = super::RetrieveDirectChatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveDirectChatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_direct_chat(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveDirectChatSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateExternalConnection" => {
                    #[allow(non_camel_case_types)]
                    struct CreateExternalConnectionSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::CreateExternalConnectionRequest>
                    for CreateExternalConnectionSvc<T> {
                        type Response = super::CreateExternalConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateExternalConnectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_external_connection(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateExternalConnectionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveExternalConnection" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveExternalConnectionSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::RetrieveExternalConnectionRequest,
                    > for RetrieveExternalConnectionSvc<T> {
                        type Response = super::RetrieveExternalConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveExternalConnectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_external_connection(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveExternalConnectionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateExternalMemberLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateExternalMemberLinkSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::CreateExternalMemberLinkRequest>
                    for CreateExternalMemberLinkSvc<T> {
                        type Response = super::CreateExternalMemberLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateExternalMemberLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_external_member_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateExternalMemberLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveExternalMemberLink" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveExternalMemberLinkSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::RetrieveExternalMemberLinkRequest,
                    > for RetrieveExternalMemberLinkSvc<T> {
                        type Response = super::RetrieveExternalMemberLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveExternalMemberLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_external_member_link(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveExternalMemberLinkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateManagedFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CreateManagedFriendRequestSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::CreateManagedFriendRequestRequest,
                    > for CreateManagedFriendRequestSvc<T> {
                        type Response = super::CreateManagedFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateManagedFriendRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_managed_friend_request(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateManagedFriendRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveManagedFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveManagedFriendRequestSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::RetrieveManagedFriendRequestRequest,
                    > for RetrieveManagedFriendRequestSvc<T> {
                        type Response = super::RetrieveManagedFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveManagedFriendRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_managed_friend_request(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveManagedFriendRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/AcceptManagedFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptManagedFriendRequestSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::AcceptManagedFriendRequestRequest,
                    > for AcceptManagedFriendRequestSvc<T> {
                        type Response = super::AcceptManagedFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AcceptManagedFriendRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::accept_managed_friend_request(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AcceptManagedFriendRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/DeclineManagedFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct DeclineManagedFriendRequestSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::DeclineManagedFriendRequestRequest,
                    > for DeclineManagedFriendRequestSvc<T> {
                        type Response = super::DeclineManagedFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeclineManagedFriendRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::decline_managed_friend_request(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeclineManagedFriendRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CancelManagedFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CancelManagedFriendRequestSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::CancelManagedFriendRequestRequest,
                    > for CancelManagedFriendRequestSvc<T> {
                        type Response = super::CancelManagedFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CancelManagedFriendRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::cancel_managed_friend_request(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CancelManagedFriendRequestSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateManagedFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct CreateManagedFriendshipSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::CreateManagedFriendshipRequest>
                    for CreateManagedFriendshipSvc<T> {
                        type Response = super::CreateManagedFriendshipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateManagedFriendshipRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_managed_friendship(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateManagedFriendshipSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveManagedFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveManagedFriendshipSvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::RetrieveManagedFriendshipRequest,
                    > for RetrieveManagedFriendshipSvc<T> {
                        type Response = super::RetrieveManagedFriendshipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveManagedFriendshipRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_managed_friendship(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveManagedFriendshipSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RemoveManagedFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveManagedFriendshipSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::RemoveManagedFriendshipRequest>
                    for RemoveManagedFriendshipSvc<T> {
                        type Response = super::RemoveManagedFriendshipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveManagedFriendshipRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::remove_managed_friendship(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveManagedFriendshipSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateSharedChannelPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSharedChannelPolicySvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::CreateSharedChannelPolicyRequest,
                    > for CreateSharedChannelPolicySvc<T> {
                        type Response = super::CreateSharedChannelPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateSharedChannelPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_shared_channel_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateSharedChannelPolicySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveSharedChannelPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveSharedChannelPolicySvc<T: SocialAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<
                        super::RetrieveSharedChannelPolicyRequest,
                    > for RetrieveSharedChannelPolicySvc<T> {
                        type Response = super::RetrieveSharedChannelPolicyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveSharedChannelPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_shared_channel_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveSharedChannelPolicySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/CreateUserBlock" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUserBlockSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::CreateUserBlockRequest>
                    for CreateUserBlockSvc<T> {
                        type Response = super::CreateUserBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUserBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::create_user_block(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateUserBlockSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialAdminService/RetrieveUserBlock" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveUserBlockSvc<T: SocialAdminService>(pub Arc<T>);
                    impl<
                        T: SocialAdminService,
                    > tonic::server::UnaryService<super::RetrieveUserBlockRequest>
                    for RetrieveUserBlockSvc<T> {
                        type Response = super::RetrieveUserBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveUserBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialAdminService>::retrieve_user_block(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveUserBlockSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for SocialAdminServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.SocialAdminService";
    impl<T> tonic::server::NamedService for SocialAdminServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod social_runtime_admin_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SocialRuntimeAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SocialRuntimeAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SocialRuntimeAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SocialRuntimeAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            SocialRuntimeAdminServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn claim_pending_shared_channel_sync_targeted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ClaimPendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ClaimPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ClaimPendingSharedChannelSyncTargeted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ClaimPendingSharedChannelSyncTargeted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_dead_letter_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDeadLetterSharedChannelSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListDeadLetterSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeadLetterSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ListDeadLetterSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_delivered_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDeliveredSharedChannelSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveredSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeliveredSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ListDeliveredSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_delivery_state_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListDeliveryStateSharedChannelSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveryStateSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeliveryStateSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ListDeliveryStateSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pending_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPendingSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPendingSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListPendingSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ListPendingSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reclaim_stale_pending_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ReclaimStalePendingSharedChannelSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ReclaimStalePendingSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ReclaimStalePendingSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ReclaimStalePendingSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn release_pending_shared_channel_sync_targeted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ReleasePendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ReleasePendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ReleasePendingSharedChannelSyncTargeted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "ReleasePendingSharedChannelSyncTargeted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn repair_derived_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::RepairDerivedSnapshotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepairDerivedSnapshotResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairDerivedSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "RepairDerivedSnapshot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn repair_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::RepairSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepairSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "RepairSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn republish_pending_shared_channel_sync_targeted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RepublishPendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RepublishPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepublishPendingSharedChannelSyncTargeted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "RepublishPendingSharedChannelSyncTargeted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn requeue_dead_letter_shared_channel_sync(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RequeueDeadLetterSharedChannelSyncRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RequeueDeadLetterSharedChannelSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RequeueDeadLetterSharedChannelSync",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "RequeueDeadLetterSharedChannelSync",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn requeue_dead_letter_shared_channel_sync_targeted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RequeueDeadLetterSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RequeueDeadLetterSharedChannelSyncTargetedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RequeueDeadLetterSharedChannelSyncTargeted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "RequeueDeadLetterSharedChannelSyncTargeted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn takeover_pending_shared_channel_sync_targeted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::TakeoverPendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::TakeoverPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/TakeoverPendingSharedChannelSyncTargeted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
                        "TakeoverPendingSharedChannelSyncTargeted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod social_runtime_admin_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SocialRuntimeAdminServiceServer.
    #[async_trait]
    pub trait SocialRuntimeAdminService: std::marker::Send + std::marker::Sync + 'static {
        async fn claim_pending_shared_channel_sync_targeted(
            &self,
            request: tonic::Request<super::ClaimPendingSharedChannelSyncTargetedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClaimPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        >;
        async fn list_dead_letter_shared_channel_sync(
            &self,
            request: tonic::Request<super::ListDeadLetterSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeadLetterSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn list_delivered_shared_channel_sync(
            &self,
            request: tonic::Request<super::ListDeliveredSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveredSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn list_delivery_state_shared_channel_sync(
            &self,
            request: tonic::Request<super::ListDeliveryStateSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeliveryStateSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn list_pending_shared_channel_sync(
            &self,
            request: tonic::Request<super::ListPendingSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPendingSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn reclaim_stale_pending_shared_channel_sync(
            &self,
            request: tonic::Request<super::ReclaimStalePendingSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReclaimStalePendingSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn release_pending_shared_channel_sync_targeted(
            &self,
            request: tonic::Request<
                super::ReleasePendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ReleasePendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        >;
        async fn repair_derived_snapshot(
            &self,
            request: tonic::Request<super::RepairDerivedSnapshotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepairDerivedSnapshotResponse>,
            tonic::Status,
        >;
        async fn repair_shared_channel_sync(
            &self,
            request: tonic::Request<super::RepairSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepairSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn republish_pending_shared_channel_sync_targeted(
            &self,
            request: tonic::Request<
                super::RepublishPendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RepublishPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        >;
        async fn requeue_dead_letter_shared_channel_sync(
            &self,
            request: tonic::Request<super::RequeueDeadLetterSharedChannelSyncRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequeueDeadLetterSharedChannelSyncResponse>,
            tonic::Status,
        >;
        async fn requeue_dead_letter_shared_channel_sync_targeted(
            &self,
            request: tonic::Request<
                super::RequeueDeadLetterSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RequeueDeadLetterSharedChannelSyncTargetedResponse>,
            tonic::Status,
        >;
        async fn takeover_pending_shared_channel_sync_targeted(
            &self,
            request: tonic::Request<
                super::TakeoverPendingSharedChannelSyncTargetedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::TakeoverPendingSharedChannelSyncTargetedResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SocialRuntimeAdminServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SocialRuntimeAdminServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for SocialRuntimeAdminServiceServer<T>
    where
        T: SocialRuntimeAdminService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ClaimPendingSharedChannelSyncTargeted" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimPendingSharedChannelSyncTargetedSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ClaimPendingSharedChannelSyncTargetedRequest,
                    > for ClaimPendingSharedChannelSyncTargetedSvc<T> {
                        type Response = super::ClaimPendingSharedChannelSyncTargetedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ClaimPendingSharedChannelSyncTargetedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::claim_pending_shared_channel_sync_targeted(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ClaimPendingSharedChannelSyncTargetedSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeadLetterSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeadLetterSharedChannelSyncSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ListDeadLetterSharedChannelSyncRequest,
                    > for ListDeadLetterSharedChannelSyncSvc<T> {
                        type Response = super::ListDeadLetterSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListDeadLetterSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::list_dead_letter_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDeadLetterSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeliveredSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeliveredSharedChannelSyncSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ListDeliveredSharedChannelSyncRequest,
                    > for ListDeliveredSharedChannelSyncSvc<T> {
                        type Response = super::ListDeliveredSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListDeliveredSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::list_delivered_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDeliveredSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListDeliveryStateSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeliveryStateSharedChannelSyncSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ListDeliveryStateSharedChannelSyncRequest,
                    > for ListDeliveryStateSharedChannelSyncSvc<T> {
                        type Response = super::ListDeliveryStateSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListDeliveryStateSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::list_delivery_state_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDeliveryStateSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ListPendingSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct ListPendingSharedChannelSyncSvc<T: SocialRuntimeAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ListPendingSharedChannelSyncRequest,
                    > for ListPendingSharedChannelSyncSvc<T> {
                        type Response = super::ListPendingSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListPendingSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::list_pending_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListPendingSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ReclaimStalePendingSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct ReclaimStalePendingSharedChannelSyncSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ReclaimStalePendingSharedChannelSyncRequest,
                    > for ReclaimStalePendingSharedChannelSyncSvc<T> {
                        type Response = super::ReclaimStalePendingSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ReclaimStalePendingSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::reclaim_stale_pending_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReclaimStalePendingSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/ReleasePendingSharedChannelSyncTargeted" => {
                    #[allow(non_camel_case_types)]
                    struct ReleasePendingSharedChannelSyncTargetedSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::ReleasePendingSharedChannelSyncTargetedRequest,
                    > for ReleasePendingSharedChannelSyncTargetedSvc<T> {
                        type Response = super::ReleasePendingSharedChannelSyncTargetedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ReleasePendingSharedChannelSyncTargetedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::release_pending_shared_channel_sync_targeted(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReleasePendingSharedChannelSyncTargetedSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairDerivedSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct RepairDerivedSnapshotSvc<T: SocialRuntimeAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<super::RepairDerivedSnapshotRequest>
                    for RepairDerivedSnapshotSvc<T> {
                        type Response = super::RepairDerivedSnapshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepairDerivedSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::repair_derived_snapshot(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RepairDerivedSnapshotSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepairSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct RepairSharedChannelSyncSvc<T: SocialRuntimeAdminService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<super::RepairSharedChannelSyncRequest>
                    for RepairSharedChannelSyncSvc<T> {
                        type Response = super::RepairSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RepairSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::repair_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RepairSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RepublishPendingSharedChannelSyncTargeted" => {
                    #[allow(non_camel_case_types)]
                    struct RepublishPendingSharedChannelSyncTargetedSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::RepublishPendingSharedChannelSyncTargetedRequest,
                    > for RepublishPendingSharedChannelSyncTargetedSvc<T> {
                        type Response = super::RepublishPendingSharedChannelSyncTargetedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RepublishPendingSharedChannelSyncTargetedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::republish_pending_shared_channel_sync_targeted(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RepublishPendingSharedChannelSyncTargetedSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RequeueDeadLetterSharedChannelSync" => {
                    #[allow(non_camel_case_types)]
                    struct RequeueDeadLetterSharedChannelSyncSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::RequeueDeadLetterSharedChannelSyncRequest,
                    > for RequeueDeadLetterSharedChannelSyncSvc<T> {
                        type Response = super::RequeueDeadLetterSharedChannelSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RequeueDeadLetterSharedChannelSyncRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::requeue_dead_letter_shared_channel_sync(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RequeueDeadLetterSharedChannelSyncSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/RequeueDeadLetterSharedChannelSyncTargeted" => {
                    #[allow(non_camel_case_types)]
                    struct RequeueDeadLetterSharedChannelSyncTargetedSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::RequeueDeadLetterSharedChannelSyncTargetedRequest,
                    > for RequeueDeadLetterSharedChannelSyncTargetedSvc<T> {
                        type Response = super::RequeueDeadLetterSharedChannelSyncTargetedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RequeueDeadLetterSharedChannelSyncTargetedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::requeue_dead_letter_shared_channel_sync_targeted(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RequeueDeadLetterSharedChannelSyncTargetedSvc(
                            inner,
                        );
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.SocialRuntimeAdminService/TakeoverPendingSharedChannelSyncTargeted" => {
                    #[allow(non_camel_case_types)]
                    struct TakeoverPendingSharedChannelSyncTargetedSvc<
                        T: SocialRuntimeAdminService,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: SocialRuntimeAdminService,
                    > tonic::server::UnaryService<
                        super::TakeoverPendingSharedChannelSyncTargetedRequest,
                    > for TakeoverPendingSharedChannelSyncTargetedSvc<T> {
                        type Response = super::TakeoverPendingSharedChannelSyncTargetedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TakeoverPendingSharedChannelSyncTargetedRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialRuntimeAdminService>::takeover_pending_shared_channel_sync_targeted(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TakeoverPendingSharedChannelSyncTargetedSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for SocialRuntimeAdminServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.SocialRuntimeAdminService";
    impl<T> tonic::server::NamedService for SocialRuntimeAdminServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod audit_admin_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AuditAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuditAdminServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AuditAdminServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AuditAdminServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AuditAdminServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_audit_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuditRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditRecordsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.AuditAdminService/ListAuditRecords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.AuditAdminService",
                        "ListAuditRecords",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_audit_record(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuditRecordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAuditRecordResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.AuditAdminService/CreateAuditRecord",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.AuditAdminService",
                        "CreateAuditRecord",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_audit_export(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveAuditExportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveAuditExportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdkwork.communication.backend.v3.AuditAdminService/RetrieveAuditExport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.backend.v3.AuditAdminService",
                        "RetrieveAuditExport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod audit_admin_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuditAdminServiceServer.
    #[async_trait]
    pub trait AuditAdminService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_audit_records(
            &self,
            request: tonic::Request<super::ListAuditRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuditRecordsResponse>,
            tonic::Status,
        >;
        async fn create_audit_record(
            &self,
            request: tonic::Request<super::CreateAuditRecordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAuditRecordResponse>,
            tonic::Status,
        >;
        async fn retrieve_audit_export(
            &self,
            request: tonic::Request<super::RetrieveAuditExportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveAuditExportResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AuditAdminServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AuditAdminServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuditAdminServiceServer<T>
    where
        T: AuditAdminService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/sdkwork.communication.backend.v3.AuditAdminService/ListAuditRecords" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuditRecordsSvc<T: AuditAdminService>(pub Arc<T>);
                    impl<
                        T: AuditAdminService,
                    > tonic::server::UnaryService<super::ListAuditRecordsRequest>
                    for ListAuditRecordsSvc<T> {
                        type Response = super::ListAuditRecordsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuditRecordsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditAdminService>::list_audit_records(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListAuditRecordsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.AuditAdminService/CreateAuditRecord" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuditRecordSvc<T: AuditAdminService>(pub Arc<T>);
                    impl<
                        T: AuditAdminService,
                    > tonic::server::UnaryService<super::CreateAuditRecordRequest>
                    for CreateAuditRecordSvc<T> {
                        type Response = super::CreateAuditRecordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAuditRecordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditAdminService>::create_audit_record(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateAuditRecordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdkwork.communication.backend.v3.AuditAdminService/RetrieveAuditExport" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveAuditExportSvc<T: AuditAdminService>(pub Arc<T>);
                    impl<
                        T: AuditAdminService,
                    > tonic::server::UnaryService<super::RetrieveAuditExportRequest>
                    for RetrieveAuditExportSvc<T> {
                        type Response = super::RetrieveAuditExportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveAuditExportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AuditAdminService>::retrieve_audit_export(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RetrieveAuditExportSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for AuditAdminServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "sdkwork.communication.backend.v3.AuditAdminService";
    impl<T> tonic::server::NamedService for AuditAdminServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
