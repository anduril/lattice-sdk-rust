pub use crate::prelude::*;

/// What type of (de)correlation was this entity added with.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorrelationMetadataType {
    CorrelationTypeInvalid,
    CorrelationTypeManual,
    CorrelationTypeAutomated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CorrelationMetadataType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CorrelationTypeInvalid => serializer.serialize_str("CORRELATION_TYPE_INVALID"),
            Self::CorrelationTypeManual => serializer.serialize_str("CORRELATION_TYPE_MANUAL"),
            Self::CorrelationTypeAutomated => {
                serializer.serialize_str("CORRELATION_TYPE_AUTOMATED")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CorrelationMetadataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CORRELATION_TYPE_INVALID" => Ok(Self::CorrelationTypeInvalid),
            "CORRELATION_TYPE_MANUAL" => Ok(Self::CorrelationTypeManual),
            "CORRELATION_TYPE_AUTOMATED" => Ok(Self::CorrelationTypeAutomated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CorrelationMetadataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorrelationTypeInvalid => write!(f, "CORRELATION_TYPE_INVALID"),
            Self::CorrelationTypeManual => write!(f, "CORRELATION_TYPE_MANUAL"),
            Self::CorrelationTypeAutomated => write!(f, "CORRELATION_TYPE_AUTOMATED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
