pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TaskStreamRequestTaskType {
    TaskStreamRequestTaskTypeTaskTypeUrls(TaskStreamRequestTaskTypeTaskTypeUrls),

    TaskStreamRequestTaskTypeTaskTypePrefix(TaskStreamRequestTaskTypeTaskTypePrefix),
}

impl TaskStreamRequestTaskType {
    pub fn is_task_stream_request_task_type_task_type_urls(&self) -> bool {
        matches!(self, Self::TaskStreamRequestTaskTypeTaskTypeUrls(_))
    }

    pub fn is_task_stream_request_task_type_task_type_prefix(&self) -> bool {
        matches!(self, Self::TaskStreamRequestTaskTypeTaskTypePrefix(_))
    }

    pub fn as_task_stream_request_task_type_task_type_urls(
        &self,
    ) -> Option<&TaskStreamRequestTaskTypeTaskTypeUrls> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeUrls(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_task_stream_request_task_type_task_type_urls(
        self,
    ) -> Option<TaskStreamRequestTaskTypeTaskTypeUrls> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeUrls(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_task_stream_request_task_type_task_type_prefix(
        &self,
    ) -> Option<&TaskStreamRequestTaskTypeTaskTypePrefix> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_task_stream_request_task_type_task_type_prefix(
        self,
    ) -> Option<TaskStreamRequestTaskTypeTaskTypePrefix> {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for TaskStreamRequestTaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TaskStreamRequestTaskTypeTaskTypeUrls(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TaskStreamRequestTaskTypeTaskTypePrefix(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
