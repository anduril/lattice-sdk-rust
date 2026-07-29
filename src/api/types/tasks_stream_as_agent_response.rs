pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
#[non_exhaustive]
pub enum StreamAsAgentResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {
        #[serde(flatten)]
        data: StreamHeartbeat,
    },

    #[serde(rename = "agent_request")]
    #[non_exhaustive]
    AgentRequest {},

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl StreamAsAgentResponse {
    pub fn heartbeat(data: StreamHeartbeat) -> Self {
        Self::Heartbeat { data }
    }

    pub fn agent_request() -> Self {
        Self::AgentRequest {}
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
