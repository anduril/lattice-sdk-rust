pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
#[non_exhaustive]
pub enum StreamEntitiesResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {},

    #[serde(rename = "entity")]
    #[non_exhaustive]
    Entity {},

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl StreamEntitiesResponse {
    pub fn heartbeat() -> Self {
        Self::Heartbeat {}
    }

    pub fn entity() -> Self {
        Self::Entity {}
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
