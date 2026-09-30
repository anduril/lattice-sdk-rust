pub use crate::prelude::*;

/// Meaning of alt.
/// altitude in meters above either WGS84 or EGM96, use altitude_reference to
/// determine what zero means.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlaAltitudeReference {
    AltitudeReferenceInvalid,
    AltitudeReferenceHeightAboveWgs84,
    AltitudeReferenceHeightAboveEgm96,
    AltitudeReferenceUnknown,
    AltitudeReferenceBarometric,
    AltitudeReferenceAboveSeaFloor,
    AltitudeReferenceBelowSeaSurface,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LlaAltitudeReference {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AltitudeReferenceInvalid => {
                serializer.serialize_str("ALTITUDE_REFERENCE_INVALID")
            }
            Self::AltitudeReferenceHeightAboveWgs84 => {
                serializer.serialize_str("ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84")
            }
            Self::AltitudeReferenceHeightAboveEgm96 => {
                serializer.serialize_str("ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96")
            }
            Self::AltitudeReferenceUnknown => {
                serializer.serialize_str("ALTITUDE_REFERENCE_UNKNOWN")
            }
            Self::AltitudeReferenceBarometric => {
                serializer.serialize_str("ALTITUDE_REFERENCE_BAROMETRIC")
            }
            Self::AltitudeReferenceAboveSeaFloor => {
                serializer.serialize_str("ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR")
            }
            Self::AltitudeReferenceBelowSeaSurface => {
                serializer.serialize_str("ALTITUDE_REFERENCE_BELOW_SEA_SURFACE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LlaAltitudeReference {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALTITUDE_REFERENCE_INVALID" => Ok(Self::AltitudeReferenceInvalid),
            "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84" => Ok(Self::AltitudeReferenceHeightAboveWgs84),
            "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96" => Ok(Self::AltitudeReferenceHeightAboveEgm96),
            "ALTITUDE_REFERENCE_UNKNOWN" => Ok(Self::AltitudeReferenceUnknown),
            "ALTITUDE_REFERENCE_BAROMETRIC" => Ok(Self::AltitudeReferenceBarometric),
            "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR" => Ok(Self::AltitudeReferenceAboveSeaFloor),
            "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE" => Ok(Self::AltitudeReferenceBelowSeaSurface),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LlaAltitudeReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AltitudeReferenceInvalid => write!(f, "ALTITUDE_REFERENCE_INVALID"),
            Self::AltitudeReferenceHeightAboveWgs84 => {
                write!(f, "ALTITUDE_REFERENCE_HEIGHT_ABOVE_WGS84")
            }
            Self::AltitudeReferenceHeightAboveEgm96 => {
                write!(f, "ALTITUDE_REFERENCE_HEIGHT_ABOVE_EGM96")
            }
            Self::AltitudeReferenceUnknown => write!(f, "ALTITUDE_REFERENCE_UNKNOWN"),
            Self::AltitudeReferenceBarometric => write!(f, "ALTITUDE_REFERENCE_BAROMETRIC"),
            Self::AltitudeReferenceAboveSeaFloor => write!(f, "ALTITUDE_REFERENCE_ABOVE_SEA_FLOOR"),
            Self::AltitudeReferenceBelowSeaSurface => {
                write!(f, "ALTITUDE_REFERENCE_BELOW_SEA_SURFACE")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
