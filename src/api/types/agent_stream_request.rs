pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentStreamRequest {
    /// The selector criteria to determine which tasks the agent receives.
    #[serde(rename = "agentSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_selector: Option<EntityIdsSelector>,
    /// The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    #[serde(rename = "heartbeatIntervalMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval_ms: Option<i64>,
}

impl AgentStreamRequest {
    pub fn builder() -> AgentStreamRequestBuilder {
        <AgentStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentStreamRequestBuilder {
    agent_selector: Option<EntityIdsSelector>,
    heartbeat_interval_ms: Option<i64>,
}

impl AgentStreamRequestBuilder {
    pub fn agent_selector(mut self, value: EntityIdsSelector) -> Self {
        self.agent_selector = Some(value);
        self
    }

    pub fn heartbeat_interval_ms(mut self, value: i64) -> Self {
        self.heartbeat_interval_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentStreamRequest`].
    pub fn build(self) -> Result<AgentStreamRequest, BuildError> {
        Ok(AgentStreamRequest {
            agent_selector: self.agent_selector,
            heartbeat_interval_ms: self.heartbeat_interval_ms,
        })
    }
}
