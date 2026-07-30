pub use crate::prelude::*;

/// The task event that occurred.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskEventDataTaskEvent {
    /// The type of event that occurred for this task.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<TaskEventDataTaskEventEventType>,
    /// The task associated with this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

impl TaskEventDataTaskEvent {
    pub fn builder() -> TaskEventDataTaskEventBuilder {
        <TaskEventDataTaskEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskEventDataTaskEventBuilder {
    event_type: Option<TaskEventDataTaskEventEventType>,
    task: Option<Task>,
}

impl TaskEventDataTaskEventBuilder {
    pub fn event_type(mut self, value: TaskEventDataTaskEventEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn task(mut self, value: Task) -> Self {
        self.task = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskEventDataTaskEvent`].
    pub fn build(self) -> Result<TaskEventDataTaskEvent, BuildError> {
        Ok(TaskEventDataTaskEvent {
            event_type: self.event_type,
            task: self.task,
        })
    }
}
