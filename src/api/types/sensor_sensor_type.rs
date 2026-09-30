pub use crate::prelude::*;

/// The type of sensor
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SensorSensorType {
    SensorTypeInvalid,
    SensorTypeRadar,
    SensorTypeCamera,
    SensorTypeTransponder,
    SensorTypeRf,
    SensorTypeGps,
    SensorTypePtuPos,
    SensorTypePerimeter,
    SensorTypeSonar,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SensorSensorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SensorTypeInvalid => serializer.serialize_str("SENSOR_TYPE_INVALID"),
            Self::SensorTypeRadar => serializer.serialize_str("SENSOR_TYPE_RADAR"),
            Self::SensorTypeCamera => serializer.serialize_str("SENSOR_TYPE_CAMERA"),
            Self::SensorTypeTransponder => serializer.serialize_str("SENSOR_TYPE_TRANSPONDER"),
            Self::SensorTypeRf => serializer.serialize_str("SENSOR_TYPE_RF"),
            Self::SensorTypeGps => serializer.serialize_str("SENSOR_TYPE_GPS"),
            Self::SensorTypePtuPos => serializer.serialize_str("SENSOR_TYPE_PTU_POS"),
            Self::SensorTypePerimeter => serializer.serialize_str("SENSOR_TYPE_PERIMETER"),
            Self::SensorTypeSonar => serializer.serialize_str("SENSOR_TYPE_SONAR"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SensorSensorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SENSOR_TYPE_INVALID" => Ok(Self::SensorTypeInvalid),
            "SENSOR_TYPE_RADAR" => Ok(Self::SensorTypeRadar),
            "SENSOR_TYPE_CAMERA" => Ok(Self::SensorTypeCamera),
            "SENSOR_TYPE_TRANSPONDER" => Ok(Self::SensorTypeTransponder),
            "SENSOR_TYPE_RF" => Ok(Self::SensorTypeRf),
            "SENSOR_TYPE_GPS" => Ok(Self::SensorTypeGps),
            "SENSOR_TYPE_PTU_POS" => Ok(Self::SensorTypePtuPos),
            "SENSOR_TYPE_PERIMETER" => Ok(Self::SensorTypePerimeter),
            "SENSOR_TYPE_SONAR" => Ok(Self::SensorTypeSonar),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SensorSensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SensorTypeInvalid => write!(f, "SENSOR_TYPE_INVALID"),
            Self::SensorTypeRadar => write!(f, "SENSOR_TYPE_RADAR"),
            Self::SensorTypeCamera => write!(f, "SENSOR_TYPE_CAMERA"),
            Self::SensorTypeTransponder => write!(f, "SENSOR_TYPE_TRANSPONDER"),
            Self::SensorTypeRf => write!(f, "SENSOR_TYPE_RF"),
            Self::SensorTypeGps => write!(f, "SENSOR_TYPE_GPS"),
            Self::SensorTypePtuPos => write!(f, "SENSOR_TYPE_PTU_POS"),
            Self::SensorTypePerimeter => write!(f, "SENSOR_TYPE_PERIMETER"),
            Self::SensorTypeSonar => write!(f, "SENSOR_TYPE_SONAR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
