pub use crate::prelude::*;

/// Contains information about a task event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskEventData {
    /// The task event that occurred.
    #[serde(rename = "taskEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_event: Option<TaskEventDataTaskEvent>,
}

impl TaskEventData {
    pub fn builder() -> TaskEventDataBuilder {
        <TaskEventDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskEventDataBuilder {
    task_event: Option<TaskEventDataTaskEvent>,
}

impl TaskEventDataBuilder {
    pub fn task_event(mut self, value: TaskEventDataTaskEvent) -> Self {
        self.task_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskEventData`].
    pub fn build(self) -> Result<TaskEventData, BuildError> {
        Ok(TaskEventData {
            task_event: self.task_event,
        })
    }
}
