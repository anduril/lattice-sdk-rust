pub use crate::prelude::*;

/// Response streamed to an agent containing task actions to perform.
///
/// This message is streamed from Tasks API to agents and contains one of three
/// possible requests: execute a task, cancel a task, or complete a task. The agent
/// should process these requests according to its capabilities and report status
/// updates back to Tasks API using the UpdateStatus endpoint.
///
/// Multiple responses may be sent for different tasks, and the agent should maintain
/// the connection to receive ongoing task requests. The connection may also be used
/// for heartbeat messages to ensure the agent is still responsive.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentRequest {
    #[serde(rename = "executeRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_request: Option<ExecuteRequest>,
    #[serde(rename = "cancelRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_request: Option<CancelRequest>,
    #[serde(rename = "completeRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_request: Option<CompleteRequest>,
}

impl AgentRequest {
    pub fn builder() -> AgentRequestBuilder {
        <AgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentRequestBuilder {
    execute_request: Option<ExecuteRequest>,
    cancel_request: Option<CancelRequest>,
    complete_request: Option<CompleteRequest>,
}

impl AgentRequestBuilder {
    pub fn execute_request(mut self, value: ExecuteRequest) -> Self {
        self.execute_request = Some(value);
        self
    }

    pub fn cancel_request(mut self, value: CancelRequest) -> Self {
        self.cancel_request = Some(value);
        self
    }

    pub fn complete_request(mut self, value: CompleteRequest) -> Self {
        self.complete_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentRequest`].
    pub fn build(self) -> Result<AgentRequest, BuildError> {
        Ok(AgentRequest {
            execute_request: self.execute_request,
            cancel_request: self.cancel_request,
            complete_request: self.complete_request,
        })
    }
}
