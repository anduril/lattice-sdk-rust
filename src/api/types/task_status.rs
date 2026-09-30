pub use crate::prelude::*;

/// Comprehensive status information for a task at a given point in time.
///
/// TaskStatus contains all status-related information for a task, including its current state,
/// any error conditions, progress details, results, timing information, and resource allocations.
/// This object evolves throughout a task's lifecycle, providing increasing detail as the task
/// progresses from creation through execution to completion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskStatus {
    /// Status of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatusStatus>,
    /// Any errors associated with the task.
    #[serde(rename = "taskError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_error: Option<TaskError>,
    /// Any incremental progress on the task, should be from the tasks/v* /progress folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<GoogleProtobufAny>,
    /// Any final result of the task, should be from tasks/v* /result folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<GoogleProtobufAny>,
    /// Time the task began execution, may not be known even for executing Tasks.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// Any estimate for how the task will progress, should be from tasks/v* /estimates folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<GoogleProtobufAny>,
    /// Any allocated agents of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<Allocation>,
}

impl TaskStatus {
    pub fn builder() -> TaskStatusBuilder {
        <TaskStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskStatusBuilder {
    status: Option<TaskStatusStatus>,
    task_error: Option<TaskError>,
    progress: Option<GoogleProtobufAny>,
    result: Option<GoogleProtobufAny>,
    start_time: Option<DateTime<FixedOffset>>,
    estimate: Option<GoogleProtobufAny>,
    allocation: Option<Allocation>,
}

impl TaskStatusBuilder {
    pub fn status(mut self, value: TaskStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn task_error(mut self, value: TaskError) -> Self {
        self.task_error = Some(value);
        self
    }

    pub fn progress(mut self, value: GoogleProtobufAny) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn result(mut self, value: GoogleProtobufAny) -> Self {
        self.result = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn estimate(mut self, value: GoogleProtobufAny) -> Self {
        self.estimate = Some(value);
        self
    }

    pub fn allocation(mut self, value: Allocation) -> Self {
        self.allocation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskStatus`].
    pub fn build(self) -> Result<TaskStatus, BuildError> {
        Ok(TaskStatus {
            status: self.status,
            task_error: self.task_error,
            progress: self.progress,
            result: self.result,
            start_time: self.start_time,
            estimate: self.estimate,
            allocation: self.allocation,
        })
    }
}
