pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskQuery {
    /// If set, returns results starting from the given pageToken.
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// If present matches tasks with this parent task ID.
    /// This is mutually exclusive with all other query parameters, for example, either provide parent task ID, or
    /// any of the remaining parameters, but not both.
    #[serde(rename = "parentTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    #[serde(rename = "statusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<TaskQueryStatusFilter>,
    /// If provided, only provides tasks updated within the time range.
    #[serde(rename = "updateTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_range: Option<TaskQueryUpdateTimeRange>,
}

impl TaskQuery {
    pub fn builder() -> TaskQueryBuilder {
        <TaskQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskQueryBuilder {
    page_token: Option<String>,
    parent_task_id: Option<String>,
    status_filter: Option<TaskQueryStatusFilter>,
    update_time_range: Option<TaskQueryUpdateTimeRange>,
}

impl TaskQueryBuilder {
    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    pub fn status_filter(mut self, value: TaskQueryStatusFilter) -> Self {
        self.status_filter = Some(value);
        self
    }

    pub fn update_time_range(mut self, value: TaskQueryUpdateTimeRange) -> Self {
        self.update_time_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskQuery`].
    pub fn build(self) -> Result<TaskQuery, BuildError> {
        Ok(TaskQuery {
            page_token: self.page_token,
            parent_task_id: self.parent_task_id,
            status_filter: self.status_filter,
            update_time_range: self.update_time_range,
        })
    }
}
