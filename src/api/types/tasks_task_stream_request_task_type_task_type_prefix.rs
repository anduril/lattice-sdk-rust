pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TaskStreamRequestTaskTypeTaskTypePrefix {
    /// Prefix string to match task types. Any task with a type that starts with this prefix will be included.
    pub task_type_prefix: String,
}

impl TaskStreamRequestTaskTypeTaskTypePrefix {
    pub fn builder() -> TaskStreamRequestTaskTypeTaskTypePrefixBuilder {
        <TaskStreamRequestTaskTypeTaskTypePrefixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStreamRequestTaskTypeTaskTypePrefixBuilder {
    task_type_prefix: Option<String>,
}

impl TaskStreamRequestTaskTypeTaskTypePrefixBuilder {
    pub fn task_type_prefix(mut self, value: impl Into<String>) -> Self {
        self.task_type_prefix = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskStreamRequestTaskTypeTaskTypePrefix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_type_prefix`](TaskStreamRequestTaskTypeTaskTypePrefixBuilder::task_type_prefix)
    pub fn build(self) -> Result<TaskStreamRequestTaskTypeTaskTypePrefix, BuildError> {
        Ok(TaskStreamRequestTaskTypeTaskTypePrefix {
            task_type_prefix: self
                .task_type_prefix
                .ok_or_else(|| BuildError::missing_field("task_type_prefix"))?,
        })
    }
}
