pub use crate::prelude::*;

/// Status indicating whether the entity is able to communicate with Entity Manager.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthConnectionStatus {
    ConnectionStatusInvalid,
    ConnectionStatusOnline,
    ConnectionStatusOffline,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HealthConnectionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConnectionStatusInvalid => serializer.serialize_str("CONNECTION_STATUS_INVALID"),
            Self::ConnectionStatusOnline => serializer.serialize_str("CONNECTION_STATUS_ONLINE"),
            Self::ConnectionStatusOffline => serializer.serialize_str("CONNECTION_STATUS_OFFLINE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HealthConnectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CONNECTION_STATUS_INVALID" => Ok(Self::ConnectionStatusInvalid),
            "CONNECTION_STATUS_ONLINE" => Ok(Self::ConnectionStatusOnline),
            "CONNECTION_STATUS_OFFLINE" => Ok(Self::ConnectionStatusOffline),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HealthConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionStatusInvalid => write!(f, "CONNECTION_STATUS_INVALID"),
            Self::ConnectionStatusOnline => write!(f, "CONNECTION_STATUS_ONLINE"),
            Self::ConnectionStatusOffline => write!(f, "CONNECTION_STATUS_OFFLINE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
