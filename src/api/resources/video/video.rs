use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VideoClient {
    pub http_client: HttpClient,
}

impl VideoClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of active egress stream objects.
    /// Results are ordered by egress stream create time. If the
    /// egress backend is unreachable, the listed streams might be stale or degraded.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Desired number of egress streams per page. Defaults to 50 if left blank,
    /// and capped at 100. The response may contain fewer than max page size.
    /// * `page_token` - To retrieve the next page, pass the `nextPageToken` from the previous
    /// response. Leave empty for the first page.
    ///
    /// Keep the rest of the request identical between pages, otherwise the
    /// server may reject it.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .list_egress_streams(
    ///             &ListEgressStreamsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_egress_streams(
        &self,
        request: &ListEgressStreamsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEgressStreamsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v1/video/egress_streams",
                None,
                QueryBuilder::new()
                    .int("pageSize", request.page_size.clone())
                    .string("pageToken", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an egress stream that publishes a live stream to a downstream consumer.
    /// A stream in `STREAM_STATUS_UNAVAILABLE` is rejected as not-live.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .create_egress_stream(
    ///             &CreateEgressStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_egress_stream(
        &self,
        request: &CreateEgressStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateEgressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/video/egress_streams",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an egress stream object and its associated metadata.
    ///
    /// # Arguments
    ///
    /// * `egress_id` - Identifier of the egress stream to retrieve.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .get_egress_stream(&"egressId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_egress_stream(
        &self,
        egress_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetEgressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/video/egress_streams/{}", egress_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes the egress stream for a live stream. Returns `NOT_FOUND` if no matching active
    /// egress stream exists.
    ///
    /// # Arguments
    ///
    /// * `egress_id` - Identifier of the egress stream to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .delete_egress_stream(&"egressId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_egress_stream(
        &self,
        egress_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteEgressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v1/video/egress_streams/{}", egress_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a list of top level ingress stream objects, including ingress streams and internal
    /// Anduril streams. Will only return active streams.
    /// Results are ordered by ingress stream create time.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Desired number of ingress streams per page. Defaults to 50 if left blank,
    /// and capped at 100. The response may contain fewer than requested.
    /// * `page_token` - To retrieve the next page, pass the `nextPageToken` from the previous
    /// response. Leave empty for the first page.
    ///
    /// Keep the rest of the request identical between pages, otherwise the
    /// server may reject it.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .list_ingress_streams(
    ///             &ListIngressStreamsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_ingress_streams(
        &self,
        request: &ListIngressStreamsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListIngressStreamsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v1/video/ingress_streams",
                None,
                QueryBuilder::new()
                    .int("pageSize", request.page_size.clone())
                    .string("pageToken", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a video ingress stream, returning metadata that you can use to stream live video to
    /// Lattice. Exactly one of `rtsp` or `srt` must be set on the request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .create_ingress_stream(
    ///             &CreateIngressStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_ingress_stream(
        &self,
        request: &CreateIngressStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateIngressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/video/ingress_streams",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a top level ingress stream object and its associated metadata. This includes
    /// ingress streams and internal Anduril streams.
    ///
    /// # Arguments
    ///
    /// * `ingress_id` - Identifier of the ingress stream to retrieve.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .get_ingress_stream(&"ingressId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_ingress_stream(
        &self,
        ingress_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetIngressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/video/ingress_streams/{}", ingress_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a video ingress stream and transitions the stream to `STREAM_STATUS_ARCHIVED`.
    /// Any egress streams consuming this stream will be stopped automatically.
    ///
    /// # Arguments
    ///
    /// * `ingress_id` - Identifier of the ingress stream to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .video
    ///         .delete_ingress_stream(&"ingressId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_ingress_stream(
        &self,
        ingress_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteIngressStreamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v1/video/ingress_streams/{}", ingress_id),
                None,
                None,
                options,
            )
            .await
    }
}
