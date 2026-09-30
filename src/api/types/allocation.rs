pub use crate::prelude::*;

/// Allocation contains a list of agents allocated to a task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Allocation {
    /// Agents actively being utilized in a task.
    #[serde(rename = "activeAgents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agents: Option<Vec<Agent>>,
}

impl Allocation {
    pub fn builder() -> AllocationBuilder {
        <AllocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AllocationBuilder {
    active_agents: Option<Vec<Agent>>,
}

impl AllocationBuilder {
    pub fn active_agents(mut self, value: Vec<Agent>) -> Self {
        self.active_agents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Allocation`].
    pub fn build(self) -> Result<Allocation, BuildError> {
        Ok(Allocation {
            active_agents: self.active_agents,
        })
    }
}
