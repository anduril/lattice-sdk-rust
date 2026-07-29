pub use crate::prelude::*;

/// Health of an individual component.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComponentHealth {
    /// Consistent internal ID for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Display name for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Health for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<ComponentHealthHealth>,
    /// Human-readable describing the component state. These messages should be understandable by end users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ComponentMessage>>,
    /// The last update time for this specific component.
    /// If this timestamp is unset, the data is assumed to be most recent
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub update_time: Option<DateTime<FixedOffset>>,
}

impl ComponentHealth {
    pub fn builder() -> ComponentHealthBuilder {
        <ComponentHealthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComponentHealthBuilder {
    id: Option<String>,
    name: Option<String>,
    health: Option<ComponentHealthHealth>,
    messages: Option<Vec<ComponentMessage>>,
    update_time: Option<DateTime<FixedOffset>>,
}

impl ComponentHealthBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn health(mut self, value: ComponentHealthHealth) -> Self {
        self.health = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ComponentMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn update_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.update_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComponentHealth`].
    pub fn build(self) -> Result<ComponentHealth, BuildError> {
        Ok(ComponentHealth {
            id: self.id,
            name: self.name,
            health: self.health,
            messages: self.messages,
            update_time: self.update_time,
        })
    }
}
