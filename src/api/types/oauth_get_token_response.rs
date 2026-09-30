pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTokenResponse {
    /// The access token
    #[serde(default)]
    pub access_token: String,
    /// The type of token (typically "Bearer")
    #[serde(default)]
    pub token_type: String,
    /// Lifetime of the access token in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// Lifetime of the refresh token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_expires_in: Option<i64>,
    /// Enforce that a token cannot be used before a specific unixtime
    #[serde(rename = "not-before-policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before_policy: Option<i64>,
    /// The scope of the access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl GetTokenResponse {
    pub fn builder() -> GetTokenResponseBuilder {
        <GetTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTokenResponseBuilder {
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<i64>,
    refresh_expires_in: Option<i64>,
    not_before_policy: Option<i64>,
    scope: Option<String>,
}

impl GetTokenResponseBuilder {
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    pub fn token_type(mut self, value: impl Into<String>) -> Self {
        self.token_type = Some(value.into());
        self
    }

    pub fn expires_in(mut self, value: i64) -> Self {
        self.expires_in = Some(value);
        self
    }

    pub fn refresh_expires_in(mut self, value: i64) -> Self {
        self.refresh_expires_in = Some(value);
        self
    }

    pub fn not_before_policy(mut self, value: i64) -> Self {
        self.not_before_policy = Some(value);
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_token`](GetTokenResponseBuilder::access_token)
    /// - [`token_type`](GetTokenResponseBuilder::token_type)
    pub fn build(self) -> Result<GetTokenResponse, BuildError> {
        Ok(GetTokenResponse {
            access_token: self
                .access_token
                .ok_or_else(|| BuildError::missing_field("access_token"))?,
            token_type: self
                .token_type
                .ok_or_else(|| BuildError::missing_field("token_type"))?,
            expires_in: self.expires_in,
            refresh_expires_in: self.refresh_expires_in,
            not_before_policy: self.not_before_policy,
            scope: self.scope,
        })
    }
}
