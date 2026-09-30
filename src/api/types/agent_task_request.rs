pub use crate::prelude::*;

/// The wrapper for a task's action requests: execute, cancel, or complete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentTaskRequest {
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

impl AgentTaskRequest {
    pub fn builder() -> AgentTaskRequestBuilder {
        <AgentTaskRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentTaskRequestBuilder {
    execute_request: Option<ExecuteRequest>,
    cancel_request: Option<CancelRequest>,
    complete_request: Option<CompleteRequest>,
}

impl AgentTaskRequestBuilder {
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

    /// Consumes the builder and constructs a [`AgentTaskRequest`].
    pub fn build(self) -> Result<AgentTaskRequest, BuildError> {
        Ok(AgentTaskRequest {
            execute_request: self.execute_request,
            cancel_request: self.cancel_request,
            complete_request: self.complete_request,
        })
    }
}
