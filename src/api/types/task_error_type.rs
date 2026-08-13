pub use crate::prelude::*;

/// Error information associated with a task.
///
/// TaskError contains structured error details, including an error code, a human-readable
/// message, and optional extended error information. This structure is used when a task
/// encounters problems during its lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskError {
    /// Error code for task error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<TaskErrorCode>,
    /// Descriptive human-readable string regarding this error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Any additional details regarding this error.
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<GoogleProtobufAny>,
}

impl TaskError {
    pub fn builder() -> TaskErrorBuilder {
        <TaskErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskErrorBuilder {
    code: Option<TaskErrorCode>,
    message: Option<String>,
    error_details: Option<GoogleProtobufAny>,
}

impl TaskErrorBuilder {
    pub fn code(mut self, value: TaskErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn error_details(mut self, value: GoogleProtobufAny) -> Self {
        self.error_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskError`].
    pub fn build(self) -> Result<TaskError, BuildError> {
        Ok(TaskError {
            code: self.code,
            message: self.message,
            error_details: self.error_details,
        })
    }
}
