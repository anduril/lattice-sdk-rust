pub use crate::prelude::*;

/// Catalog of supported tasks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskCatalog {
    #[serde(rename = "taskDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
}

impl TaskCatalog {
    pub fn builder() -> TaskCatalogBuilder {
        <TaskCatalogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskCatalogBuilder {
    task_definitions: Option<Vec<TaskDefinition>>,
}

impl TaskCatalogBuilder {
    pub fn task_definitions(mut self, value: Vec<TaskDefinition>) -> Self {
        self.task_definitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskCatalog`].
    pub fn build(self) -> Result<TaskCatalog, BuildError> {
        Ok(TaskCatalog {
            task_definitions: self.task_definitions,
        })
    }
}
