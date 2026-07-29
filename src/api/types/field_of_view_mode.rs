pub use crate::prelude::*;

/// The mode that this sensor is currently in, used to display for context in the UI. Some sensors can emit multiple
/// sensor field of views with different modes, for example a radar can simultaneously search broadly and perform
/// tighter bounded tracking.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldOfViewMode {
    SensorModeInvalid,
    SensorModeSearch,
    SensorModeTrack,
    SensorModeWeaponSupport,
    SensorModeAuto,
    SensorModeMute,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FieldOfViewMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SensorModeInvalid => serializer.serialize_str("SENSOR_MODE_INVALID"),
            Self::SensorModeSearch => serializer.serialize_str("SENSOR_MODE_SEARCH"),
            Self::SensorModeTrack => serializer.serialize_str("SENSOR_MODE_TRACK"),
            Self::SensorModeWeaponSupport => serializer.serialize_str("SENSOR_MODE_WEAPON_SUPPORT"),
            Self::SensorModeAuto => serializer.serialize_str("SENSOR_MODE_AUTO"),
            Self::SensorModeMute => serializer.serialize_str("SENSOR_MODE_MUTE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FieldOfViewMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SENSOR_MODE_INVALID" => Ok(Self::SensorModeInvalid),
            "SENSOR_MODE_SEARCH" => Ok(Self::SensorModeSearch),
            "SENSOR_MODE_TRACK" => Ok(Self::SensorModeTrack),
            "SENSOR_MODE_WEAPON_SUPPORT" => Ok(Self::SensorModeWeaponSupport),
            "SENSOR_MODE_AUTO" => Ok(Self::SensorModeAuto),
            "SENSOR_MODE_MUTE" => Ok(Self::SensorModeMute),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FieldOfViewMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SensorModeInvalid => write!(f, "SENSOR_MODE_INVALID"),
            Self::SensorModeSearch => write!(f, "SENSOR_MODE_SEARCH"),
            Self::SensorModeTrack => write!(f, "SENSOR_MODE_TRACK"),
            Self::SensorModeWeaponSupport => write!(f, "SENSOR_MODE_WEAPON_SUPPORT"),
            Self::SensorModeAuto => write!(f, "SENSOR_MODE_AUTO"),
            Self::SensorModeMute => write!(f, "SENSOR_MODE_MUTE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
