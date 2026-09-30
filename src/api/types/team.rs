pub use crate::prelude::*;

/// Represents a team of agents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Team {
    /// Entity ID of the team
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Agent>>,
}

impl Team {
    pub fn builder() -> TeamBuilder {
        <TeamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamBuilder {
    entity_id: Option<String>,
    members: Option<Vec<Agent>>,
}

impl TeamBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<Agent>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Team`].
    pub fn build(self) -> Result<Team, BuildError> {
        Ok(Team {
            entity_id: self.entity_id,
            members: self.members,
        })
    }
}
