pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTokenRequest {
    /// The type of grant being requested
    pub grant_type: String,
    /// The client identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The client secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl GetTokenRequest {
    pub fn builder() -> GetTokenRequestBuilder {
        <GetTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTokenRequestBuilder {
    grant_type: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl GetTokenRequestBuilder {
    pub fn grant_type(mut self, value: impl Into<String>) -> Self {
        self.grant_type = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`grant_type`](GetTokenRequestBuilder::grant_type)
    pub fn build(self) -> Result<GetTokenRequest, BuildError> {
        Ok(GetTokenRequest {
            grant_type: self
                .grant_type
                .ok_or_else(|| BuildError::missing_field("grant_type"))?,
            client_id: self.client_id,
            client_secret: self.client_secret,
        })
    }
}
