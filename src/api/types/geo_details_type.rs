pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoDetailsType {
    GeoTypeInvalid,
    GeoTypeGeneral,
    GeoTypeHazard,
    GeoTypeEmergency,
    GeoTypeEngagementZone,
    GeoTypeControlArea,
    GeoTypeBullseye,
    GeoTypeAcm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GeoDetailsType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GeoTypeInvalid => serializer.serialize_str("GEO_TYPE_INVALID"),
            Self::GeoTypeGeneral => serializer.serialize_str("GEO_TYPE_GENERAL"),
            Self::GeoTypeHazard => serializer.serialize_str("GEO_TYPE_HAZARD"),
            Self::GeoTypeEmergency => serializer.serialize_str("GEO_TYPE_EMERGENCY"),
            Self::GeoTypeEngagementZone => serializer.serialize_str("GEO_TYPE_ENGAGEMENT_ZONE"),
            Self::GeoTypeControlArea => serializer.serialize_str("GEO_TYPE_CONTROL_AREA"),
            Self::GeoTypeBullseye => serializer.serialize_str("GEO_TYPE_BULLSEYE"),
            Self::GeoTypeAcm => serializer.serialize_str("GEO_TYPE_ACM"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GeoDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GEO_TYPE_INVALID" => Ok(Self::GeoTypeInvalid),
            "GEO_TYPE_GENERAL" => Ok(Self::GeoTypeGeneral),
            "GEO_TYPE_HAZARD" => Ok(Self::GeoTypeHazard),
            "GEO_TYPE_EMERGENCY" => Ok(Self::GeoTypeEmergency),
            "GEO_TYPE_ENGAGEMENT_ZONE" => Ok(Self::GeoTypeEngagementZone),
            "GEO_TYPE_CONTROL_AREA" => Ok(Self::GeoTypeControlArea),
            "GEO_TYPE_BULLSEYE" => Ok(Self::GeoTypeBullseye),
            "GEO_TYPE_ACM" => Ok(Self::GeoTypeAcm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GeoDetailsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GeoTypeInvalid => write!(f, "GEO_TYPE_INVALID"),
            Self::GeoTypeGeneral => write!(f, "GEO_TYPE_GENERAL"),
            Self::GeoTypeHazard => write!(f, "GEO_TYPE_HAZARD"),
            Self::GeoTypeEmergency => write!(f, "GEO_TYPE_EMERGENCY"),
            Self::GeoTypeEngagementZone => write!(f, "GEO_TYPE_ENGAGEMENT_ZONE"),
            Self::GeoTypeControlArea => write!(f, "GEO_TYPE_CONTROL_AREA"),
            Self::GeoTypeBullseye => write!(f, "GEO_TYPE_BULLSEYE"),
            Self::GeoTypeAcm => write!(f, "GEO_TYPE_ACM"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
