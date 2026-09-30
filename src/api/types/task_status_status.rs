pub use crate::prelude::*;

/// Status of the task.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskStatusStatus {
    StatusInvalid,
    StatusCreated,
    StatusScheduledInManager,
    StatusSent,
    StatusMachineReceipt,
    StatusAck,
    StatusWilco,
    StatusExecuting,
    StatusWaitingForUpdate,
    StatusDoneOk,
    StatusDoneNotOk,
    StatusReplaced,
    StatusCancelRequested,
    StatusCompleteRequested,
    StatusVersionRejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskStatusStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StatusInvalid => serializer.serialize_str("STATUS_INVALID"),
            Self::StatusCreated => serializer.serialize_str("STATUS_CREATED"),
            Self::StatusScheduledInManager => {
                serializer.serialize_str("STATUS_SCHEDULED_IN_MANAGER")
            }
            Self::StatusSent => serializer.serialize_str("STATUS_SENT"),
            Self::StatusMachineReceipt => serializer.serialize_str("STATUS_MACHINE_RECEIPT"),
            Self::StatusAck => serializer.serialize_str("STATUS_ACK"),
            Self::StatusWilco => serializer.serialize_str("STATUS_WILCO"),
            Self::StatusExecuting => serializer.serialize_str("STATUS_EXECUTING"),
            Self::StatusWaitingForUpdate => serializer.serialize_str("STATUS_WAITING_FOR_UPDATE"),
            Self::StatusDoneOk => serializer.serialize_str("STATUS_DONE_OK"),
            Self::StatusDoneNotOk => serializer.serialize_str("STATUS_DONE_NOT_OK"),
            Self::StatusReplaced => serializer.serialize_str("STATUS_REPLACED"),
            Self::StatusCancelRequested => serializer.serialize_str("STATUS_CANCEL_REQUESTED"),
            Self::StatusCompleteRequested => serializer.serialize_str("STATUS_COMPLETE_REQUESTED"),
            Self::StatusVersionRejected => serializer.serialize_str("STATUS_VERSION_REJECTED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "STATUS_INVALID" => Ok(Self::StatusInvalid),
            "STATUS_CREATED" => Ok(Self::StatusCreated),
            "STATUS_SCHEDULED_IN_MANAGER" => Ok(Self::StatusScheduledInManager),
            "STATUS_SENT" => Ok(Self::StatusSent),
            "STATUS_MACHINE_RECEIPT" => Ok(Self::StatusMachineReceipt),
            "STATUS_ACK" => Ok(Self::StatusAck),
            "STATUS_WILCO" => Ok(Self::StatusWilco),
            "STATUS_EXECUTING" => Ok(Self::StatusExecuting),
            "STATUS_WAITING_FOR_UPDATE" => Ok(Self::StatusWaitingForUpdate),
            "STATUS_DONE_OK" => Ok(Self::StatusDoneOk),
            "STATUS_DONE_NOT_OK" => Ok(Self::StatusDoneNotOk),
            "STATUS_REPLACED" => Ok(Self::StatusReplaced),
            "STATUS_CANCEL_REQUESTED" => Ok(Self::StatusCancelRequested),
            "STATUS_COMPLETE_REQUESTED" => Ok(Self::StatusCompleteRequested),
            "STATUS_VERSION_REJECTED" => Ok(Self::StatusVersionRejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StatusInvalid => write!(f, "STATUS_INVALID"),
            Self::StatusCreated => write!(f, "STATUS_CREATED"),
            Self::StatusScheduledInManager => write!(f, "STATUS_SCHEDULED_IN_MANAGER"),
            Self::StatusSent => write!(f, "STATUS_SENT"),
            Self::StatusMachineReceipt => write!(f, "STATUS_MACHINE_RECEIPT"),
            Self::StatusAck => write!(f, "STATUS_ACK"),
            Self::StatusWilco => write!(f, "STATUS_WILCO"),
            Self::StatusExecuting => write!(f, "STATUS_EXECUTING"),
            Self::StatusWaitingForUpdate => write!(f, "STATUS_WAITING_FOR_UPDATE"),
            Self::StatusDoneOk => write!(f, "STATUS_DONE_OK"),
            Self::StatusDoneNotOk => write!(f, "STATUS_DONE_NOT_OK"),
            Self::StatusReplaced => write!(f, "STATUS_REPLACED"),
            Self::StatusCancelRequested => write!(f, "STATUS_CANCEL_REQUESTED"),
            Self::StatusCompleteRequested => write!(f, "STATUS_COMPLETE_REQUESTED"),
            Self::StatusVersionRejected => write!(f, "STATUS_VERSION_REJECTED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
