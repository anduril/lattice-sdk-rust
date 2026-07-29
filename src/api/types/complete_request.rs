pub use crate::prelude::*;

/// The request to complete a task.
/// Contains the unique ID of the task to complete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteRequest {
    /// ID of the task to complete.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl CompleteRequest {
    pub fn builder() -> CompleteRequestBuilder {
        <CompleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteRequestBuilder {
    task_id: Option<String>,
}

impl CompleteRequestBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteRequest`].
    pub fn build(self) -> Result<CompleteRequest, BuildError> {
        Ok(CompleteRequest {
            task_id: self.task_id,
        })
    }
}
