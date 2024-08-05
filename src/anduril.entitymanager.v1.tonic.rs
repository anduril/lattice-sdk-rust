// @generated
/// Generated client implementations.
pub mod entity_manager_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EntityManagerApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EntityManagerApiClient<tonic::transport::Channel> {
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
    impl<T> EntityManagerApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> EntityManagerApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EntityManagerApiClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEntityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/GetEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "GetEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_entity_components(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamEntityComponentsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamEntityComponentsResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/StreamEntityComponents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "StreamEntityComponents",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn put_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::PutEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutEntityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/PutEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "PutEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn publish_entities(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PublishEntitiesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::PublishEntitiesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/PublishEntities",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "PublishEntities",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn override_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::OverrideEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OverrideEntityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/OverrideEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "OverrideEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_entity_override(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveEntityOverrideRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveEntityOverrideResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/RemoveEntityOverride",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "RemoveEntityOverride",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEntityResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/anduril.entitymanager.v1.EntityManagerAPI/DeleteEntity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "anduril.entitymanager.v1.EntityManagerAPI",
                        "DeleteEntity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod entity_manager_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EntityManagerApiServer.
    #[async_trait]
    pub trait EntityManagerApi: Send + Sync + 'static {
        async fn get_entity(
            &self,
            request: tonic::Request<super::GetEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEntityResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamEntityComponents method.
        type StreamEntityComponentsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamEntityComponentsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_entity_components(
            &self,
            request: tonic::Request<super::StreamEntityComponentsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamEntityComponentsStream>,
            tonic::Status,
        >;
        async fn put_entity(
            &self,
            request: tonic::Request<super::PutEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutEntityResponse>,
            tonic::Status,
        >;
        async fn publish_entities(
            &self,
            request: tonic::Request<tonic::Streaming<super::PublishEntitiesRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::PublishEntitiesResponse>,
            tonic::Status,
        >;
        async fn override_entity(
            &self,
            request: tonic::Request<super::OverrideEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OverrideEntityResponse>,
            tonic::Status,
        >;
        async fn remove_entity_override(
            &self,
            request: tonic::Request<super::RemoveEntityOverrideRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveEntityOverrideResponse>,
            tonic::Status,
        >;
        async fn delete_entity(
            &self,
            request: tonic::Request<super::DeleteEntityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEntityResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct EntityManagerApiServer<T: EntityManagerApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EntityManagerApi> EntityManagerApiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EntityManagerApiServer<T>
    where
        T: EntityManagerApi,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/anduril.entitymanager.v1.EntityManagerAPI/GetEntity" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntitySvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::UnaryService<super::GetEntityRequest>
                    for GetEntitySvc<T> {
                        type Response = super::GetEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEntityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::get_entity(&inner, request).await
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
                        let inner = inner.0;
                        let method = GetEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                "/anduril.entitymanager.v1.EntityManagerAPI/StreamEntityComponents" => {
                    #[allow(non_camel_case_types)]
                    struct StreamEntityComponentsSvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::ServerStreamingService<
                        super::StreamEntityComponentsRequest,
                    > for StreamEntityComponentsSvc<T> {
                        type Response = super::StreamEntityComponentsResponse;
                        type ResponseStream = T::StreamEntityComponentsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamEntityComponentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::stream_entity_components(
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
                        let inner = inner.0;
                        let method = StreamEntityComponentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                "/anduril.entitymanager.v1.EntityManagerAPI/PutEntity" => {
                    #[allow(non_camel_case_types)]
                    struct PutEntitySvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::UnaryService<super::PutEntityRequest>
                    for PutEntitySvc<T> {
                        type Response = super::PutEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutEntityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::put_entity(&inner, request).await
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
                        let inner = inner.0;
                        let method = PutEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                "/anduril.entitymanager.v1.EntityManagerAPI/PublishEntities" => {
                    #[allow(non_camel_case_types)]
                    struct PublishEntitiesSvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::ClientStreamingService<
                        super::PublishEntitiesRequest,
                    > for PublishEntitiesSvc<T> {
                        type Response = super::PublishEntitiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PublishEntitiesRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::publish_entities(&inner, request)
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
                        let inner = inner.0;
                        let method = PublishEntitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/anduril.entitymanager.v1.EntityManagerAPI/OverrideEntity" => {
                    #[allow(non_camel_case_types)]
                    struct OverrideEntitySvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::UnaryService<super::OverrideEntityRequest>
                    for OverrideEntitySvc<T> {
                        type Response = super::OverrideEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OverrideEntityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::override_entity(&inner, request)
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
                        let inner = inner.0;
                        let method = OverrideEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                "/anduril.entitymanager.v1.EntityManagerAPI/RemoveEntityOverride" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveEntityOverrideSvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::UnaryService<super::RemoveEntityOverrideRequest>
                    for RemoveEntityOverrideSvc<T> {
                        type Response = super::RemoveEntityOverrideResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveEntityOverrideRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::remove_entity_override(
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
                        let inner = inner.0;
                        let method = RemoveEntityOverrideSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                "/anduril.entitymanager.v1.EntityManagerAPI/DeleteEntity" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEntitySvc<T: EntityManagerApi>(pub Arc<T>);
                    impl<
                        T: EntityManagerApi,
                    > tonic::server::UnaryService<super::DeleteEntityRequest>
                    for DeleteEntitySvc<T> {
                        type Response = super::DeleteEntityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEntityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EntityManagerApi>::delete_entity(&inner, request)
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
                        let inner = inner.0;
                        let method = DeleteEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: EntityManagerApi> Clone for EntityManagerApiServer<T> {
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
    impl<T: EntityManagerApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EntityManagerApi> tonic::server::NamedService for EntityManagerApiServer<T> {
        const NAME: &'static str = "anduril.entitymanager.v1.EntityManagerAPI";
    }
}
