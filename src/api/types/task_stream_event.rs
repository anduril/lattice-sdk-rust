pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskStreamEvent {
    #[serde(flatten)]
    pub task_event_data_fields: TaskEventData,
}

impl TaskStreamEvent {
    pub fn builder() -> TaskStreamEventBuilder {
        <TaskStreamEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStreamEventBuilder {
    task_event_data_fields: Option<TaskEventData>,
}

impl TaskStreamEventBuilder {
    pub fn task_event_data_fields(mut self, value: TaskEventData) -> Self {
        self.task_event_data_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStreamEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_event_data_fields`](TaskStreamEventBuilder::task_event_data_fields)
    pub fn build(self) -> Result<TaskStreamEvent, BuildError> {
        Ok(TaskStreamEvent {
            task_event_data_fields: self
                .task_event_data_fields
                .ok_or_else(|| BuildError::missing_field("task_event_data_fields"))?,
        })
    }
}
