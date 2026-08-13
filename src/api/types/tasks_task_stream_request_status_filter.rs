pub use crate::prelude::*;

/// A filter for task statuses (inclusive or exclusive).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskStreamRequestStatusFilter {
    /// The statuses to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<TaskStreamRequestStatusFilterStatusesItem>>,
    /// The type of filter to apply.
    #[serde(rename = "filterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<TaskStreamRequestStatusFilterFilterType>,
}

impl TaskStreamRequestStatusFilter {
    pub fn builder() -> TaskStreamRequestStatusFilterBuilder {
        <TaskStreamRequestStatusFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStreamRequestStatusFilterBuilder {
    statuses: Option<Vec<TaskStreamRequestStatusFilterStatusesItem>>,
    filter_type: Option<TaskStreamRequestStatusFilterFilterType>,
}

impl TaskStreamRequestStatusFilterBuilder {
    pub fn statuses(mut self, value: Vec<TaskStreamRequestStatusFilterStatusesItem>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn filter_type(mut self, value: TaskStreamRequestStatusFilterFilterType) -> Self {
        self.filter_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStreamRequestStatusFilter`].
    pub fn build(self) -> Result<TaskStreamRequestStatusFilter, BuildError> {
        Ok(TaskStreamRequestStatusFilter {
            statuses: self.statuses,
            filter_type: self.filter_type,
        })
    }
}
