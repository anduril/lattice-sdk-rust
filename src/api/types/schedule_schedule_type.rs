pub use crate::prelude::*;

/// The schedule type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleScheduleType {
    ScheduleTypeInvalid,
    ScheduleTypeZoneEnabled,
    ScheduleTypeZoneTempEnabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScheduleScheduleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ScheduleTypeInvalid => serializer.serialize_str("SCHEDULE_TYPE_INVALID"),
            Self::ScheduleTypeZoneEnabled => serializer.serialize_str("SCHEDULE_TYPE_ZONE_ENABLED"),
            Self::ScheduleTypeZoneTempEnabled => {
                serializer.serialize_str("SCHEDULE_TYPE_ZONE_TEMP_ENABLED")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScheduleScheduleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SCHEDULE_TYPE_INVALID" => Ok(Self::ScheduleTypeInvalid),
            "SCHEDULE_TYPE_ZONE_ENABLED" => Ok(Self::ScheduleTypeZoneEnabled),
            "SCHEDULE_TYPE_ZONE_TEMP_ENABLED" => Ok(Self::ScheduleTypeZoneTempEnabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScheduleScheduleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScheduleTypeInvalid => write!(f, "SCHEDULE_TYPE_INVALID"),
            Self::ScheduleTypeZoneEnabled => write!(f, "SCHEDULE_TYPE_ZONE_ENABLED"),
            Self::ScheduleTypeZoneTempEnabled => write!(f, "SCHEDULE_TYPE_ZONE_TEMP_ENABLED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
