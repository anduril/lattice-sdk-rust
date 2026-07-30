pub use crate::prelude::*;

/// The type of the override, defined by the stage of the entity lifecycle that the entity was in when the override
/// was requested.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OverrideType {
    OverrideTypeInvalid,
    OverrideTypeLive,
    OverrideTypePostExpiry,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OverrideType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OverrideTypeInvalid => serializer.serialize_str("OVERRIDE_TYPE_INVALID"),
            Self::OverrideTypeLive => serializer.serialize_str("OVERRIDE_TYPE_LIVE"),
            Self::OverrideTypePostExpiry => serializer.serialize_str("OVERRIDE_TYPE_POST_EXPIRY"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OverrideType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "OVERRIDE_TYPE_INVALID" => Ok(Self::OverrideTypeInvalid),
            "OVERRIDE_TYPE_LIVE" => Ok(Self::OverrideTypeLive),
            "OVERRIDE_TYPE_POST_EXPIRY" => Ok(Self::OverrideTypePostExpiry),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OverrideType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OverrideTypeInvalid => write!(f, "OVERRIDE_TYPE_INVALID"),
            Self::OverrideTypeLive => write!(f, "OVERRIDE_TYPE_LIVE"),
            Self::OverrideTypePostExpiry => write!(f, "OVERRIDE_TYPE_POST_EXPIRY"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
