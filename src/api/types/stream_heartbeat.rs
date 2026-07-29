pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamHeartbeat {
    #[serde(flatten)]
    pub heartbeat_object_fields: HeartbeatObject,
}

impl StreamHeartbeat {
    pub fn builder() -> StreamHeartbeatBuilder {
        <StreamHeartbeatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamHeartbeatBuilder {
    heartbeat_object_fields: Option<HeartbeatObject>,
}

impl StreamHeartbeatBuilder {
    pub fn heartbeat_object_fields(mut self, value: HeartbeatObject) -> Self {
        self.heartbeat_object_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamHeartbeat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`heartbeat_object_fields`](StreamHeartbeatBuilder::heartbeat_object_fields)
    pub fn build(self) -> Result<StreamHeartbeat, BuildError> {
        Ok(StreamHeartbeat {
            heartbeat_object_fields: self
                .heartbeat_object_fields
                .ok_or_else(|| BuildError::missing_field("heartbeat_object_fields"))?,
        })
    }
}
