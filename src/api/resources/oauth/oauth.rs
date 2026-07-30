use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OauthClient {
    pub http_client: HttpClient,
}

impl OauthClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets a new short-lived token using the specified client credentials
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
    /// use anduril::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .oauth
    ///         .get_token(
    ///             &GetTokenRequest {
    ///                 grant_type: "client_credentials".to_string(),
    ///                 client_id: None,
    ///                 client_secret: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_token(
        &self,
        request: &GetTokenRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTokenResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/oauth/token",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
