pub use crate::prelude::*;

/// Health for this component.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentHealthHealth {
    HealthStatusInvalid,
    HealthStatusHealthy,
    HealthStatusWarn,
    HealthStatusFail,
    HealthStatusOffline,
    HealthStatusNotReady,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ComponentHealthHealth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::HealthStatusInvalid => serializer.serialize_str("HEALTH_STATUS_INVALID"),
            Self::HealthStatusHealthy => serializer.serialize_str("HEALTH_STATUS_HEALTHY"),
            Self::HealthStatusWarn => serializer.serialize_str("HEALTH_STATUS_WARN"),
            Self::HealthStatusFail => serializer.serialize_str("HEALTH_STATUS_FAIL"),
            Self::HealthStatusOffline => serializer.serialize_str("HEALTH_STATUS_OFFLINE"),
            Self::HealthStatusNotReady => serializer.serialize_str("HEALTH_STATUS_NOT_READY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ComponentHealthHealth {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "HEALTH_STATUS_INVALID" => Ok(Self::HealthStatusInvalid),
            "HEALTH_STATUS_HEALTHY" => Ok(Self::HealthStatusHealthy),
            "HEALTH_STATUS_WARN" => Ok(Self::HealthStatusWarn),
            "HEALTH_STATUS_FAIL" => Ok(Self::HealthStatusFail),
            "HEALTH_STATUS_OFFLINE" => Ok(Self::HealthStatusOffline),
            "HEALTH_STATUS_NOT_READY" => Ok(Self::HealthStatusNotReady),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ComponentHealthHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HealthStatusInvalid => write!(f, "HEALTH_STATUS_INVALID"),
            Self::HealthStatusHealthy => write!(f, "HEALTH_STATUS_HEALTHY"),
            Self::HealthStatusWarn => write!(f, "HEALTH_STATUS_WARN"),
            Self::HealthStatusFail => write!(f, "HEALTH_STATUS_FAIL"),
            Self::HealthStatusOffline => write!(f, "HEALTH_STATUS_OFFLINE"),
            Self::HealthStatusNotReady => write!(f, "HEALTH_STATUS_NOT_READY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
