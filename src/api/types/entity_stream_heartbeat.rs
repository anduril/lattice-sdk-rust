pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntityStreamHeartbeat {
    #[serde(flatten)]
    pub heartbeat_object_fields: HeartbeatObject,
}

impl EntityStreamHeartbeat {
    pub fn builder() -> EntityStreamHeartbeatBuilder {
        <EntityStreamHeartbeatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityStreamHeartbeatBuilder {
    heartbeat_object_fields: Option<HeartbeatObject>,
}

impl EntityStreamHeartbeatBuilder {
    pub fn heartbeat_object_fields(mut self, value: HeartbeatObject) -> Self {
        self.heartbeat_object_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityStreamHeartbeat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`heartbeat_object_fields`](EntityStreamHeartbeatBuilder::heartbeat_object_fields)
    pub fn build(self) -> Result<EntityStreamHeartbeat, BuildError> {
        Ok(EntityStreamHeartbeat {
            heartbeat_object_fields: self
                .heartbeat_object_fields
                .ok_or_else(|| BuildError::missing_field("heartbeat_object_fields"))?,
        })
    }
}
