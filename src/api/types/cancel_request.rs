pub use crate::prelude::*;

/// The request to cancel a task.
/// Contains the task, and the assignee of the request to cancel the task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CancelRequest {
    /// The unique task ID of the task to cancel.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// The assignee of the Task. Useful for agent routing where an endpoint owns multiple agents,
    /// especially onBehalfOf assignees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Principal>,
    /// The principal that requested to cancel the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
}

impl CancelRequest {
    pub fn builder() -> CancelRequestBuilder {
        <CancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelRequestBuilder {
    task_id: Option<String>,
    assignee: Option<Principal>,
    author: Option<Principal>,
}

impl CancelRequestBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn assignee(mut self, value: Principal) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn author(mut self, value: Principal) -> Self {
        self.author = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelRequest`].
    pub fn build(self) -> Result<CancelRequest, BuildError> {
        Ok(CancelRequest {
            task_id: self.task_id,
            assignee: self.assignee,
            author: self.author,
        })
    }
}
