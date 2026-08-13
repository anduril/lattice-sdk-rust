pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskQueryStatusFilter {
    /// Status of the task to filter by, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskQueryStatusFilterStatus>,
}

impl TaskQueryStatusFilter {
    pub fn builder() -> TaskQueryStatusFilterBuilder {
        <TaskQueryStatusFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskQueryStatusFilterBuilder {
    status: Option<TaskQueryStatusFilterStatus>,
}

impl TaskQueryStatusFilterBuilder {
    pub fn status(mut self, value: TaskQueryStatusFilterStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskQueryStatusFilter`].
    pub fn build(self) -> Result<TaskQueryStatusFilter, BuildError> {
        Ok(TaskQueryStatusFilter {
            status: self.status,
        })
    }
}
