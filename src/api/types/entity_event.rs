pub use crate::prelude::*;

/// Event representing some type of entity change.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityEvent {
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EntityEventEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub time: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl EntityEvent {
    pub fn builder() -> EntityEventBuilder {
        <EntityEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityEventBuilder {
    event_type: Option<EntityEventEventType>,
    time: Option<DateTime<FixedOffset>>,
    entity: Option<Entity>,
}

impl EntityEventBuilder {
    pub fn event_type(mut self, value: EntityEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.time = Some(value);
        self
    }

    pub fn entity(mut self, value: Entity) -> Self {
        self.entity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityEvent`].
    pub fn build(self) -> Result<EntityEvent, BuildError> {
        Ok(EntityEvent {
            event_type: self.event_type,
            time: self.time,
            entity: self.entity,
        })
    }
}
