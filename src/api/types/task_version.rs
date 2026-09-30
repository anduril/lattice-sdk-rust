pub use crate::prelude::*;

/// Versioning information for a task.
///
/// TaskVersion provides a unique identifier for each task, along with separate version counters
/// for tracking changes to the task's definition and its status. This versioning system enables
/// optimistic concurrency control, ensuring that updates from multiple sources don't conflict.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskVersion {
    /// The unique identifier for this task, used to distinguish it from all other tasks in the system.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Counter that increments on changes to the task definition.
    /// Unset (0) initially, starts at 1 on creation, and increments with each update to task fields.
    #[serde(rename = "definitionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_version: Option<i64>,
    /// Counter that increments on changes to TaskStatus.
    /// Unset (0) initially, starts at 1 on creation, and increments with each status update.
    #[serde(rename = "statusVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_version: Option<i64>,
}

impl TaskVersion {
    pub fn builder() -> TaskVersionBuilder {
        <TaskVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskVersionBuilder {
    task_id: Option<String>,
    definition_version: Option<i64>,
    status_version: Option<i64>,
}

impl TaskVersionBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn definition_version(mut self, value: i64) -> Self {
        self.definition_version = Some(value);
        self
    }

    pub fn status_version(mut self, value: i64) -> Self {
        self.status_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskVersion`].
    pub fn build(self) -> Result<TaskVersion, BuildError> {
        Ok(TaskVersion {
            task_id: self.task_id,
            definition_version: self.definition_version,
            status_version: self.status_version,
        })
    }
}
