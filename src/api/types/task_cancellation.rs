pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskCancellation {
    /// Who or what is requesting to cancel this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
}

impl TaskCancellation {
    pub fn builder() -> TaskCancellationBuilder {
        <TaskCancellationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskCancellationBuilder {
    author: Option<Principal>,
}

impl TaskCancellationBuilder {
    pub fn author(mut self, value: Principal) -> Self {
        self.author = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskCancellation`].
    pub fn build(self) -> Result<TaskCancellation, BuildError> {
        Ok(TaskCancellation {
            author: self.author,
        })
    }
}
