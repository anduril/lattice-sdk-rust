pub use crate::prelude::*;

/// The template used when creating this entity. Specifies minimum required components.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OntologyTemplate {
    TemplateInvalid,
    TemplateTrack,
    TemplateSensorPointOfInterest,
    TemplateAsset,
    TemplateGeo,
    TemplateSignalOfInterest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OntologyTemplate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TemplateInvalid => serializer.serialize_str("TEMPLATE_INVALID"),
            Self::TemplateTrack => serializer.serialize_str("TEMPLATE_TRACK"),
            Self::TemplateSensorPointOfInterest => {
                serializer.serialize_str("TEMPLATE_SENSOR_POINT_OF_INTEREST")
            }
            Self::TemplateAsset => serializer.serialize_str("TEMPLATE_ASSET"),
            Self::TemplateGeo => serializer.serialize_str("TEMPLATE_GEO"),
            Self::TemplateSignalOfInterest => {
                serializer.serialize_str("TEMPLATE_SIGNAL_OF_INTEREST")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OntologyTemplate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TEMPLATE_INVALID" => Ok(Self::TemplateInvalid),
            "TEMPLATE_TRACK" => Ok(Self::TemplateTrack),
            "TEMPLATE_SENSOR_POINT_OF_INTEREST" => Ok(Self::TemplateSensorPointOfInterest),
            "TEMPLATE_ASSET" => Ok(Self::TemplateAsset),
            "TEMPLATE_GEO" => Ok(Self::TemplateGeo),
            "TEMPLATE_SIGNAL_OF_INTEREST" => Ok(Self::TemplateSignalOfInterest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OntologyTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TemplateInvalid => write!(f, "TEMPLATE_INVALID"),
            Self::TemplateTrack => write!(f, "TEMPLATE_TRACK"),
            Self::TemplateSensorPointOfInterest => write!(f, "TEMPLATE_SENSOR_POINT_OF_INTEREST"),
            Self::TemplateAsset => write!(f, "TEMPLATE_ASSET"),
            Self::TemplateGeo => write!(f, "TEMPLATE_GEO"),
            Self::TemplateSignalOfInterest => write!(f, "TEMPLATE_SIGNAL_OF_INTEREST"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
