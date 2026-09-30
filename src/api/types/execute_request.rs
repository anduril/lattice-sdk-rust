pub use crate::prelude::*;

/// The request to execute a task.
/// Contains the unique ID of the task to execute.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecuteRequest {
    /// The task to execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

impl ExecuteRequest {
    pub fn builder() -> ExecuteRequestBuilder {
        <ExecuteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecuteRequestBuilder {
    task: Option<Task>,
}

impl ExecuteRequestBuilder {
    pub fn task(mut self, value: Task) -> Self {
        self.task = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecuteRequest`].
    pub fn build(self) -> Result<ExecuteRequest, BuildError> {
        Ok(ExecuteRequest { task: self.task })
    }
}
