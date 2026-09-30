pub use crate::prelude::*;

/// System Principal representing some autonomous system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct System {
    /// Name of the service associated with this System.
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// The Entity ID of the System.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Whether the System Principal (for example, an Asset) can own scheduling.
    /// This means we bypass manager-owned scheduling and defer to the system
    /// Principal to handle scheduling and give us status updates for the task.
    /// Regardless of the value defined by the client, the Task Manager will
    /// determine and set this value appropriately.
    #[serde(rename = "managesOwnScheduling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manages_own_scheduling: Option<bool>,
}

impl System {
    pub fn builder() -> SystemBuilder {
        <SystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemBuilder {
    service_name: Option<String>,
    entity_id: Option<String>,
    manages_own_scheduling: Option<bool>,
}

impl SystemBuilder {
    pub fn service_name(mut self, value: impl Into<String>) -> Self {
        self.service_name = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn manages_own_scheduling(mut self, value: bool) -> Self {
        self.manages_own_scheduling = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`System`].
    pub fn build(self) -> Result<System, BuildError> {
        Ok(System {
            service_name: self.service_name,
            entity_id: self.entity_id,
            manages_own_scheduling: self.manages_own_scheduling,
        })
    }
}
