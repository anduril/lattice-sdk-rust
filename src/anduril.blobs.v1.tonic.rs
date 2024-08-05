// @generated
/// Generated client implementations.
pub mod blobs_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BlobsApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BlobsApiClient<tonic::transport::Channel> {
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
    impl<T> BlobsApiClient<T>
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
        ) -> BlobsApiClient<InterceptedService<T, F>>
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
            BlobsApiClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBlobResponse>,
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
                "/anduril.blobs.v1.BlobsAPI/CreateBlob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("anduril.blobs.v1.BlobsAPI", "CreateBlob"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_blob_chunked(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::CreateBlobChunkedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateBlobChunkedResponse>,
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
                "/anduril.blobs.v1.BlobsAPI/CreateBlobChunked",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("anduril.blobs.v1.BlobsAPI", "CreateBlobChunked"),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn get_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GetBlobResponse>>,
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
                "/anduril.blobs.v1.BlobsAPI/GetBlob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("anduril.blobs.v1.BlobsAPI", "GetBlob"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn head_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::HeadBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeadBlobResponse>,
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
                "/anduril.blobs.v1.BlobsAPI/HeadBlob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("anduril.blobs.v1.BlobsAPI", "HeadBlob"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_blob_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamBlobMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamBlobMetadataResponse>>,
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
                "/anduril.blobs.v1.BlobsAPI/StreamBlobMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("anduril.blobs.v1.BlobsAPI", "StreamBlobMetadata"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod blobs_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BlobsApiServer.
    #[async_trait]
    pub trait BlobsApi: Send + Sync + 'static {
        async fn create_blob(
            &self,
            request: tonic::Request<super::CreateBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBlobResponse>,
            tonic::Status,
        >;
        async fn create_blob_chunked(
            &self,
            request: tonic::Request<tonic::Streaming<super::CreateBlobChunkedRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBlobChunkedResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetBlob method.
        type GetBlobStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::GetBlobResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_blob(
            &self,
            request: tonic::Request<super::GetBlobRequest>,
        ) -> std::result::Result<tonic::Response<Self::GetBlobStream>, tonic::Status>;
        async fn head_blob(
            &self,
            request: tonic::Request<super::HeadBlobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeadBlobResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamBlobMetadata method.
        type StreamBlobMetadataStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamBlobMetadataResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_blob_metadata(
            &self,
            request: tonic::Request<super::StreamBlobMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamBlobMetadataStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BlobsApiServer<T: BlobsApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BlobsApi> BlobsApiServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BlobsApiServer<T>
    where
        T: BlobsApi,
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
                "/anduril.blobs.v1.BlobsAPI/CreateBlob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBlobSvc<T: BlobsApi>(pub Arc<T>);
                    impl<
                        T: BlobsApi,
                    > tonic::server::UnaryService<super::CreateBlobRequest>
                    for CreateBlobSvc<T> {
                        type Response = super::CreateBlobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBlobRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BlobsApi>::create_blob(&inner, request).await
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
                        let method = CreateBlobSvc(inner);
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
                "/anduril.blobs.v1.BlobsAPI/CreateBlobChunked" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBlobChunkedSvc<T: BlobsApi>(pub Arc<T>);
                    impl<
                        T: BlobsApi,
                    > tonic::server::ClientStreamingService<
                        super::CreateBlobChunkedRequest,
                    > for CreateBlobChunkedSvc<T> {
                        type Response = super::CreateBlobChunkedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::CreateBlobChunkedRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BlobsApi>::create_blob_chunked(&inner, request).await
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
                        let method = CreateBlobChunkedSvc(inner);
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
                "/anduril.blobs.v1.BlobsAPI/GetBlob" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlobSvc<T: BlobsApi>(pub Arc<T>);
                    impl<
                        T: BlobsApi,
                    > tonic::server::ServerStreamingService<super::GetBlobRequest>
                    for GetBlobSvc<T> {
                        type Response = super::GetBlobResponse;
                        type ResponseStream = T::GetBlobStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlobRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BlobsApi>::get_blob(&inner, request).await
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
                        let method = GetBlobSvc(inner);
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
                "/anduril.blobs.v1.BlobsAPI/HeadBlob" => {
                    #[allow(non_camel_case_types)]
                    struct HeadBlobSvc<T: BlobsApi>(pub Arc<T>);
                    impl<T: BlobsApi> tonic::server::UnaryService<super::HeadBlobRequest>
                    for HeadBlobSvc<T> {
                        type Response = super::HeadBlobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeadBlobRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BlobsApi>::head_blob(&inner, request).await
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
                        let method = HeadBlobSvc(inner);
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
                "/anduril.blobs.v1.BlobsAPI/StreamBlobMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct StreamBlobMetadataSvc<T: BlobsApi>(pub Arc<T>);
                    impl<
                        T: BlobsApi,
                    > tonic::server::ServerStreamingService<
                        super::StreamBlobMetadataRequest,
                    > for StreamBlobMetadataSvc<T> {
                        type Response = super::StreamBlobMetadataResponse;
                        type ResponseStream = T::StreamBlobMetadataStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamBlobMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BlobsApi>::stream_blob_metadata(&inner, request).await
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
                        let method = StreamBlobMetadataSvc(inner);
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
    impl<T: BlobsApi> Clone for BlobsApiServer<T> {
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
    impl<T: BlobsApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BlobsApi> tonic::server::NamedService for BlobsApiServer<T> {
        const NAME: &'static str = "anduril.blobs.v1.BlobsAPI";
    }
}
