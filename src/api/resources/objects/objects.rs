use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `since_timestamp` - Sets the age for the oldest objects to query across the environment.
    /// * `page_token` - Base64 and URL-encoded cursor returned by the service to continue paging.
    /// * `all_objects_in_mesh` - Lists objects across all environment nodes in a Lattice Mesh.
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
    /// use anduril::prelude::*;
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
    /// use anduril::prelude::*;
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
    /// use anduril::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client.objects.upload_object(&vec![], None).await;
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
    /// use anduril::prelude::*;
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

    /// Returns metadata for a specified object path. Use this to fetch metadata such as object size (size_bytes), its expiry time (expiry_time), or its latest update timestamp (last_updated_at).
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
    /// use anduril::prelude::*;
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
