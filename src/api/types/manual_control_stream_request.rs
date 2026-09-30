pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManualControlStreamRequest {
    /// The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    #[serde(rename = "heartbeatIntervalMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval_ms: Option<i64>,
}

impl ManualControlStreamRequest {
    pub fn builder() -> ManualControlStreamRequestBuilder {
        <ManualControlStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualControlStreamRequestBuilder {
    heartbeat_interval_ms: Option<i64>,
}

impl ManualControlStreamRequestBuilder {
    pub fn heartbeat_interval_ms(mut self, value: i64) -> Self {
        self.heartbeat_interval_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualControlStreamRequest`].
    pub fn build(self) -> Result<ManualControlStreamRequest, BuildError> {
        Ok(ManualControlStreamRequest {
            heartbeat_interval_ms: self.heartbeat_interval_ms,
        })
    }
}
