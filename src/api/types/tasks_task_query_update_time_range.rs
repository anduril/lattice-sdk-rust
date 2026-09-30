pub use crate::prelude::*;

/// If provided, only provides tasks updated within the time range.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskQueryUpdateTimeRange {
    /// If provided, returns tasks only updated after this time.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// If provided, returns tasks only updated before this time.
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl TaskQueryUpdateTimeRange {
    pub fn builder() -> TaskQueryUpdateTimeRangeBuilder {
        <TaskQueryUpdateTimeRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskQueryUpdateTimeRangeBuilder {
    start_time: Option<String>,
    end_time: Option<String>,
}

impl TaskQueryUpdateTimeRangeBuilder {
    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    pub fn end_time(mut self, value: impl Into<String>) -> Self {
        self.end_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskQueryUpdateTimeRange`].
    pub fn build(self) -> Result<TaskQueryUpdateTimeRange, BuildError> {
        Ok(TaskQueryUpdateTimeRange {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}
