pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlAreaDetailsType {
    ControlAreaTypeInvalid,
    ControlAreaTypeKeepInZone,
    ControlAreaTypeKeepOutZone,
    ControlAreaTypeDitchZone,
    ControlAreaTypeLoiterZone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ControlAreaDetailsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ControlAreaTypeInvalid => serializer.serialize_str("CONTROL_AREA_TYPE_INVALID"),
            Self::ControlAreaTypeKeepInZone => {
                serializer.serialize_str("CONTROL_AREA_TYPE_KEEP_IN_ZONE")
            }
            Self::ControlAreaTypeKeepOutZone => {
                serializer.serialize_str("CONTROL_AREA_TYPE_KEEP_OUT_ZONE")
            }
            Self::ControlAreaTypeDitchZone => {
                serializer.serialize_str("CONTROL_AREA_TYPE_DITCH_ZONE")
            }
            Self::ControlAreaTypeLoiterZone => {
                serializer.serialize_str("CONTROL_AREA_TYPE_LOITER_ZONE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ControlAreaDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CONTROL_AREA_TYPE_INVALID" => Ok(Self::ControlAreaTypeInvalid),
            "CONTROL_AREA_TYPE_KEEP_IN_ZONE" => Ok(Self::ControlAreaTypeKeepInZone),
            "CONTROL_AREA_TYPE_KEEP_OUT_ZONE" => Ok(Self::ControlAreaTypeKeepOutZone),
            "CONTROL_AREA_TYPE_DITCH_ZONE" => Ok(Self::ControlAreaTypeDitchZone),
            "CONTROL_AREA_TYPE_LOITER_ZONE" => Ok(Self::ControlAreaTypeLoiterZone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ControlAreaDetailsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ControlAreaTypeInvalid => write!(f, "CONTROL_AREA_TYPE_INVALID"),
            Self::ControlAreaTypeKeepInZone => write!(f, "CONTROL_AREA_TYPE_KEEP_IN_ZONE"),
            Self::ControlAreaTypeKeepOutZone => write!(f, "CONTROL_AREA_TYPE_KEEP_OUT_ZONE"),
            Self::ControlAreaTypeDitchZone => write!(f, "CONTROL_AREA_TYPE_DITCH_ZONE"),
            Self::ControlAreaTypeLoiterZone => write!(f, "CONTROL_AREA_TYPE_LOITER_ZONE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
