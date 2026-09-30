pub use crate::prelude::*;

/// status of the override
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OverrideStatus {
    OverrideStatusInvalid,
    OverrideStatusApplied,
    OverrideStatusPending,
    OverrideStatusTimeout,
    OverrideStatusRejected,
    OverrideStatusDeletionPending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OverrideStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OverrideStatusInvalid => serializer.serialize_str("OVERRIDE_STATUS_INVALID"),
            Self::OverrideStatusApplied => serializer.serialize_str("OVERRIDE_STATUS_APPLIED"),
            Self::OverrideStatusPending => serializer.serialize_str("OVERRIDE_STATUS_PENDING"),
            Self::OverrideStatusTimeout => serializer.serialize_str("OVERRIDE_STATUS_TIMEOUT"),
            Self::OverrideStatusRejected => serializer.serialize_str("OVERRIDE_STATUS_REJECTED"),
            Self::OverrideStatusDeletionPending => {
                serializer.serialize_str("OVERRIDE_STATUS_DELETION_PENDING")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OverrideStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "OVERRIDE_STATUS_INVALID" => Ok(Self::OverrideStatusInvalid),
            "OVERRIDE_STATUS_APPLIED" => Ok(Self::OverrideStatusApplied),
            "OVERRIDE_STATUS_PENDING" => Ok(Self::OverrideStatusPending),
            "OVERRIDE_STATUS_TIMEOUT" => Ok(Self::OverrideStatusTimeout),
            "OVERRIDE_STATUS_REJECTED" => Ok(Self::OverrideStatusRejected),
            "OVERRIDE_STATUS_DELETION_PENDING" => Ok(Self::OverrideStatusDeletionPending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OverrideStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverrideStatusInvalid => write!(f, "OVERRIDE_STATUS_INVALID"),
            Self::OverrideStatusApplied => write!(f, "OVERRIDE_STATUS_APPLIED"),
            Self::OverrideStatusPending => write!(f, "OVERRIDE_STATUS_PENDING"),
            Self::OverrideStatusTimeout => write!(f, "OVERRIDE_STATUS_TIMEOUT"),
            Self::OverrideStatusRejected => write!(f, "OVERRIDE_STATUS_REJECTED"),
            Self::OverrideStatusDeletionPending => write!(f, "OVERRIDE_STATUS_DELETION_PENDING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
