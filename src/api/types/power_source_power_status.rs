pub use crate::prelude::*;

/// Status of the power source.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PowerSourcePowerStatus {
    PowerStatusInvalid,
    PowerStatusUnknown,
    PowerStatusNotPresent,
    PowerStatusOperating,
    PowerStatusDisabled,
    PowerStatusError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PowerSourcePowerStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PowerStatusInvalid => serializer.serialize_str("POWER_STATUS_INVALID"),
            Self::PowerStatusUnknown => serializer.serialize_str("POWER_STATUS_UNKNOWN"),
            Self::PowerStatusNotPresent => serializer.serialize_str("POWER_STATUS_NOT_PRESENT"),
            Self::PowerStatusOperating => serializer.serialize_str("POWER_STATUS_OPERATING"),
            Self::PowerStatusDisabled => serializer.serialize_str("POWER_STATUS_DISABLED"),
            Self::PowerStatusError => serializer.serialize_str("POWER_STATUS_ERROR"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PowerSourcePowerStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "POWER_STATUS_INVALID" => Ok(Self::PowerStatusInvalid),
            "POWER_STATUS_UNKNOWN" => Ok(Self::PowerStatusUnknown),
            "POWER_STATUS_NOT_PRESENT" => Ok(Self::PowerStatusNotPresent),
            "POWER_STATUS_OPERATING" => Ok(Self::PowerStatusOperating),
            "POWER_STATUS_DISABLED" => Ok(Self::PowerStatusDisabled),
            "POWER_STATUS_ERROR" => Ok(Self::PowerStatusError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PowerSourcePowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PowerStatusInvalid => write!(f, "POWER_STATUS_INVALID"),
            Self::PowerStatusUnknown => write!(f, "POWER_STATUS_UNKNOWN"),
            Self::PowerStatusNotPresent => write!(f, "POWER_STATUS_NOT_PRESENT"),
            Self::PowerStatusOperating => write!(f, "POWER_STATUS_OPERATING"),
            Self::PowerStatusDisabled => write!(f, "POWER_STATUS_DISABLED"),
            Self::PowerStatusError => write!(f, "POWER_STATUS_ERROR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
