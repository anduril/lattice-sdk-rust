pub use crate::prelude::*;

/// `ExecutionConstraints` provides scheduling details that informs the agent when to execute the task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionConstraints {
    /// The timestamp after which the agent can execute the task
    #[serde(rename = "startAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_after: Option<DateTime<FixedOffset>>,
    /// The timestamp before which the agent can execute the task.
    #[serde(rename = "completeBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub complete_before: Option<DateTime<FixedOffset>>,
}

impl ExecutionConstraints {
    pub fn builder() -> ExecutionConstraintsBuilder {
        <ExecutionConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionConstraintsBuilder {
    start_after: Option<DateTime<FixedOffset>>,
    complete_before: Option<DateTime<FixedOffset>>,
}

impl ExecutionConstraintsBuilder {
    pub fn start_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_after = Some(value);
        self
    }

    pub fn complete_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.complete_before = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionConstraints`].
    pub fn build(self) -> Result<ExecutionConstraints, BuildError> {
        Ok(ExecutionConstraints {
            start_after: self.start_after,
            complete_before: self.complete_before,
        })
    }
}
