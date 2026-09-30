pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HeartbeatObject {
    /// The timestamp at which the heartbeat message was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HeartbeatObject {
    pub fn builder() -> HeartbeatObjectBuilder {
        <HeartbeatObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HeartbeatObjectBuilder {
    timestamp: Option<String>,
}

impl HeartbeatObjectBuilder {
    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HeartbeatObject`].
    pub fn build(self) -> Result<HeartbeatObject, BuildError> {
        Ok(HeartbeatObject {
            timestamp: self.timestamp,
        })
    }
}
