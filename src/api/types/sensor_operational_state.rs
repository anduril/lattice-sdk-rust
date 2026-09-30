pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SensorOperationalState {
    OperationalStateInvalid,
    OperationalStateOff,
    OperationalStateNonOperational,
    OperationalStateDegraded,
    OperationalStateOperational,
    OperationalStateDenied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SensorOperationalState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OperationalStateInvalid => serializer.serialize_str("OPERATIONAL_STATE_INVALID"),
            Self::OperationalStateOff => serializer.serialize_str("OPERATIONAL_STATE_OFF"),
            Self::OperationalStateNonOperational => {
                serializer.serialize_str("OPERATIONAL_STATE_NON_OPERATIONAL")
            }
            Self::OperationalStateDegraded => {
                serializer.serialize_str("OPERATIONAL_STATE_DEGRADED")
            }
            Self::OperationalStateOperational => {
                serializer.serialize_str("OPERATIONAL_STATE_OPERATIONAL")
            }
            Self::OperationalStateDenied => serializer.serialize_str("OPERATIONAL_STATE_DENIED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SensorOperationalState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "OPERATIONAL_STATE_INVALID" => Ok(Self::OperationalStateInvalid),
            "OPERATIONAL_STATE_OFF" => Ok(Self::OperationalStateOff),
            "OPERATIONAL_STATE_NON_OPERATIONAL" => Ok(Self::OperationalStateNonOperational),
            "OPERATIONAL_STATE_DEGRADED" => Ok(Self::OperationalStateDegraded),
            "OPERATIONAL_STATE_OPERATIONAL" => Ok(Self::OperationalStateOperational),
            "OPERATIONAL_STATE_DENIED" => Ok(Self::OperationalStateDenied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SensorOperationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperationalStateInvalid => write!(f, "OPERATIONAL_STATE_INVALID"),
            Self::OperationalStateOff => write!(f, "OPERATIONAL_STATE_OFF"),
            Self::OperationalStateNonOperational => write!(f, "OPERATIONAL_STATE_NON_OPERATIONAL"),
            Self::OperationalStateDegraded => write!(f, "OPERATIONAL_STATE_DEGRADED"),
            Self::OperationalStateOperational => write!(f, "OPERATIONAL_STATE_OPERATIONAL"),
            Self::OperationalStateDenied => write!(f, "OPERATIONAL_STATE_DENIED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
