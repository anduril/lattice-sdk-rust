use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ObjectsClient {
    pub http_client: HttpClient,
}

impl ObjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists objects in your environment. You can define a prefix to list a subset of your objects. If you do not set a prefix, Lattice returns all available objects. By default this endpoint will list local objects only.
    ///
    /// # Arguments
    ///
    /// * `prefix` - Filters the objects based on the specified prefix path. If no path is specified, all objects are returned.
    /// * `since_timestamp` - Filters out objects whose `last_updated_at` is earlier than this timestamp.
    ///
    /// `last_updated_at` records when an object arrived on the node that holds it, so this filter selects objects that arrived since the given time. It is not the time the object was authored: a copy that reaches a node later carries the later arrival time.
    /// * `page_token` - Opaque cursor for continuing the same list request. Start a new listing without the previous cursor if any query parameter or listing scope changes.
    /// * `all_objects_in_mesh` - Lists objects across all environment nodes in a Lattice Mesh. When false or unset, only objects held by the local node are returned.
    /// * `max_page_size` - Sets the maximum number of items that should be returned on a single page.
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
    ///         .objects
    ///         .list_objects(
    ///             &ListObjectsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_objects(
        &self,
        request: &ListObjectsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api/v1/objects",
                None,
                QueryBuilder::new()
                    .string("prefix", request.prefix.clone())
                    .datetime("sinceTimestamp", request.since_timestamp.clone())
                    .string("pageToken", request.page_token.clone())
                    .bool("allObjectsInMesh", request.all_objects_in_mesh.clone())
                    .int("maxPageSize", request.max_page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_objects_paginated(
        &self,
        request: &ListObjectsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("prefix", request.prefix.clone())
            .datetime("sinceTimestamp", request.since_timestamp.clone())
            .bool("allObjectsInMesh", request.all_objects_in_mesh.clone())
            .int("maxPageSize", request.max_page_size.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("pageToken".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "api/v1/objects",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("path_metadatas")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("next_page_token")
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Fetches an object from your environment using the objectPath path parameter.
    ///
    /// # Arguments
    ///
    /// * `object_path` - The path of the object to fetch.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .objects
    ///         .get_object(&"objectPath".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_object(
        &self,
        object_path: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("api/v1/objects/{}", object_path),
                None,
                None,
                options,
            )
            .await
    }

    /// Uploads an object. The object must be 1 GiB or smaller.
    ///
    /// # Arguments
    ///
    /// * `object_path` - Path of the Object that is to be uploaded.
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
    ///         .objects
    ///         .upload_object(&"objectPath".to_string(), &vec![], None)
    ///         .await;
    /// }
    /// ```
    pub async fn upload_object(
        &self,
        object_path: &str,
        request: &Vec<u8>,
        options: Option<RequestOptions>,
    ) -> Result<PathMetadata, ApiError> {
        self.http_client
            .execute_bytes_request(
                Method::POST,
                &format!("api/v1/objects/{}", object_path),
                Some(request.to_vec()),
                None,
                "application/octet-stream",
                options,
            )
            .await
    }

    /// Deletes an object from your environment given the objectPath path parameter.
    ///
    /// # Arguments
    ///
    /// * `object_path` - The path of the object to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .objects
    ///         .delete_object(&"objectPath".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_object(
        &self,
        object_path: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v1/objects/{}", object_path),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns metadata for a specified object path. Use this to fetch metadata such as object size (size_bytes), its expiry time (expiry_time), or when it arrived on the node holding it (last_updated_at).
    ///
    /// # Arguments
    ///
    /// * `object_path` - The path of the object to query.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .objects
    ///         .get_object_metadata(&"objectPath".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_object_metadata(
        &self,
        object_path: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::HEAD,
                &format!("api/v1/objects/{}", object_path),
                None,
                None,
                options,
            )
            .await
    }
}
