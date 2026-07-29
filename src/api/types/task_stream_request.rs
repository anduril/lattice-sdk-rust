pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskStreamRequest {
    /// The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    #[serde(rename = "heartbeatIntervalMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval_ms: Option<i64>,
    /// The time interval, in milliseconds, after an update for a given task before another one will be sent for the same task.
    /// If set, value must be >= 250.
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i64>,
    /// Optional flag to only include tasks created or updated after the stream is initiated, and not any previous preexisting tasks.
    /// If unset or false, the stream will include any new tasks and task updates, as well as all preexisting tasks.
    #[serde(rename = "excludePreexistingTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_preexisting_tasks: Option<bool>,
    /// Optional filter that only returns tasks with specific types. If not provided, all task types will be streamed.
    #[serde(rename = "taskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskStreamRequestTaskType>,
    /// If provided, returns tasks which have been updated since the given time.
    #[serde(rename = "updateStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_start_time: Option<Timestamp>,
    /// A filter for tasks with a specific parent task ID.
    /// Note: This filter is mutually exclusive with all other filter fields (`updateStartTime`, `assignee`, `statusFilter`, `taskType`).
    /// Either provide `parentTaskId` or any combination of the other filters, but not both.
    #[serde(rename = "parentTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    /// A filter for tasks assigned to a specific principal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Principal>,
    /// A filter for task statuses (inclusive or exclusive).
    #[serde(rename = "statusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<TaskStreamRequestStatusFilter>,
}

impl TaskStreamRequest {
    pub fn builder() -> TaskStreamRequestBuilder {
        <TaskStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStreamRequestBuilder {
    heartbeat_interval_ms: Option<i64>,
    rate_limit: Option<i64>,
    exclude_preexisting_tasks: Option<bool>,
    task_type: Option<TaskStreamRequestTaskType>,
    update_start_time: Option<Timestamp>,
    parent_task_id: Option<String>,
    assignee: Option<Principal>,
    status_filter: Option<TaskStreamRequestStatusFilter>,
}

impl TaskStreamRequestBuilder {
    pub fn heartbeat_interval_ms(mut self, value: i64) -> Self {
        self.heartbeat_interval_ms = Some(value);
        self
    }

    pub fn rate_limit(mut self, value: i64) -> Self {
        self.rate_limit = Some(value);
        self
    }

    pub fn exclude_preexisting_tasks(mut self, value: bool) -> Self {
        self.exclude_preexisting_tasks = Some(value);
        self
    }

    pub fn task_type(mut self, value: TaskStreamRequestTaskType) -> Self {
        self.task_type = Some(value);
        self
    }

    pub fn update_start_time(mut self, value: Timestamp) -> Self {
        self.update_start_time = Some(value);
        self
    }

    pub fn parent_task_id(mut self, value: impl Into<String>) -> Self {
        self.parent_task_id = Some(value.into());
        self
    }

    pub fn assignee(mut self, value: Principal) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn status_filter(mut self, value: TaskStreamRequestStatusFilter) -> Self {
        self.status_filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStreamRequest`].
    pub fn build(self) -> Result<TaskStreamRequest, BuildError> {
        Ok(TaskStreamRequest {
            heartbeat_interval_ms: self.heartbeat_interval_ms,
            rate_limit: self.rate_limit,
            exclude_preexisting_tasks: self.exclude_preexisting_tasks,
            task_type: self.task_type,
            update_start_time: self.update_start_time,
            parent_task_id: self.parent_task_id,
            assignee: self.assignee,
            status_filter: self.status_filter,
        })
    }
}
