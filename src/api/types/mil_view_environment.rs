pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MilViewEnvironment {
    EnvironmentUnknown,
    EnvironmentAir,
    EnvironmentSurface,
    EnvironmentSubSurface,
    EnvironmentLand,
    EnvironmentSpace,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MilViewEnvironment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EnvironmentUnknown => serializer.serialize_str("ENVIRONMENT_UNKNOWN"),
            Self::EnvironmentAir => serializer.serialize_str("ENVIRONMENT_AIR"),
            Self::EnvironmentSurface => serializer.serialize_str("ENVIRONMENT_SURFACE"),
            Self::EnvironmentSubSurface => serializer.serialize_str("ENVIRONMENT_SUB_SURFACE"),
            Self::EnvironmentLand => serializer.serialize_str("ENVIRONMENT_LAND"),
            Self::EnvironmentSpace => serializer.serialize_str("ENVIRONMENT_SPACE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MilViewEnvironment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ENVIRONMENT_UNKNOWN" => Ok(Self::EnvironmentUnknown),
            "ENVIRONMENT_AIR" => Ok(Self::EnvironmentAir),
            "ENVIRONMENT_SURFACE" => Ok(Self::EnvironmentSurface),
            "ENVIRONMENT_SUB_SURFACE" => Ok(Self::EnvironmentSubSurface),
            "ENVIRONMENT_LAND" => Ok(Self::EnvironmentLand),
            "ENVIRONMENT_SPACE" => Ok(Self::EnvironmentSpace),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MilViewEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvironmentUnknown => write!(f, "ENVIRONMENT_UNKNOWN"),
            Self::EnvironmentAir => write!(f, "ENVIRONMENT_AIR"),
            Self::EnvironmentSurface => write!(f, "ENVIRONMENT_SURFACE"),
            Self::EnvironmentSubSurface => write!(f, "ENVIRONMENT_SUB_SURFACE"),
            Self::EnvironmentLand => write!(f, "ENVIRONMENT_LAND"),
            Self::EnvironmentSpace => write!(f, "ENVIRONMENT_SPACE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
