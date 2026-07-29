pub use crate::prelude::*;

/// Describes the relationships associated with this task: the system assigned to
/// execute the task, and the parent task, if one exists.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Relations {
    /// The system, user, or team assigned to the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Principal>,
    /// Identifies the parent task if the task is a sub-task.
    #[serde(rename = "parentTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
}

impl Relations {
    pub fn builder() -> RelationsBuilder {
        <RelationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RelationsBuilder {
    assignee: Option<Principal>,
    parent_task_id: Option<String>,
}

impl RelationsBuilder {
    pub fn assignee(mut self, value: Principal) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Relations`].
    pub fn build(self) -> Result<Relations, BuildError> {
        Ok(Relations {
            assignee: self.assignee,
            parent_task_id: self.parent_task_id,
        })
    }
}
