pub use crate::prelude::*;

/// An entity wrapper used in task definitions, with additional metadata.
///
/// TaskEntity wraps an entity reference with additional contextual information for task execution.
/// This structure allows entities to be passed to tasks with supplementary metadata that aids
/// in proper task execution, while also serving as an extension point for future capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskEntity {
    /// The wrapped entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    /// Indicates that this entity was generated from a snapshot of a live entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl TaskEntity {
    pub fn builder() -> TaskEntityBuilder {
        <TaskEntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskEntityBuilder {
    entity: Option<Entity>,
    snapshot: Option<bool>,
}

impl TaskEntityBuilder {
    pub fn entity(mut self, value: Entity) -> Self {
        self.entity = Some(value);
        self
    }

    pub fn snapshot(mut self, value: bool) -> Self {
        self.snapshot = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskEntity`].
    pub fn build(self) -> Result<TaskEntity, BuildError> {
        Ok(TaskEntity {
            entity: self.entity,
            snapshot: self.snapshot,
        })
    }
}
