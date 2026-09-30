pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event")]
#[non_exhaustive]
pub enum StreamTasksResponse {
    #[serde(rename = "heartbeat")]
    #[non_exhaustive]
    Heartbeat {
        #[serde(flatten)]
        data: StreamHeartbeat,
    },

    #[serde(rename = "task_event")]
    #[non_exhaustive]
    TaskEvent {
        #[serde(flatten)]
        data: TaskStreamEvent,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl StreamTasksResponse {
    pub fn heartbeat(data: StreamHeartbeat) -> Self {
        Self::Heartbeat { data }
    }

    pub fn task_event(data: TaskStreamEvent) -> Self {
        Self::TaskEvent { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
