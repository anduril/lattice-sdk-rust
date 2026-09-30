pub use crate::prelude::*;

/// Contains status of entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Status {
    /// A string that describes the activity that the entity is performing.
    /// Examples include "RECONNAISSANCE", "INTERDICTION", "RETURN TO BASE (RTB)", "PREPARING FOR LAUNCH".
    #[serde(rename = "platformActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_activity: Option<String>,
    /// A human-readable string that describes the role the entity is currently performing. E.g. "Team Member", "Commander".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl Status {
    pub fn builder() -> StatusBuilder {
        <StatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusBuilder {
    platform_activity: Option<String>,
    role: Option<String>,
}

impl StatusBuilder {
    pub fn platform_activity(mut self, value: impl Into<String>) -> Self {
        self.platform_activity = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Status`].
    pub fn build(self) -> Result<Status, BuildError> {
        Ok(Status {
            platform_activity: self.platform_activity,
            role: self.role,
        })
    }
}
