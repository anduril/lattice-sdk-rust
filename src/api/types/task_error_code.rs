pub use crate::prelude::*;

/// Error code for task error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskErrorCode {
    ErrorCodeInvalid,
    ErrorCodeCancelled,
    ErrorCodeRejected,
    ErrorCodeTimeout,
    ErrorCodeFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ErrorCodeInvalid => serializer.serialize_str("ERROR_CODE_INVALID"),
            Self::ErrorCodeCancelled => serializer.serialize_str("ERROR_CODE_CANCELLED"),
            Self::ErrorCodeRejected => serializer.serialize_str("ERROR_CODE_REJECTED"),
            Self::ErrorCodeTimeout => serializer.serialize_str("ERROR_CODE_TIMEOUT"),
            Self::ErrorCodeFailed => serializer.serialize_str("ERROR_CODE_FAILED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ERROR_CODE_INVALID" => Ok(Self::ErrorCodeInvalid),
            "ERROR_CODE_CANCELLED" => Ok(Self::ErrorCodeCancelled),
            "ERROR_CODE_REJECTED" => Ok(Self::ErrorCodeRejected),
            "ERROR_CODE_TIMEOUT" => Ok(Self::ErrorCodeTimeout),
            "ERROR_CODE_FAILED" => Ok(Self::ErrorCodeFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorCodeInvalid => write!(f, "ERROR_CODE_INVALID"),
            Self::ErrorCodeCancelled => write!(f, "ERROR_CODE_CANCELLED"),
            Self::ErrorCodeRejected => write!(f, "ERROR_CODE_REJECTED"),
            Self::ErrorCodeTimeout => write!(f, "ERROR_CODE_TIMEOUT"),
            Self::ErrorCodeFailed => write!(f, "ERROR_CODE_FAILED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
