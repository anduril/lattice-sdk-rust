pub use crate::prelude::*;

/// Reference frame, assumed to be Earth-centered
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrbitMeanElementsMetadataRefFrame {
    EciReferenceFrameInvalid,
    EciReferenceFrameTeme,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrbitMeanElementsMetadataRefFrame {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EciReferenceFrameInvalid => {
                serializer.serialize_str("ECI_REFERENCE_FRAME_INVALID")
            }
            Self::EciReferenceFrameTeme => serializer.serialize_str("ECI_REFERENCE_FRAME_TEME"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrbitMeanElementsMetadataRefFrame {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ECI_REFERENCE_FRAME_INVALID" => Ok(Self::EciReferenceFrameInvalid),
            "ECI_REFERENCE_FRAME_TEME" => Ok(Self::EciReferenceFrameTeme),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrbitMeanElementsMetadataRefFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EciReferenceFrameInvalid => write!(f, "ECI_REFERENCE_FRAME_INVALID"),
            Self::EciReferenceFrameTeme => write!(f, "ECI_REFERENCE_FRAME_TEME"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
