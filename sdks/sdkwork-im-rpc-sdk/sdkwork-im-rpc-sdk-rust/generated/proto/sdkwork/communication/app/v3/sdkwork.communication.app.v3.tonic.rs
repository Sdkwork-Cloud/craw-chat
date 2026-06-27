// @generated
/// Generated client implementations.
pub mod automation_service_client {
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
    pub struct AutomationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AutomationServiceClient<tonic::transport::Channel> {
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
    impl<T> AutomationServiceClient<T>
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
        ) -> AutomationServiceClient<InterceptedService<T, F>>
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
            AutomationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_automation_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAutomationExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomationExecutionResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAutomationExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "CreateAutomationExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_automation_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveAutomationExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveAutomationExecutionResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/RetrieveAutomationExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "RetrieveAutomationExecution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_agent_response(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentResponseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentResponseResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAgentResponse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "CreateAgentResponse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_agent_response(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteAgentResponseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteAgentResponseResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/CompleteAgentResponse",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "CompleteAgentResponse",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_agent_response_frame(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentResponseFrameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentResponseFrameResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAgentResponseFrame",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "CreateAgentResponseFrame",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_agent_tool_call(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestAgentToolCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAgentToolCallResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/RequestAgentToolCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "RequestAgentToolCall",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_agent_tool_call(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteAgentToolCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteAgentToolCallResponse>,
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
                "/sdkwork.communication.app.v3.AutomationService/CompleteAgentToolCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.AutomationService",
                        "CompleteAgentToolCall",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod automation_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AutomationServiceServer.
    #[async_trait]
    pub trait AutomationService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_automation_execution(
            &self,
            request: tonic::Request<super::CreateAutomationExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAutomationExecutionResponse>,
            tonic::Status,
        >;
        async fn retrieve_automation_execution(
            &self,
            request: tonic::Request<super::RetrieveAutomationExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveAutomationExecutionResponse>,
            tonic::Status,
        >;
        async fn create_agent_response(
            &self,
            request: tonic::Request<super::CreateAgentResponseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentResponseResponse>,
            tonic::Status,
        >;
        async fn complete_agent_response(
            &self,
            request: tonic::Request<super::CompleteAgentResponseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteAgentResponseResponse>,
            tonic::Status,
        >;
        async fn create_agent_response_frame(
            &self,
            request: tonic::Request<super::CreateAgentResponseFrameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentResponseFrameResponse>,
            tonic::Status,
        >;
        async fn request_agent_tool_call(
            &self,
            request: tonic::Request<super::RequestAgentToolCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RequestAgentToolCallResponse>,
            tonic::Status,
        >;
        async fn complete_agent_tool_call(
            &self,
            request: tonic::Request<super::CompleteAgentToolCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteAgentToolCallResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AutomationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AutomationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AutomationServiceServer<T>
    where
        T: AutomationService,
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAutomationExecution" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAutomationExecutionSvc<T: AutomationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<
                        super::CreateAutomationExecutionRequest,
                    > for CreateAutomationExecutionSvc<T> {
                        type Response = super::CreateAutomationExecutionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAutomationExecutionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::create_automation_execution(
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
                        let method = CreateAutomationExecutionSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/RetrieveAutomationExecution" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveAutomationExecutionSvc<T: AutomationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<
                        super::RetrieveAutomationExecutionRequest,
                    > for RetrieveAutomationExecutionSvc<T> {
                        type Response = super::RetrieveAutomationExecutionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveAutomationExecutionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::retrieve_automation_execution(
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
                        let method = RetrieveAutomationExecutionSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAgentResponse" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAgentResponseSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::CreateAgentResponseRequest>
                    for CreateAgentResponseSvc<T> {
                        type Response = super::CreateAgentResponseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAgentResponseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::create_agent_response(
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
                        let method = CreateAgentResponseSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/CompleteAgentResponse" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAgentResponseSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::CompleteAgentResponseRequest>
                    for CompleteAgentResponseSvc<T> {
                        type Response = super::CompleteAgentResponseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteAgentResponseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::complete_agent_response(
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
                        let method = CompleteAgentResponseSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/CreateAgentResponseFrame" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAgentResponseFrameSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::CreateAgentResponseFrameRequest>
                    for CreateAgentResponseFrameSvc<T> {
                        type Response = super::CreateAgentResponseFrameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAgentResponseFrameRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::create_agent_response_frame(
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
                        let method = CreateAgentResponseFrameSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/RequestAgentToolCall" => {
                    #[allow(non_camel_case_types)]
                    struct RequestAgentToolCallSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::RequestAgentToolCallRequest>
                    for RequestAgentToolCallSvc<T> {
                        type Response = super::RequestAgentToolCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestAgentToolCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::request_agent_tool_call(
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
                        let method = RequestAgentToolCallSvc(inner);
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
                "/sdkwork.communication.app.v3.AutomationService/CompleteAgentToolCall" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteAgentToolCallSvc<T: AutomationService>(pub Arc<T>);
                    impl<
                        T: AutomationService,
                    > tonic::server::UnaryService<super::CompleteAgentToolCallRequest>
                    for CompleteAgentToolCallSvc<T> {
                        type Response = super::CompleteAgentToolCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteAgentToolCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AutomationService>::complete_agent_tool_call(
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
                        let method = CompleteAgentToolCallSvc(inner);
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
    impl<T> Clone for AutomationServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.AutomationService";
    impl<T> tonic::server::NamedService for AutomationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod call_service_client {
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
    pub struct CallServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CallServiceClient<tonic::transport::Channel> {
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
    impl<T> CallServiceClient<T>
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
        ) -> CallServiceClient<InterceptedService<T, F>>
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
            CallServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/CreateCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "CreateCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/RetrieveCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "RetrieveCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn invite_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::InviteCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/InviteCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "InviteCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/AcceptCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "AcceptCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reject_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::RejectCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RejectCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/RejectCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "RejectCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn end_call_session(
            &mut self,
            request: impl tonic::IntoRequest<super::EndCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EndCallSessionResponse>,
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
                "/sdkwork.communication.app.v3.CallService/EndCallSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "EndCallSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_call_signal(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCallSignalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallSignalResponse>,
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
                "/sdkwork.communication.app.v3.CallService/CreateCallSignal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "CreateCallSignal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_call_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCallCredentialRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallCredentialResponse>,
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
                "/sdkwork.communication.app.v3.CallService/CreateCallCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "CreateCallCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_call_signals(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchCallSignalsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchCallSignalsResponse>>,
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
                "/sdkwork.communication.app.v3.CallService/WatchCallSignals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.CallService",
                        "WatchCallSignals",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod call_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CallServiceServer.
    #[async_trait]
    pub trait CallService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_call_session(
            &self,
            request: tonic::Request<super::CreateCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallSessionResponse>,
            tonic::Status,
        >;
        async fn retrieve_call_session(
            &self,
            request: tonic::Request<super::RetrieveCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveCallSessionResponse>,
            tonic::Status,
        >;
        async fn invite_call_session(
            &self,
            request: tonic::Request<super::InviteCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteCallSessionResponse>,
            tonic::Status,
        >;
        async fn accept_call_session(
            &self,
            request: tonic::Request<super::AcceptCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptCallSessionResponse>,
            tonic::Status,
        >;
        async fn reject_call_session(
            &self,
            request: tonic::Request<super::RejectCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RejectCallSessionResponse>,
            tonic::Status,
        >;
        async fn end_call_session(
            &self,
            request: tonic::Request<super::EndCallSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EndCallSessionResponse>,
            tonic::Status,
        >;
        async fn create_call_signal(
            &self,
            request: tonic::Request<super::CreateCallSignalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallSignalResponse>,
            tonic::Status,
        >;
        async fn create_call_credential(
            &self,
            request: tonic::Request<super::CreateCallCredentialRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCallCredentialResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchCallSignals method.
        type WatchCallSignalsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::WatchCallSignalsResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn watch_call_signals(
            &self,
            request: tonic::Request<super::WatchCallSignalsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchCallSignalsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CallServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CallServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CallServiceServer<T>
    where
        T: CallService,
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
                "/sdkwork.communication.app.v3.CallService/CreateCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::CreateCallSessionRequest>
                    for CreateCallSessionSvc<T> {
                        type Response = super::CreateCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::create_call_session(&inner, request)
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
                        let method = CreateCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/RetrieveCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::RetrieveCallSessionRequest>
                    for RetrieveCallSessionSvc<T> {
                        type Response = super::RetrieveCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::retrieve_call_session(&inner, request)
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
                        let method = RetrieveCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/InviteCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct InviteCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::InviteCallSessionRequest>
                    for InviteCallSessionSvc<T> {
                        type Response = super::InviteCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InviteCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::invite_call_session(&inner, request)
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
                        let method = InviteCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/AcceptCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::AcceptCallSessionRequest>
                    for AcceptCallSessionSvc<T> {
                        type Response = super::AcceptCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::accept_call_session(&inner, request)
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
                        let method = AcceptCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/RejectCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct RejectCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::RejectCallSessionRequest>
                    for RejectCallSessionSvc<T> {
                        type Response = super::RejectCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RejectCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::reject_call_session(&inner, request)
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
                        let method = RejectCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/EndCallSession" => {
                    #[allow(non_camel_case_types)]
                    struct EndCallSessionSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::EndCallSessionRequest>
                    for EndCallSessionSvc<T> {
                        type Response = super::EndCallSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EndCallSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::end_call_session(&inner, request).await
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
                        let method = EndCallSessionSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/CreateCallSignal" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCallSignalSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::CreateCallSignalRequest>
                    for CreateCallSignalSvc<T> {
                        type Response = super::CreateCallSignalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCallSignalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::create_call_signal(&inner, request)
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
                        let method = CreateCallSignalSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/CreateCallCredential" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCallCredentialSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::UnaryService<super::CreateCallCredentialRequest>
                    for CreateCallCredentialSvc<T> {
                        type Response = super::CreateCallCredentialResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCallCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::create_call_credential(&inner, request)
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
                        let method = CreateCallCredentialSvc(inner);
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
                "/sdkwork.communication.app.v3.CallService/WatchCallSignals" => {
                    #[allow(non_camel_case_types)]
                    struct WatchCallSignalsSvc<T: CallService>(pub Arc<T>);
                    impl<
                        T: CallService,
                    > tonic::server::ServerStreamingService<
                        super::WatchCallSignalsRequest,
                    > for WatchCallSignalsSvc<T> {
                        type Response = super::WatchCallSignalsResponse;
                        type ResponseStream = T::WatchCallSignalsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchCallSignalsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CallService>::watch_call_signals(&inner, request)
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
                        let method = WatchCallSignalsSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T> Clone for CallServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.CallService";
    impl<T> tonic::server::NamedService for CallServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod conversation_service_client {
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
    pub struct ConversationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConversationServiceClient<tonic::transport::Channel> {
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
    impl<T> ConversationServiceClient<T>
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
        ) -> ConversationServiceClient<InterceptedService<T, F>>
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
            ConversationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConversationResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "CreateConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_agent_dialog(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentDialogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentDialogResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateAgentDialog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "CreateAgentDialog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_agent_handoff(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAgentHandoffRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentHandoffResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateAgentHandoff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "CreateAgentHandoff",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_system_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSystemChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSystemChannelResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateSystemChannel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "CreateSystemChannel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateThreadResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateThread",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "CreateThread",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bind_direct_chat(
            &mut self,
            request: impl tonic::IntoRequest<super::BindDirectChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BindDirectChatResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/BindDirectChat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "BindDirectChat",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RetrieveConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_inbox(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveInboxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveInboxResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveInbox",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RetrieveInbox",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_conversation_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMembersResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/ListConversationMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "ListConversationMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_conversation_member(
            &mut self,
            request: impl tonic::IntoRequest<super::AddConversationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddConversationMemberResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/AddConversationMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "AddConversationMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_conversation_member(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveConversationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveConversationMemberResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RemoveConversationMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RemoveConversationMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_conversation_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferConversationOwnerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferConversationOwnerResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/TransferConversationOwner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "TransferConversationOwner",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_conversation_member_role(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeConversationMemberRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeConversationMemberRoleResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/ChangeConversationMemberRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "ChangeConversationMemberRole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn leave_conversation(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeaveConversationResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/LeaveConversation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "LeaveConversation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_conversation_preferences(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RetrieveConversationPreferencesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationPreferencesResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversationPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RetrieveConversationPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_conversation_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConversationPreferencesResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateConversationPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "UpdateConversationPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationProfileResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RetrieveConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_conversation_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConversationProfileResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateConversationProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "UpdateConversationProfile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_read_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveReadCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveReadCursorResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveReadCursor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "RetrieveReadCursor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_read_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReadCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateReadCursorResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateReadCursor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "UpdateReadCursor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_conversation_member_directory(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListConversationMemberDirectoryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMemberDirectoryResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/ListConversationMemberDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "ListConversationMemberDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_pinned_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPinnedMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPinnedMessagesResponse>,
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
                "/sdkwork.communication.app.v3.ConversationService/ListPinnedMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ConversationService",
                        "ListPinnedMessages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod conversation_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ConversationServiceServer.
    #[async_trait]
    pub trait ConversationService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_conversation(
            &self,
            request: tonic::Request<super::CreateConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConversationResponse>,
            tonic::Status,
        >;
        async fn create_agent_dialog(
            &self,
            request: tonic::Request<super::CreateAgentDialogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentDialogResponse>,
            tonic::Status,
        >;
        async fn create_agent_handoff(
            &self,
            request: tonic::Request<super::CreateAgentHandoffRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAgentHandoffResponse>,
            tonic::Status,
        >;
        async fn create_system_channel(
            &self,
            request: tonic::Request<super::CreateSystemChannelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSystemChannelResponse>,
            tonic::Status,
        >;
        async fn create_thread(
            &self,
            request: tonic::Request<super::CreateThreadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateThreadResponse>,
            tonic::Status,
        >;
        async fn bind_direct_chat(
            &self,
            request: tonic::Request<super::BindDirectChatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BindDirectChatResponse>,
            tonic::Status,
        >;
        async fn retrieve_conversation(
            &self,
            request: tonic::Request<super::RetrieveConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationResponse>,
            tonic::Status,
        >;
        async fn retrieve_inbox(
            &self,
            request: tonic::Request<super::RetrieveInboxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveInboxResponse>,
            tonic::Status,
        >;
        async fn list_conversation_members(
            &self,
            request: tonic::Request<super::ListConversationMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMembersResponse>,
            tonic::Status,
        >;
        async fn add_conversation_member(
            &self,
            request: tonic::Request<super::AddConversationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddConversationMemberResponse>,
            tonic::Status,
        >;
        async fn remove_conversation_member(
            &self,
            request: tonic::Request<super::RemoveConversationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveConversationMemberResponse>,
            tonic::Status,
        >;
        async fn transfer_conversation_owner(
            &self,
            request: tonic::Request<super::TransferConversationOwnerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferConversationOwnerResponse>,
            tonic::Status,
        >;
        async fn change_conversation_member_role(
            &self,
            request: tonic::Request<super::ChangeConversationMemberRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeConversationMemberRoleResponse>,
            tonic::Status,
        >;
        async fn leave_conversation(
            &self,
            request: tonic::Request<super::LeaveConversationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeaveConversationResponse>,
            tonic::Status,
        >;
        async fn retrieve_conversation_preferences(
            &self,
            request: tonic::Request<super::RetrieveConversationPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationPreferencesResponse>,
            tonic::Status,
        >;
        async fn update_conversation_preferences(
            &self,
            request: tonic::Request<super::UpdateConversationPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConversationPreferencesResponse>,
            tonic::Status,
        >;
        async fn retrieve_conversation_profile(
            &self,
            request: tonic::Request<super::RetrieveConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveConversationProfileResponse>,
            tonic::Status,
        >;
        async fn update_conversation_profile(
            &self,
            request: tonic::Request<super::UpdateConversationProfileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateConversationProfileResponse>,
            tonic::Status,
        >;
        async fn retrieve_read_cursor(
            &self,
            request: tonic::Request<super::RetrieveReadCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveReadCursorResponse>,
            tonic::Status,
        >;
        async fn update_read_cursor(
            &self,
            request: tonic::Request<super::UpdateReadCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateReadCursorResponse>,
            tonic::Status,
        >;
        async fn list_conversation_member_directory(
            &self,
            request: tonic::Request<super::ListConversationMemberDirectoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMemberDirectoryResponse>,
            tonic::Status,
        >;
        async fn list_pinned_messages(
            &self,
            request: tonic::Request<super::ListPinnedMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPinnedMessagesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ConversationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ConversationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ConversationServiceServer<T>
    where
        T: ConversationService,
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
                "/sdkwork.communication.app.v3.ConversationService/CreateConversation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConversationSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::CreateConversationRequest>
                    for CreateConversationSvc<T> {
                        type Response = super::CreateConversationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConversationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::create_conversation(
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
                        let method = CreateConversationSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/CreateAgentDialog" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAgentDialogSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::CreateAgentDialogRequest>
                    for CreateAgentDialogSvc<T> {
                        type Response = super::CreateAgentDialogResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAgentDialogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::create_agent_dialog(
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
                        let method = CreateAgentDialogSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/CreateAgentHandoff" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAgentHandoffSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::CreateAgentHandoffRequest>
                    for CreateAgentHandoffSvc<T> {
                        type Response = super::CreateAgentHandoffResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAgentHandoffRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::create_agent_handoff(
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
                        let method = CreateAgentHandoffSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/CreateSystemChannel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSystemChannelSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::CreateSystemChannelRequest>
                    for CreateSystemChannelSvc<T> {
                        type Response = super::CreateSystemChannelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSystemChannelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::create_system_channel(
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
                        let method = CreateSystemChannelSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/CreateThread" => {
                    #[allow(non_camel_case_types)]
                    struct CreateThreadSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::CreateThreadRequest>
                    for CreateThreadSvc<T> {
                        type Response = super::CreateThreadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateThreadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::create_thread(&inner, request)
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
                        let method = CreateThreadSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/BindDirectChat" => {
                    #[allow(non_camel_case_types)]
                    struct BindDirectChatSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::BindDirectChatRequest>
                    for BindDirectChatSvc<T> {
                        type Response = super::BindDirectChatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BindDirectChatRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::bind_direct_chat(
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
                        let method = BindDirectChatSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversation" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveConversationSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::RetrieveConversationRequest>
                    for RetrieveConversationSvc<T> {
                        type Response = super::RetrieveConversationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveConversationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::retrieve_conversation(
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
                        let method = RetrieveConversationSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveInbox" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveInboxSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::RetrieveInboxRequest>
                    for RetrieveInboxSvc<T> {
                        type Response = super::RetrieveInboxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveInboxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::retrieve_inbox(&inner, request)
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
                        let method = RetrieveInboxSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/ListConversationMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListConversationMembersSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::ListConversationMembersRequest>
                    for ListConversationMembersSvc<T> {
                        type Response = super::ListConversationMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListConversationMembersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::list_conversation_members(
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
                        let method = ListConversationMembersSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/AddConversationMember" => {
                    #[allow(non_camel_case_types)]
                    struct AddConversationMemberSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::AddConversationMemberRequest>
                    for AddConversationMemberSvc<T> {
                        type Response = super::AddConversationMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddConversationMemberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::add_conversation_member(
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
                        let method = AddConversationMemberSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RemoveConversationMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveConversationMemberSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::RemoveConversationMemberRequest>
                    for RemoveConversationMemberSvc<T> {
                        type Response = super::RemoveConversationMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveConversationMemberRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::remove_conversation_member(
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
                        let method = RemoveConversationMemberSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/TransferConversationOwner" => {
                    #[allow(non_camel_case_types)]
                    struct TransferConversationOwnerSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::TransferConversationOwnerRequest,
                    > for TransferConversationOwnerSvc<T> {
                        type Response = super::TransferConversationOwnerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TransferConversationOwnerRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::transfer_conversation_owner(
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
                        let method = TransferConversationOwnerSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/ChangeConversationMemberRole" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeConversationMemberRoleSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::ChangeConversationMemberRoleRequest,
                    > for ChangeConversationMemberRoleSvc<T> {
                        type Response = super::ChangeConversationMemberRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ChangeConversationMemberRoleRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::change_conversation_member_role(
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
                        let method = ChangeConversationMemberRoleSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/LeaveConversation" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveConversationSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::LeaveConversationRequest>
                    for LeaveConversationSvc<T> {
                        type Response = super::LeaveConversationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaveConversationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::leave_conversation(
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
                        let method = LeaveConversationSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversationPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveConversationPreferencesSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::RetrieveConversationPreferencesRequest,
                    > for RetrieveConversationPreferencesSvc<T> {
                        type Response = super::RetrieveConversationPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveConversationPreferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::retrieve_conversation_preferences(
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
                        let method = RetrieveConversationPreferencesSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateConversationPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConversationPreferencesSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::UpdateConversationPreferencesRequest,
                    > for UpdateConversationPreferencesSvc<T> {
                        type Response = super::UpdateConversationPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateConversationPreferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::update_conversation_preferences(
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
                        let method = UpdateConversationPreferencesSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveConversationProfile" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveConversationProfileSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::RetrieveConversationProfileRequest,
                    > for RetrieveConversationProfileSvc<T> {
                        type Response = super::RetrieveConversationProfileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveConversationProfileRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::retrieve_conversation_profile(
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
                        let method = RetrieveConversationProfileSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateConversationProfile" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConversationProfileSvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::UpdateConversationProfileRequest,
                    > for UpdateConversationProfileSvc<T> {
                        type Response = super::UpdateConversationProfileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateConversationProfileRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::update_conversation_profile(
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
                        let method = UpdateConversationProfileSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/RetrieveReadCursor" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveReadCursorSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::RetrieveReadCursorRequest>
                    for RetrieveReadCursorSvc<T> {
                        type Response = super::RetrieveReadCursorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveReadCursorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::retrieve_read_cursor(
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
                        let method = RetrieveReadCursorSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/UpdateReadCursor" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateReadCursorSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::UpdateReadCursorRequest>
                    for UpdateReadCursorSvc<T> {
                        type Response = super::UpdateReadCursorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateReadCursorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::update_read_cursor(
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
                        let method = UpdateReadCursorSvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/ListConversationMemberDirectory" => {
                    #[allow(non_camel_case_types)]
                    struct ListConversationMemberDirectorySvc<T: ConversationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<
                        super::ListConversationMemberDirectoryRequest,
                    > for ListConversationMemberDirectorySvc<T> {
                        type Response = super::ListConversationMemberDirectoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListConversationMemberDirectoryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::list_conversation_member_directory(
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
                        let method = ListConversationMemberDirectorySvc(inner);
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
                "/sdkwork.communication.app.v3.ConversationService/ListPinnedMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ListPinnedMessagesSvc<T: ConversationService>(pub Arc<T>);
                    impl<
                        T: ConversationService,
                    > tonic::server::UnaryService<super::ListPinnedMessagesRequest>
                    for ListPinnedMessagesSvc<T> {
                        type Response = super::ListPinnedMessagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPinnedMessagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ConversationService>::list_pinned_messages(
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
                        let method = ListPinnedMessagesSvc(inner);
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
    impl<T> Clone for ConversationServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.ConversationService";
    impl<T> tonic::server::NamedService for ConversationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod contact_service_client {
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
    pub struct ContactServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContactServiceClient<tonic::transport::Channel> {
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
    impl<T> ContactServiceClient<T>
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
        ) -> ContactServiceClient<InterceptedService<T, F>>
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
            ContactServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/ListContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "ListContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_contact_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactTagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactTagsResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/ListContactTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "ListContactTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_contact_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateContactTagResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/CreateContactTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "CreateContactTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_contact_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactTagResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/UpdateContactTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "UpdateContactTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_contact_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteContactTagResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/DeleteContactTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "DeleteContactTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_contact_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContactRecommendationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateContactRecommendationResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/CreateContactRecommendation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "CreateContactRecommendation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_contact_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveContactPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveContactPreferencesResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/RetrieveContactPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "RetrieveContactPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_contact_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContactPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactPreferencesResponse>,
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
                "/sdkwork.communication.app.v3.ContactService/UpdateContactPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.ContactService",
                        "UpdateContactPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod contact_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ContactServiceServer.
    #[async_trait]
    pub trait ContactService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_contacts(
            &self,
            request: tonic::Request<super::ListContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsResponse>,
            tonic::Status,
        >;
        async fn list_contact_tags(
            &self,
            request: tonic::Request<super::ListContactTagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactTagsResponse>,
            tonic::Status,
        >;
        async fn create_contact_tag(
            &self,
            request: tonic::Request<super::CreateContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateContactTagResponse>,
            tonic::Status,
        >;
        async fn update_contact_tag(
            &self,
            request: tonic::Request<super::UpdateContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactTagResponse>,
            tonic::Status,
        >;
        async fn delete_contact_tag(
            &self,
            request: tonic::Request<super::DeleteContactTagRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteContactTagResponse>,
            tonic::Status,
        >;
        async fn create_contact_recommendation(
            &self,
            request: tonic::Request<super::CreateContactRecommendationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateContactRecommendationResponse>,
            tonic::Status,
        >;
        async fn retrieve_contact_preferences(
            &self,
            request: tonic::Request<super::RetrieveContactPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveContactPreferencesResponse>,
            tonic::Status,
        >;
        async fn update_contact_preferences(
            &self,
            request: tonic::Request<super::UpdateContactPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateContactPreferencesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ContactServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ContactServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ContactServiceServer<T>
    where
        T: ContactService,
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
                "/sdkwork.communication.app.v3.ContactService/ListContacts" => {
                    #[allow(non_camel_case_types)]
                    struct ListContactsSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::ListContactsRequest>
                    for ListContactsSvc<T> {
                        type Response = super::ListContactsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListContactsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::list_contacts(&inner, request).await
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
                        let method = ListContactsSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/ListContactTags" => {
                    #[allow(non_camel_case_types)]
                    struct ListContactTagsSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::ListContactTagsRequest>
                    for ListContactTagsSvc<T> {
                        type Response = super::ListContactTagsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListContactTagsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::list_contact_tags(&inner, request)
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
                        let method = ListContactTagsSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/CreateContactTag" => {
                    #[allow(non_camel_case_types)]
                    struct CreateContactTagSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::CreateContactTagRequest>
                    for CreateContactTagSvc<T> {
                        type Response = super::CreateContactTagResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateContactTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::create_contact_tag(&inner, request)
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
                        let method = CreateContactTagSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/UpdateContactTag" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateContactTagSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::UpdateContactTagRequest>
                    for UpdateContactTagSvc<T> {
                        type Response = super::UpdateContactTagResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateContactTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::update_contact_tag(&inner, request)
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
                        let method = UpdateContactTagSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/DeleteContactTag" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContactTagSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::DeleteContactTagRequest>
                    for DeleteContactTagSvc<T> {
                        type Response = super::DeleteContactTagResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteContactTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::delete_contact_tag(&inner, request)
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
                        let method = DeleteContactTagSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/CreateContactRecommendation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateContactRecommendationSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<
                        super::CreateContactRecommendationRequest,
                    > for CreateContactRecommendationSvc<T> {
                        type Response = super::CreateContactRecommendationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateContactRecommendationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::create_contact_recommendation(
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
                        let method = CreateContactRecommendationSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/RetrieveContactPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveContactPreferencesSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<
                        super::RetrieveContactPreferencesRequest,
                    > for RetrieveContactPreferencesSvc<T> {
                        type Response = super::RetrieveContactPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveContactPreferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::retrieve_contact_preferences(
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
                        let method = RetrieveContactPreferencesSvc(inner);
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
                "/sdkwork.communication.app.v3.ContactService/UpdateContactPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateContactPreferencesSvc<T: ContactService>(pub Arc<T>);
                    impl<
                        T: ContactService,
                    > tonic::server::UnaryService<super::UpdateContactPreferencesRequest>
                    for UpdateContactPreferencesSvc<T> {
                        type Response = super::UpdateContactPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateContactPreferencesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ContactService>::update_contact_preferences(
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
                        let method = UpdateContactPreferencesSvc(inner);
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
    impl<T> Clone for ContactServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.ContactService";
    impl<T> tonic::server::NamedService for ContactServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod message_service_client {
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
    pub struct MessageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MessageServiceClient<tonic::transport::Channel> {
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
    impl<T> MessageServiceClient<T>
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
        ) -> MessageServiceClient<InterceptedService<T, F>>
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
            MessageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_conversation_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConversationMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMessagesResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/ListConversationMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "ListConversationMessages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_conversation_message(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConversationMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConversationMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/CreateConversationMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "CreateConversationMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn publish_system_channel_message(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishSystemChannelMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PublishSystemChannelMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/PublishSystemChannelMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "PublishSystemChannelMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_message_interaction_summary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RetrieveMessageInteractionSummaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveMessageInteractionSummaryResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/RetrieveMessageInteractionSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "RetrieveMessageInteractionSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn edit_message(
            &mut self,
            request: impl tonic::IntoRequest<super::EditMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EditMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/EditMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "EditMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn recall_message(
            &mut self,
            request: impl tonic::IntoRequest<super::RecallMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecallMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/RecallMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "RecallMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_favorite_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFavoriteMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFavoriteMessagesResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/ListFavoriteMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "ListFavoriteMessages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_message_favorite(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMessageFavoriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMessageFavoriteResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/CreateMessageFavorite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "CreateMessageFavorite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_message_favorite(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMessageFavoriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageFavoriteResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageFavorite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "DeleteMessageFavorite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_message_visibility(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMessageVisibilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageVisibilityResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageVisibility",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "DeleteMessageVisibility",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_message_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMessageReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMessageReactionResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/CreateMessageReaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "CreateMessageReaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_message_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMessageReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageReactionResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageReaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "DeleteMessageReaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pin_message(
            &mut self,
            request: impl tonic::IntoRequest<super::PinMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PinMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/PinMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "PinMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpin_message(
            &mut self,
            request: impl tonic::IntoRequest<super::UnpinMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnpinMessageResponse>,
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
                "/sdkwork.communication.app.v3.MessageService/UnpinMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.MessageService",
                        "UnpinMessage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod message_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MessageServiceServer.
    #[async_trait]
    pub trait MessageService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_conversation_messages(
            &self,
            request: tonic::Request<super::ListConversationMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConversationMessagesResponse>,
            tonic::Status,
        >;
        async fn create_conversation_message(
            &self,
            request: tonic::Request<super::CreateConversationMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateConversationMessageResponse>,
            tonic::Status,
        >;
        async fn publish_system_channel_message(
            &self,
            request: tonic::Request<super::PublishSystemChannelMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PublishSystemChannelMessageResponse>,
            tonic::Status,
        >;
        async fn retrieve_message_interaction_summary(
            &self,
            request: tonic::Request<super::RetrieveMessageInteractionSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveMessageInteractionSummaryResponse>,
            tonic::Status,
        >;
        async fn edit_message(
            &self,
            request: tonic::Request<super::EditMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EditMessageResponse>,
            tonic::Status,
        >;
        async fn recall_message(
            &self,
            request: tonic::Request<super::RecallMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecallMessageResponse>,
            tonic::Status,
        >;
        async fn list_favorite_messages(
            &self,
            request: tonic::Request<super::ListFavoriteMessagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFavoriteMessagesResponse>,
            tonic::Status,
        >;
        async fn create_message_favorite(
            &self,
            request: tonic::Request<super::CreateMessageFavoriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMessageFavoriteResponse>,
            tonic::Status,
        >;
        async fn delete_message_favorite(
            &self,
            request: tonic::Request<super::DeleteMessageFavoriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageFavoriteResponse>,
            tonic::Status,
        >;
        async fn delete_message_visibility(
            &self,
            request: tonic::Request<super::DeleteMessageVisibilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageVisibilityResponse>,
            tonic::Status,
        >;
        async fn create_message_reaction(
            &self,
            request: tonic::Request<super::CreateMessageReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMessageReactionResponse>,
            tonic::Status,
        >;
        async fn delete_message_reaction(
            &self,
            request: tonic::Request<super::DeleteMessageReactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMessageReactionResponse>,
            tonic::Status,
        >;
        async fn pin_message(
            &self,
            request: tonic::Request<super::PinMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PinMessageResponse>,
            tonic::Status,
        >;
        async fn unpin_message(
            &self,
            request: tonic::Request<super::UnpinMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnpinMessageResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MessageServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MessageServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageServiceServer<T>
    where
        T: MessageService,
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
                "/sdkwork.communication.app.v3.MessageService/ListConversationMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ListConversationMessagesSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::ListConversationMessagesRequest>
                    for ListConversationMessagesSvc<T> {
                        type Response = super::ListConversationMessagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListConversationMessagesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::list_conversation_messages(
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
                        let method = ListConversationMessagesSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/CreateConversationMessage" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConversationMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<
                        super::CreateConversationMessageRequest,
                    > for CreateConversationMessageSvc<T> {
                        type Response = super::CreateConversationMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateConversationMessageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::create_conversation_message(
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
                        let method = CreateConversationMessageSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/PublishSystemChannelMessage" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSystemChannelMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<
                        super::PublishSystemChannelMessageRequest,
                    > for PublishSystemChannelMessageSvc<T> {
                        type Response = super::PublishSystemChannelMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PublishSystemChannelMessageRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::publish_system_channel_message(
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
                        let method = PublishSystemChannelMessageSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/RetrieveMessageInteractionSummary" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveMessageInteractionSummarySvc<T: MessageService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<
                        super::RetrieveMessageInteractionSummaryRequest,
                    > for RetrieveMessageInteractionSummarySvc<T> {
                        type Response = super::RetrieveMessageInteractionSummaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RetrieveMessageInteractionSummaryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::retrieve_message_interaction_summary(
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
                        let method = RetrieveMessageInteractionSummarySvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/EditMessage" => {
                    #[allow(non_camel_case_types)]
                    struct EditMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::EditMessageRequest>
                    for EditMessageSvc<T> {
                        type Response = super::EditMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::edit_message(&inner, request).await
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
                        let method = EditMessageSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/RecallMessage" => {
                    #[allow(non_camel_case_types)]
                    struct RecallMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::RecallMessageRequest>
                    for RecallMessageSvc<T> {
                        type Response = super::RecallMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecallMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::recall_message(&inner, request).await
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
                        let method = RecallMessageSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/ListFavoriteMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ListFavoriteMessagesSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::ListFavoriteMessagesRequest>
                    for ListFavoriteMessagesSvc<T> {
                        type Response = super::ListFavoriteMessagesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFavoriteMessagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::list_favorite_messages(
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
                        let method = ListFavoriteMessagesSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/CreateMessageFavorite" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMessageFavoriteSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::CreateMessageFavoriteRequest>
                    for CreateMessageFavoriteSvc<T> {
                        type Response = super::CreateMessageFavoriteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMessageFavoriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::create_message_favorite(
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
                        let method = CreateMessageFavoriteSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageFavorite" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMessageFavoriteSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::DeleteMessageFavoriteRequest>
                    for DeleteMessageFavoriteSvc<T> {
                        type Response = super::DeleteMessageFavoriteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMessageFavoriteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::delete_message_favorite(
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
                        let method = DeleteMessageFavoriteSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageVisibility" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMessageVisibilitySvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::DeleteMessageVisibilityRequest>
                    for DeleteMessageVisibilitySvc<T> {
                        type Response = super::DeleteMessageVisibilityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteMessageVisibilityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::delete_message_visibility(
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
                        let method = DeleteMessageVisibilitySvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/CreateMessageReaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMessageReactionSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::CreateMessageReactionRequest>
                    for CreateMessageReactionSvc<T> {
                        type Response = super::CreateMessageReactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMessageReactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::create_message_reaction(
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
                        let method = CreateMessageReactionSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/DeleteMessageReaction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMessageReactionSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::DeleteMessageReactionRequest>
                    for DeleteMessageReactionSvc<T> {
                        type Response = super::DeleteMessageReactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMessageReactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::delete_message_reaction(
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
                        let method = DeleteMessageReactionSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/PinMessage" => {
                    #[allow(non_camel_case_types)]
                    struct PinMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::PinMessageRequest>
                    for PinMessageSvc<T> {
                        type Response = super::PinMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PinMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::pin_message(&inner, request).await
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
                        let method = PinMessageSvc(inner);
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
                "/sdkwork.communication.app.v3.MessageService/UnpinMessage" => {
                    #[allow(non_camel_case_types)]
                    struct UnpinMessageSvc<T: MessageService>(pub Arc<T>);
                    impl<
                        T: MessageService,
                    > tonic::server::UnaryService<super::UnpinMessageRequest>
                    for UnpinMessageSvc<T> {
                        type Response = super::UnpinMessageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnpinMessageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MessageService>::unpin_message(&inner, request).await
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
                        let method = UnpinMessageSvc(inner);
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
    impl<T> Clone for MessageServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.MessageService";
    impl<T> tonic::server::NamedService for MessageServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod notification_service_client {
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
    pub struct NotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NotificationServiceClient<tonic::transport::Channel> {
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
    impl<T> NotificationServiceClient<T>
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
        ) -> NotificationServiceClient<InterceptedService<T, F>>
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
            NotificationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
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
                "/sdkwork.communication.app.v3.NotificationService/ListNotifications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.NotificationService",
                        "ListNotifications",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_notification_request(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateNotificationRequestResponse>,
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
                "/sdkwork.communication.app.v3.NotificationService/CreateNotificationRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.NotificationService",
                        "CreateNotificationRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveNotificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveNotificationResponse>,
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
                "/sdkwork.communication.app.v3.NotificationService/RetrieveNotification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.NotificationService",
                        "RetrieveNotification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchNotificationsResponse>>,
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
                "/sdkwork.communication.app.v3.NotificationService/WatchNotifications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.NotificationService",
                        "WatchNotifications",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod notification_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NotificationServiceServer.
    #[async_trait]
    pub trait NotificationService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_notifications(
            &self,
            request: tonic::Request<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
            tonic::Status,
        >;
        async fn create_notification_request(
            &self,
            request: tonic::Request<super::CreateNotificationRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateNotificationRequestResponse>,
            tonic::Status,
        >;
        async fn retrieve_notification(
            &self,
            request: tonic::Request<super::RetrieveNotificationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveNotificationResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchNotifications method.
        type WatchNotificationsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::WatchNotificationsResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn watch_notifications(
            &self,
            request: tonic::Request<super::WatchNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchNotificationsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct NotificationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> NotificationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NotificationServiceServer<T>
    where
        T: NotificationService,
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
                "/sdkwork.communication.app.v3.NotificationService/ListNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct ListNotificationsSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::ListNotificationsRequest>
                    for ListNotificationsSvc<T> {
                        type Response = super::ListNotificationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNotificationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::list_notifications(
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
                        let method = ListNotificationsSvc(inner);
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
                "/sdkwork.communication.app.v3.NotificationService/CreateNotificationRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNotificationRequestSvc<T: NotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<
                        super::CreateNotificationRequestRequest,
                    > for CreateNotificationRequestSvc<T> {
                        type Response = super::CreateNotificationRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateNotificationRequestRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::create_notification_request(
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
                        let method = CreateNotificationRequestSvc(inner);
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
                "/sdkwork.communication.app.v3.NotificationService/RetrieveNotification" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveNotificationSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::UnaryService<super::RetrieveNotificationRequest>
                    for RetrieveNotificationSvc<T> {
                        type Response = super::RetrieveNotificationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveNotificationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::retrieve_notification(
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
                        let method = RetrieveNotificationSvc(inner);
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
                "/sdkwork.communication.app.v3.NotificationService/WatchNotifications" => {
                    #[allow(non_camel_case_types)]
                    struct WatchNotificationsSvc<T: NotificationService>(pub Arc<T>);
                    impl<
                        T: NotificationService,
                    > tonic::server::ServerStreamingService<
                        super::WatchNotificationsRequest,
                    > for WatchNotificationsSvc<T> {
                        type Response = super::WatchNotificationsResponse;
                        type ResponseStream = T::WatchNotificationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchNotificationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as NotificationService>::watch_notifications(
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
                        let method = WatchNotificationsSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T> Clone for NotificationServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.NotificationService";
    impl<T> tonic::server::NamedService for NotificationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod presence_service_client {
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
    pub struct PresenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PresenceServiceClient<tonic::transport::Channel> {
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
    impl<T> PresenceServiceClient<T>
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
        ) -> PresenceServiceClient<InterceptedService<T, F>>
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
            PresenceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_presence_heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePresenceHeartbeatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePresenceHeartbeatResponse>,
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
                "/sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.PresenceService",
                        "CreatePresenceHeartbeat",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_my_presence(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveMyPresenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveMyPresenceResponse>,
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
                "/sdkwork.communication.app.v3.PresenceService/RetrieveMyPresence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.PresenceService",
                        "RetrieveMyPresence",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_presence(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchPresenceRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchPresenceResponse>>,
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
                "/sdkwork.communication.app.v3.PresenceService/WatchPresence",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.PresenceService",
                        "WatchPresence",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod presence_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PresenceServiceServer.
    #[async_trait]
    pub trait PresenceService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_presence_heartbeat(
            &self,
            request: tonic::Request<super::CreatePresenceHeartbeatRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePresenceHeartbeatResponse>,
            tonic::Status,
        >;
        async fn retrieve_my_presence(
            &self,
            request: tonic::Request<super::RetrieveMyPresenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveMyPresenceResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchPresence method.
        type WatchPresenceStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WatchPresenceResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn watch_presence(
            &self,
            request: tonic::Request<super::WatchPresenceRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchPresenceStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PresenceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> PresenceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PresenceServiceServer<T>
    where
        T: PresenceService,
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
                "/sdkwork.communication.app.v3.PresenceService/CreatePresenceHeartbeat" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePresenceHeartbeatSvc<T: PresenceService>(pub Arc<T>);
                    impl<
                        T: PresenceService,
                    > tonic::server::UnaryService<super::CreatePresenceHeartbeatRequest>
                    for CreatePresenceHeartbeatSvc<T> {
                        type Response = super::CreatePresenceHeartbeatResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreatePresenceHeartbeatRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PresenceService>::create_presence_heartbeat(
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
                        let method = CreatePresenceHeartbeatSvc(inner);
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
                "/sdkwork.communication.app.v3.PresenceService/RetrieveMyPresence" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveMyPresenceSvc<T: PresenceService>(pub Arc<T>);
                    impl<
                        T: PresenceService,
                    > tonic::server::UnaryService<super::RetrieveMyPresenceRequest>
                    for RetrieveMyPresenceSvc<T> {
                        type Response = super::RetrieveMyPresenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveMyPresenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PresenceService>::retrieve_my_presence(
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
                        let method = RetrieveMyPresenceSvc(inner);
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
                "/sdkwork.communication.app.v3.PresenceService/WatchPresence" => {
                    #[allow(non_camel_case_types)]
                    struct WatchPresenceSvc<T: PresenceService>(pub Arc<T>);
                    impl<
                        T: PresenceService,
                    > tonic::server::ServerStreamingService<super::WatchPresenceRequest>
                    for WatchPresenceSvc<T> {
                        type Response = super::WatchPresenceResponse;
                        type ResponseStream = T::WatchPresenceStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchPresenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as PresenceService>::watch_presence(&inner, request)
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
                        let method = WatchPresenceSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T> Clone for PresenceServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.PresenceService";
    impl<T> tonic::server::NamedService for PresenceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod realtime_service_client {
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
    pub struct RealtimeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RealtimeServiceClient<tonic::transport::Channel> {
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
    impl<T> RealtimeServiceClient<T>
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
        ) -> RealtimeServiceClient<InterceptedService<T, F>>
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
            RealtimeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn sync_realtime_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRealtimeSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncRealtimeSubscriptionsResponse>,
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
                "/sdkwork.communication.app.v3.RealtimeService/SyncRealtimeSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RealtimeService",
                        "SyncRealtimeSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn ack_realtime_events(
            &mut self,
            request: impl tonic::IntoRequest<super::AckRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AckRealtimeEventsResponse>,
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
                "/sdkwork.communication.app.v3.RealtimeService/AckRealtimeEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RealtimeService",
                        "AckRealtimeEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_realtime_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRealtimeEventsResponse>,
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
                "/sdkwork.communication.app.v3.RealtimeService/ListRealtimeEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RealtimeService",
                        "ListRealtimeEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_realtime_events(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchRealtimeEventsResponse>>,
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
                "/sdkwork.communication.app.v3.RealtimeService/WatchRealtimeEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RealtimeService",
                        "WatchRealtimeEvents",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod realtime_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RealtimeServiceServer.
    #[async_trait]
    pub trait RealtimeService: std::marker::Send + std::marker::Sync + 'static {
        async fn sync_realtime_subscriptions(
            &self,
            request: tonic::Request<super::SyncRealtimeSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SyncRealtimeSubscriptionsResponse>,
            tonic::Status,
        >;
        async fn ack_realtime_events(
            &self,
            request: tonic::Request<super::AckRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AckRealtimeEventsResponse>,
            tonic::Status,
        >;
        async fn list_realtime_events(
            &self,
            request: tonic::Request<super::ListRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRealtimeEventsResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchRealtimeEvents method.
        type WatchRealtimeEventsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::WatchRealtimeEventsResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn watch_realtime_events(
            &self,
            request: tonic::Request<super::WatchRealtimeEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchRealtimeEventsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RealtimeServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RealtimeServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RealtimeServiceServer<T>
    where
        T: RealtimeService,
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
                "/sdkwork.communication.app.v3.RealtimeService/SyncRealtimeSubscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct SyncRealtimeSubscriptionsSvc<T: RealtimeService>(pub Arc<T>);
                    impl<
                        T: RealtimeService,
                    > tonic::server::UnaryService<
                        super::SyncRealtimeSubscriptionsRequest,
                    > for SyncRealtimeSubscriptionsSvc<T> {
                        type Response = super::SyncRealtimeSubscriptionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SyncRealtimeSubscriptionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeService>::sync_realtime_subscriptions(
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
                        let method = SyncRealtimeSubscriptionsSvc(inner);
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
                "/sdkwork.communication.app.v3.RealtimeService/AckRealtimeEvents" => {
                    #[allow(non_camel_case_types)]
                    struct AckRealtimeEventsSvc<T: RealtimeService>(pub Arc<T>);
                    impl<
                        T: RealtimeService,
                    > tonic::server::UnaryService<super::AckRealtimeEventsRequest>
                    for AckRealtimeEventsSvc<T> {
                        type Response = super::AckRealtimeEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AckRealtimeEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeService>::ack_realtime_events(&inner, request)
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
                        let method = AckRealtimeEventsSvc(inner);
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
                "/sdkwork.communication.app.v3.RealtimeService/ListRealtimeEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListRealtimeEventsSvc<T: RealtimeService>(pub Arc<T>);
                    impl<
                        T: RealtimeService,
                    > tonic::server::UnaryService<super::ListRealtimeEventsRequest>
                    for ListRealtimeEventsSvc<T> {
                        type Response = super::ListRealtimeEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRealtimeEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeService>::list_realtime_events(
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
                        let method = ListRealtimeEventsSvc(inner);
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
                "/sdkwork.communication.app.v3.RealtimeService/WatchRealtimeEvents" => {
                    #[allow(non_camel_case_types)]
                    struct WatchRealtimeEventsSvc<T: RealtimeService>(pub Arc<T>);
                    impl<
                        T: RealtimeService,
                    > tonic::server::ServerStreamingService<
                        super::WatchRealtimeEventsRequest,
                    > for WatchRealtimeEventsSvc<T> {
                        type Response = super::WatchRealtimeEventsResponse;
                        type ResponseStream = T::WatchRealtimeEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchRealtimeEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RealtimeService>::watch_realtime_events(
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
                        let method = WatchRealtimeEventsSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T> Clone for RealtimeServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.RealtimeService";
    impl<T> tonic::server::NamedService for RealtimeServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod room_service_client {
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
    pub struct RoomServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RoomServiceClient<tonic::transport::Channel> {
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
    impl<T> RoomServiceClient<T>
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
        ) -> RoomServiceClient<InterceptedService<T, F>>
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
            RoomServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_room(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRoomResponse>,
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
                "/sdkwork.communication.app.v3.RoomService/CreateRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RoomService",
                        "CreateRoom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn retrieve_room(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveRoomResponse>,
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
                "/sdkwork.communication.app.v3.RoomService/RetrieveRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RoomService",
                        "RetrieveRoom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn enter_room(
            &mut self,
            request: impl tonic::IntoRequest<super::EnterRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnterRoomResponse>,
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
                "/sdkwork.communication.app.v3.RoomService/EnterRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RoomService",
                        "EnterRoom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn leave_room(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeaveRoomResponse>,
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
                "/sdkwork.communication.app.v3.RoomService/LeaveRoom",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.RoomService",
                        "LeaveRoom",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod room_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RoomServiceServer.
    #[async_trait]
    pub trait RoomService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_room(
            &self,
            request: tonic::Request<super::CreateRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRoomResponse>,
            tonic::Status,
        >;
        async fn retrieve_room(
            &self,
            request: tonic::Request<super::RetrieveRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveRoomResponse>,
            tonic::Status,
        >;
        async fn enter_room(
            &self,
            request: tonic::Request<super::EnterRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EnterRoomResponse>,
            tonic::Status,
        >;
        async fn leave_room(
            &self,
            request: tonic::Request<super::LeaveRoomRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeaveRoomResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RoomServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RoomServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RoomServiceServer<T>
    where
        T: RoomService,
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
                "/sdkwork.communication.app.v3.RoomService/CreateRoom" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRoomSvc<T: RoomService>(pub Arc<T>);
                    impl<
                        T: RoomService,
                    > tonic::server::UnaryService<super::CreateRoomRequest>
                    for CreateRoomSvc<T> {
                        type Response = super::CreateRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRoomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoomService>::create_room(&inner, request).await
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
                        let method = CreateRoomSvc(inner);
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
                "/sdkwork.communication.app.v3.RoomService/RetrieveRoom" => {
                    #[allow(non_camel_case_types)]
                    struct RetrieveRoomSvc<T: RoomService>(pub Arc<T>);
                    impl<
                        T: RoomService,
                    > tonic::server::UnaryService<super::RetrieveRoomRequest>
                    for RetrieveRoomSvc<T> {
                        type Response = super::RetrieveRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetrieveRoomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoomService>::retrieve_room(&inner, request).await
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
                        let method = RetrieveRoomSvc(inner);
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
                "/sdkwork.communication.app.v3.RoomService/EnterRoom" => {
                    #[allow(non_camel_case_types)]
                    struct EnterRoomSvc<T: RoomService>(pub Arc<T>);
                    impl<
                        T: RoomService,
                    > tonic::server::UnaryService<super::EnterRoomRequest>
                    for EnterRoomSvc<T> {
                        type Response = super::EnterRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnterRoomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoomService>::enter_room(&inner, request).await
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
                        let method = EnterRoomSvc(inner);
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
                "/sdkwork.communication.app.v3.RoomService/LeaveRoom" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveRoomSvc<T: RoomService>(pub Arc<T>);
                    impl<
                        T: RoomService,
                    > tonic::server::UnaryService<super::LeaveRoomRequest>
                    for LeaveRoomSvc<T> {
                        type Response = super::LeaveRoomResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaveRoomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoomService>::leave_room(&inner, request).await
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
                        let method = LeaveRoomSvc(inner);
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
    impl<T> Clone for RoomServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.RoomService";
    impl<T> tonic::server::NamedService for RoomServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod social_service_client {
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
    pub struct SocialServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SocialServiceClient<tonic::transport::Channel> {
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
    impl<T> SocialServiceClient<T>
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
        ) -> SocialServiceClient<InterceptedService<T, F>>
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
            SocialServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_social_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSocialUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSocialUsersResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/ListSocialUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "ListSocialUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_friend_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFriendRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFriendRequestsResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/ListFriendRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "ListFriendRequests",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFriendRequestResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/CreateFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "CreateFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptFriendRequestResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/AcceptFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "AcceptFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn decline_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::DeclineFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeclineFriendRequestResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/DeclineFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "DeclineFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_friend_request(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelFriendRequestResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/CancelFriendRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "CancelFriendRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_friendship(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFriendshipResponse>,
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
                "/sdkwork.communication.app.v3.SocialService/RemoveFriendship",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.SocialService",
                        "RemoveFriendship",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod social_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SocialServiceServer.
    #[async_trait]
    pub trait SocialService: std::marker::Send + std::marker::Sync + 'static {
        async fn list_social_users(
            &self,
            request: tonic::Request<super::ListSocialUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSocialUsersResponse>,
            tonic::Status,
        >;
        async fn list_friend_requests(
            &self,
            request: tonic::Request<super::ListFriendRequestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFriendRequestsResponse>,
            tonic::Status,
        >;
        async fn create_friend_request(
            &self,
            request: tonic::Request<super::CreateFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFriendRequestResponse>,
            tonic::Status,
        >;
        async fn accept_friend_request(
            &self,
            request: tonic::Request<super::AcceptFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptFriendRequestResponse>,
            tonic::Status,
        >;
        async fn decline_friend_request(
            &self,
            request: tonic::Request<super::DeclineFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeclineFriendRequestResponse>,
            tonic::Status,
        >;
        async fn cancel_friend_request(
            &self,
            request: tonic::Request<super::CancelFriendRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelFriendRequestResponse>,
            tonic::Status,
        >;
        async fn remove_friendship(
            &self,
            request: tonic::Request<super::RemoveFriendshipRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveFriendshipResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SocialServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SocialServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SocialServiceServer<T>
    where
        T: SocialService,
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
                "/sdkwork.communication.app.v3.SocialService/ListSocialUsers" => {
                    #[allow(non_camel_case_types)]
                    struct ListSocialUsersSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::ListSocialUsersRequest>
                    for ListSocialUsersSvc<T> {
                        type Response = super::ListSocialUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSocialUsersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::list_social_users(&inner, request)
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
                        let method = ListSocialUsersSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/ListFriendRequests" => {
                    #[allow(non_camel_case_types)]
                    struct ListFriendRequestsSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::ListFriendRequestsRequest>
                    for ListFriendRequestsSvc<T> {
                        type Response = super::ListFriendRequestsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFriendRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::list_friend_requests(&inner, request)
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
                        let method = ListFriendRequestsSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/CreateFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFriendRequestSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::CreateFriendRequestRequest>
                    for CreateFriendRequestSvc<T> {
                        type Response = super::CreateFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFriendRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::create_friend_request(&inner, request)
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
                        let method = CreateFriendRequestSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/AcceptFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptFriendRequestSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::AcceptFriendRequestRequest>
                    for AcceptFriendRequestSvc<T> {
                        type Response = super::AcceptFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptFriendRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::accept_friend_request(&inner, request)
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
                        let method = AcceptFriendRequestSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/DeclineFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct DeclineFriendRequestSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::DeclineFriendRequestRequest>
                    for DeclineFriendRequestSvc<T> {
                        type Response = super::DeclineFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeclineFriendRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::decline_friend_request(
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
                        let method = DeclineFriendRequestSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/CancelFriendRequest" => {
                    #[allow(non_camel_case_types)]
                    struct CancelFriendRequestSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::CancelFriendRequestRequest>
                    for CancelFriendRequestSvc<T> {
                        type Response = super::CancelFriendRequestResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelFriendRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::cancel_friend_request(&inner, request)
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
                        let method = CancelFriendRequestSvc(inner);
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
                "/sdkwork.communication.app.v3.SocialService/RemoveFriendship" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFriendshipSvc<T: SocialService>(pub Arc<T>);
                    impl<
                        T: SocialService,
                    > tonic::server::UnaryService<super::RemoveFriendshipRequest>
                    for RemoveFriendshipSvc<T> {
                        type Response = super::RemoveFriendshipResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFriendshipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SocialService>::remove_friendship(&inner, request)
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
                        let method = RemoveFriendshipSvc(inner);
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
    impl<T> Clone for SocialServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.SocialService";
    impl<T> tonic::server::NamedService for SocialServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod stream_service_client {
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
    pub struct StreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StreamServiceClient<tonic::transport::Channel> {
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
    impl<T> StreamServiceClient<T>
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
        ) -> StreamServiceClient<InterceptedService<T, F>>
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
            StreamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/CreateStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "CreateStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_stream_frames(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStreamFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStreamFramesResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/ListStreamFrames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "ListStreamFrames",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn append_stream_frame(
            &mut self,
            request: impl tonic::IntoRequest<super::AppendStreamFrameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AppendStreamFrameResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/AppendStreamFrame",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "AppendStreamFrame",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_stream_checkpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStreamCheckpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamCheckpointResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/CreateStreamCheckpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "CreateStreamCheckpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteStreamResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/CompleteStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "CompleteStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn abort_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::AbortStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AbortStreamResponse>,
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
                "/sdkwork.communication.app.v3.StreamService/AbortStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "AbortStream",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_stream_frames(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchStreamFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchStreamFramesResponse>>,
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
                "/sdkwork.communication.app.v3.StreamService/WatchStreamFrames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "sdkwork.communication.app.v3.StreamService",
                        "WatchStreamFrames",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod stream_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StreamServiceServer.
    #[async_trait]
    pub trait StreamService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_stream(
            &self,
            request: tonic::Request<super::CreateStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamResponse>,
            tonic::Status,
        >;
        async fn list_stream_frames(
            &self,
            request: tonic::Request<super::ListStreamFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStreamFramesResponse>,
            tonic::Status,
        >;
        async fn append_stream_frame(
            &self,
            request: tonic::Request<super::AppendStreamFrameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AppendStreamFrameResponse>,
            tonic::Status,
        >;
        async fn create_stream_checkpoint(
            &self,
            request: tonic::Request<super::CreateStreamCheckpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamCheckpointResponse>,
            tonic::Status,
        >;
        async fn complete_stream(
            &self,
            request: tonic::Request<super::CompleteStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteStreamResponse>,
            tonic::Status,
        >;
        async fn abort_stream(
            &self,
            request: tonic::Request<super::AbortStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AbortStreamResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchStreamFrames method.
        type WatchStreamFramesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::WatchStreamFramesResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn watch_stream_frames(
            &self,
            request: tonic::Request<super::WatchStreamFramesRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchStreamFramesStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StreamServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StreamServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StreamServiceServer<T>
    where
        T: StreamService,
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
                "/sdkwork.communication.app.v3.StreamService/CreateStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStreamSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::CreateStreamRequest>
                    for CreateStreamSvc<T> {
                        type Response = super::CreateStreamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::create_stream(&inner, request).await
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
                        let method = CreateStreamSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/ListStreamFrames" => {
                    #[allow(non_camel_case_types)]
                    struct ListStreamFramesSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::ListStreamFramesRequest>
                    for ListStreamFramesSvc<T> {
                        type Response = super::ListStreamFramesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListStreamFramesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::list_stream_frames(&inner, request)
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
                        let method = ListStreamFramesSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/AppendStreamFrame" => {
                    #[allow(non_camel_case_types)]
                    struct AppendStreamFrameSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::AppendStreamFrameRequest>
                    for AppendStreamFrameSvc<T> {
                        type Response = super::AppendStreamFrameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AppendStreamFrameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::append_stream_frame(&inner, request)
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
                        let method = AppendStreamFrameSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/CreateStreamCheckpoint" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStreamCheckpointSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::CreateStreamCheckpointRequest>
                    for CreateStreamCheckpointSvc<T> {
                        type Response = super::CreateStreamCheckpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStreamCheckpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::create_stream_checkpoint(
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
                        let method = CreateStreamCheckpointSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/CompleteStream" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteStreamSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::CompleteStreamRequest>
                    for CompleteStreamSvc<T> {
                        type Response = super::CompleteStreamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::complete_stream(&inner, request).await
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
                        let method = CompleteStreamSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/AbortStream" => {
                    #[allow(non_camel_case_types)]
                    struct AbortStreamSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::UnaryService<super::AbortStreamRequest>
                    for AbortStreamSvc<T> {
                        type Response = super::AbortStreamResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AbortStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::abort_stream(&inner, request).await
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
                        let method = AbortStreamSvc(inner);
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
                "/sdkwork.communication.app.v3.StreamService/WatchStreamFrames" => {
                    #[allow(non_camel_case_types)]
                    struct WatchStreamFramesSvc<T: StreamService>(pub Arc<T>);
                    impl<
                        T: StreamService,
                    > tonic::server::ServerStreamingService<
                        super::WatchStreamFramesRequest,
                    > for WatchStreamFramesSvc<T> {
                        type Response = super::WatchStreamFramesResponse;
                        type ResponseStream = T::WatchStreamFramesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchStreamFramesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StreamService>::watch_stream_frames(&inner, request)
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
                        let method = WatchStreamFramesSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T> Clone for StreamServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "sdkwork.communication.app.v3.StreamService";
    impl<T> tonic::server::NamedService for StreamServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
