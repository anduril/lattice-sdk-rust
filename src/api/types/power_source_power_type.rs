pub use crate::prelude::*;

/// Used to determine the type of power source.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PowerSourcePowerType {
    PowerTypeInvalid,
    PowerTypeUnknown,
    PowerTypeGas,
    PowerTypeBattery,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PowerSourcePowerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PowerTypeInvalid => serializer.serialize_str("POWER_TYPE_INVALID"),
            Self::PowerTypeUnknown => serializer.serialize_str("POWER_TYPE_UNKNOWN"),
            Self::PowerTypeGas => serializer.serialize_str("POWER_TYPE_GAS"),
            Self::PowerTypeBattery => serializer.serialize_str("POWER_TYPE_BATTERY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PowerSourcePowerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "POWER_TYPE_INVALID" => Ok(Self::PowerTypeInvalid),
            "POWER_TYPE_UNKNOWN" => Ok(Self::PowerTypeUnknown),
            "POWER_TYPE_GAS" => Ok(Self::PowerTypeGas),
            "POWER_TYPE_BATTERY" => Ok(Self::PowerTypeBattery),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PowerSourcePowerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PowerTypeInvalid => write!(f, "POWER_TYPE_INVALID"),
            Self::PowerTypeUnknown => write!(f, "POWER_TYPE_UNKNOWN"),
            Self::PowerTypeGas => write!(f, "POWER_TYPE_GAS"),
            Self::PowerTypeBattery => write!(f, "POWER_TYPE_BATTERY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
