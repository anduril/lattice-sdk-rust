pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentStreamEvent {
    #[serde(flatten)]
    pub agent_task_request_fields: AgentTaskRequest,
}

impl AgentStreamEvent {
    pub fn builder() -> AgentStreamEventBuilder {
        <AgentStreamEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentStreamEventBuilder {
    agent_task_request_fields: Option<AgentTaskRequest>,
}

impl AgentStreamEventBuilder {
    pub fn agent_task_request_fields(mut self, value: AgentTaskRequest) -> Self {
        self.agent_task_request_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentStreamEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_task_request_fields`](AgentStreamEventBuilder::agent_task_request_fields)
    pub fn build(self) -> Result<AgentStreamEvent, BuildError> {
        Ok(AgentStreamEvent {
            agent_task_request_fields: self
                .agent_task_request_fields
                .ok_or_else(|| BuildError::missing_field("agent_task_request_fields"))?,
        })
    }
}
