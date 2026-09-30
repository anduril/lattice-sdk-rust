pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskStatusUpdate {
    /// The status version of the task to update. This version number increments to indicate the task's
    /// current stage in its status lifecycle. Specifically, whenever a task's status updates, the status
    /// version increments by one. Any status updates received with a lower status version number than what
    /// is known are considered stale and ignored.
    #[serde(rename = "statusVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_version: Option<i64>,
    /// The new status of the task.
    #[serde(rename = "newStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<TaskStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
}

impl TaskStatusUpdate {
    pub fn builder() -> TaskStatusUpdateBuilder {
        <TaskStatusUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStatusUpdateBuilder {
    status_version: Option<i64>,
    new_status: Option<TaskStatus>,
    author: Option<Principal>,
}

impl TaskStatusUpdateBuilder {
    pub fn status_version(mut self, value: i64) -> Self {
        self.status_version = Some(value);
        self
    }

    pub fn new_status(mut self, value: TaskStatus) -> Self {
        self.new_status = Some(value);
        self
    }

    pub fn author(mut self, value: Principal) -> Self {
        self.author = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStatusUpdate`].
    pub fn build(self) -> Result<TaskStatusUpdate, BuildError> {
        Ok(TaskStatusUpdate {
            status_version: self.status_version,
            new_status: self.new_status,
            author: self.author,
        })
    }
}
