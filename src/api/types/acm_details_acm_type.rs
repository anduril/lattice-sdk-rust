pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AcmDetailsAcmType {
    AcmDetailTypeInvalid,
    AcmDetailTypeLandingZone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AcmDetailsAcmType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AcmDetailTypeInvalid => serializer.serialize_str("ACM_DETAIL_TYPE_INVALID"),
            Self::AcmDetailTypeLandingZone => {
                serializer.serialize_str("ACM_DETAIL_TYPE_LANDING_ZONE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AcmDetailsAcmType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ACM_DETAIL_TYPE_INVALID" => Ok(Self::AcmDetailTypeInvalid),
            "ACM_DETAIL_TYPE_LANDING_ZONE" => Ok(Self::AcmDetailTypeLandingZone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AcmDetailsAcmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcmDetailTypeInvalid => write!(f, "ACM_DETAIL_TYPE_INVALID"),
            Self::AcmDetailTypeLandingZone => write!(f, "ACM_DETAIL_TYPE_LANDING_ZONE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
