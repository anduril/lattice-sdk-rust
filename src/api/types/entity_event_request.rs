pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityEventRequest {
    /// Long-poll session identifier. Leave empty to start a new polling session.
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    pub session_token: String,
    /// Maximum size of response batch. Defaults to 100. Must be between 1 and 2000 (inclusive).
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
}

impl EntityEventRequest {
    pub fn builder() -> EntityEventRequestBuilder {
        <EntityEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityEventRequestBuilder {
    session_token: Option<String>,
    batch_size: Option<i64>,
}

impl EntityEventRequestBuilder {
    pub fn session_token(mut self, value: impl Into<String>) -> Self {
        self.session_token = Some(value.into());
        self
    }

    pub fn batch_size(mut self, value: i64) -> Self {
        self.batch_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityEventRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_token`](EntityEventRequestBuilder::session_token)
    pub fn build(self) -> Result<EntityEventRequest, BuildError> {
        Ok(EntityEventRequest {
            session_token: self
                .session_token
                .ok_or_else(|| BuildError::missing_field("session_token"))?,
            batch_size: self.batch_size,
        })
    }
}
