pub use crate::prelude::*;

/// Represents an agent capable of processing tasks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Agent {
    /// Entity ID of the agent.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl Agent {
    pub fn builder() -> AgentBuilder {
        <AgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBuilder {
    entity_id: Option<String>,
}

impl AgentBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Agent`].
    pub fn build(self) -> Result<Agent, BuildError> {
        Ok(Agent {
            entity_id: self.entity_id,
        })
    }
}
