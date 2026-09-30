pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityStreamRequest {
    /// at what interval to send heartbeat events, defaults to 30s.
    #[serde(rename = "heartbeatIntervalMS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval_ms: Option<i64>,
    /// only stream pre-existing entities in the environment and then close the connection, defaults to false.
    #[serde(rename = "preExistingOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_existing_only: Option<bool>,
    /// list of components to include, leave empty to include all components.
    #[serde(rename = "componentsToInclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components_to_include: Option<Vec<String>>,
    /// Optional root of a Statement filter expression tree. If provided, only entities matching
    /// the filter are streamed. Applied dynamically: an entity that begins matching is delivered
    /// as a CREATE, and one that stops matching is delivered as a DELETE. Mirrors the filter on
    /// the gRPC StreamEntityComponents endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Statement>,
}

impl EntityStreamRequest {
    pub fn builder() -> EntityStreamRequestBuilder {
        <EntityStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityStreamRequestBuilder {
    heartbeat_interval_ms: Option<i64>,
    pre_existing_only: Option<bool>,
    components_to_include: Option<Vec<String>>,
    filter: Option<Statement>,
}

impl EntityStreamRequestBuilder {
    pub fn heartbeat_interval_ms(mut self, value: i64) -> Self {
        self.heartbeat_interval_ms = Some(value);
        self
    }

    pub fn pre_existing_only(mut self, value: bool) -> Self {
        self.pre_existing_only = Some(value);
        self
    }

    pub fn components_to_include(mut self, value: Vec<String>) -> Self {
        self.components_to_include = Some(value);
        self
    }

    pub fn filter(mut self, value: Statement) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityStreamRequest`].
    pub fn build(self) -> Result<EntityStreamRequest, BuildError> {
        Ok(EntityStreamRequest {
            heartbeat_interval_ms: self.heartbeat_interval_ms,
            pre_existing_only: self.pre_existing_only,
            components_to_include: self.components_to_include,
            filter: self.filter,
        })
    }
}
