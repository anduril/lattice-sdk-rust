pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentListener {
    /// Selector criteria to determine which Agent Tasks the agent receives
    #[serde(rename = "agentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_selector: Option<EntityIdsSelector>,
}

impl AgentListener {
    pub fn builder() -> AgentListenerBuilder {
        <AgentListenerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentListenerBuilder {
    agent_selector: Option<EntityIdsSelector>,
}

impl AgentListenerBuilder {
    pub fn agent_selector(mut self, value: EntityIdsSelector) -> Self {
        self.agent_selector = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentListener`].
    pub fn build(self) -> Result<AgentListener, BuildError> {
        Ok(AgentListener {
            agent_selector: self.agent_selector,
        })
    }
}
