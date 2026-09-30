pub use crate::prelude::*;

/// Defines a supported task by the task specification URL of its "Any" type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskDefinition {
    /// Url path must be prefixed with `type.googleapis.com/`.
    #[serde(rename = "taskSpecificationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_specification_url: Option<String>,
}

impl TaskDefinition {
    pub fn builder() -> TaskDefinitionBuilder {
        <TaskDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskDefinitionBuilder {
    task_specification_url: Option<String>,
}

impl TaskDefinitionBuilder {
    pub fn task_specification_url(mut self, value: impl Into<String>) -> Self {
        self.task_specification_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskDefinition`].
    pub fn build(self) -> Result<TaskDefinition, BuildError> {
        Ok(TaskDefinition {
            task_specification_url: self.task_specification_url,
        })
    }
}
