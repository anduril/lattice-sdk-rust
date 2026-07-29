pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AltitudeProvenanceSourceType {
    AltitudeProvenanceTypeInvalid,
    AltitudeProvenanceTypeRadarAltimeter,
    AltitudeProvenanceTypeLaserAltimeter,
    AltitudeProvenanceTypeBarometer,
    AltitudeProvenanceTypeTerrainConversion,
    AltitudeProvenanceTypeGnss,
    AltitudeProvenanceTypeSonar,
    AltitudeProvenanceTypeUserInput,
    AltitudeProvenanceTypeIns,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AltitudeProvenanceSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AltitudeProvenanceTypeInvalid => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_INVALID")
            }
            Self::AltitudeProvenanceTypeRadarAltimeter => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_RADAR_ALTIMETER")
            }
            Self::AltitudeProvenanceTypeLaserAltimeter => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_LASER_ALTIMETER")
            }
            Self::AltitudeProvenanceTypeBarometer => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_BAROMETER")
            }
            Self::AltitudeProvenanceTypeTerrainConversion => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_TERRAIN_CONVERSION")
            }
            Self::AltitudeProvenanceTypeGnss => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_GNSS")
            }
            Self::AltitudeProvenanceTypeSonar => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_SONAR")
            }
            Self::AltitudeProvenanceTypeUserInput => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_USER_INPUT")
            }
            Self::AltitudeProvenanceTypeIns => {
                serializer.serialize_str("ALTITUDE_PROVENANCE_TYPE_INS")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AltitudeProvenanceSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALTITUDE_PROVENANCE_TYPE_INVALID" => Ok(Self::AltitudeProvenanceTypeInvalid),
            "ALTITUDE_PROVENANCE_TYPE_RADAR_ALTIMETER" => {
                Ok(Self::AltitudeProvenanceTypeRadarAltimeter)
            }
            "ALTITUDE_PROVENANCE_TYPE_LASER_ALTIMETER" => {
                Ok(Self::AltitudeProvenanceTypeLaserAltimeter)
            }
            "ALTITUDE_PROVENANCE_TYPE_BAROMETER" => Ok(Self::AltitudeProvenanceTypeBarometer),
            "ALTITUDE_PROVENANCE_TYPE_TERRAIN_CONVERSION" => {
                Ok(Self::AltitudeProvenanceTypeTerrainConversion)
            }
            "ALTITUDE_PROVENANCE_TYPE_GNSS" => Ok(Self::AltitudeProvenanceTypeGnss),
            "ALTITUDE_PROVENANCE_TYPE_SONAR" => Ok(Self::AltitudeProvenanceTypeSonar),
            "ALTITUDE_PROVENANCE_TYPE_USER_INPUT" => Ok(Self::AltitudeProvenanceTypeUserInput),
            "ALTITUDE_PROVENANCE_TYPE_INS" => Ok(Self::AltitudeProvenanceTypeIns),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AltitudeProvenanceSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AltitudeProvenanceTypeInvalid => write!(f, "ALTITUDE_PROVENANCE_TYPE_INVALID"),
            Self::AltitudeProvenanceTypeRadarAltimeter => {
                write!(f, "ALTITUDE_PROVENANCE_TYPE_RADAR_ALTIMETER")
            }
            Self::AltitudeProvenanceTypeLaserAltimeter => {
                write!(f, "ALTITUDE_PROVENANCE_TYPE_LASER_ALTIMETER")
            }
            Self::AltitudeProvenanceTypeBarometer => {
                write!(f, "ALTITUDE_PROVENANCE_TYPE_BAROMETER")
            }
            Self::AltitudeProvenanceTypeTerrainConversion => {
                write!(f, "ALTITUDE_PROVENANCE_TYPE_TERRAIN_CONVERSION")
            }
            Self::AltitudeProvenanceTypeGnss => write!(f, "ALTITUDE_PROVENANCE_TYPE_GNSS"),
            Self::AltitudeProvenanceTypeSonar => write!(f, "ALTITUDE_PROVENANCE_TYPE_SONAR"),
            Self::AltitudeProvenanceTypeUserInput => {
                write!(f, "ALTITUDE_PROVENANCE_TYPE_USER_INPUT")
            }
            Self::AltitudeProvenanceTypeIns => write!(f, "ALTITUDE_PROVENANCE_TYPE_INS"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
