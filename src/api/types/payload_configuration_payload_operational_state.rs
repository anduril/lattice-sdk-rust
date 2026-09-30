pub use crate::prelude::*;

/// The operational state of this payload.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayloadConfigurationPayloadOperationalState {
    PayloadOperationalStateInvalid,
    PayloadOperationalStateOff,
    PayloadOperationalStateNonOperational,
    PayloadOperationalStateDegraded,
    PayloadOperationalStateOperational,
    PayloadOperationalStateOutOfService,
    PayloadOperationalStateUnknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PayloadConfigurationPayloadOperationalState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PayloadOperationalStateInvalid => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_INVALID")
            }
            Self::PayloadOperationalStateOff => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_OFF")
            }
            Self::PayloadOperationalStateNonOperational => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL")
            }
            Self::PayloadOperationalStateDegraded => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_DEGRADED")
            }
            Self::PayloadOperationalStateOperational => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_OPERATIONAL")
            }
            Self::PayloadOperationalStateOutOfService => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE")
            }
            Self::PayloadOperationalStateUnknown => {
                serializer.serialize_str("PAYLOAD_OPERATIONAL_STATE_UNKNOWN")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PayloadConfigurationPayloadOperationalState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PAYLOAD_OPERATIONAL_STATE_INVALID" => Ok(Self::PayloadOperationalStateInvalid),
            "PAYLOAD_OPERATIONAL_STATE_OFF" => Ok(Self::PayloadOperationalStateOff),
            "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL" => {
                Ok(Self::PayloadOperationalStateNonOperational)
            }
            "PAYLOAD_OPERATIONAL_STATE_DEGRADED" => Ok(Self::PayloadOperationalStateDegraded),
            "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL" => Ok(Self::PayloadOperationalStateOperational),
            "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE" => {
                Ok(Self::PayloadOperationalStateOutOfService)
            }
            "PAYLOAD_OPERATIONAL_STATE_UNKNOWN" => Ok(Self::PayloadOperationalStateUnknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PayloadConfigurationPayloadOperationalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadOperationalStateInvalid => write!(f, "PAYLOAD_OPERATIONAL_STATE_INVALID"),
            Self::PayloadOperationalStateOff => write!(f, "PAYLOAD_OPERATIONAL_STATE_OFF"),
            Self::PayloadOperationalStateNonOperational => {
                write!(f, "PAYLOAD_OPERATIONAL_STATE_NON_OPERATIONAL")
            }
            Self::PayloadOperationalStateDegraded => {
                write!(f, "PAYLOAD_OPERATIONAL_STATE_DEGRADED")
            }
            Self::PayloadOperationalStateOperational => {
                write!(f, "PAYLOAD_OPERATIONAL_STATE_OPERATIONAL")
            }
            Self::PayloadOperationalStateOutOfService => {
                write!(f, "PAYLOAD_OPERATIONAL_STATE_OUT_OF_SERVICE")
            }
            Self::PayloadOperationalStateUnknown => write!(f, "PAYLOAD_OPERATIONAL_STATE_UNKNOWN"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
