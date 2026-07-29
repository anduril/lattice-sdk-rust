pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScanCharacteristicsScanType {
    ScanTypeInvalid,
    ScanTypeCircular,
    ScanTypeBidirectionalHorizontalSector,
    ScanTypeBidirectionalVerticalSector,
    ScanTypeNonScanning,
    ScanTypeIrregular,
    ScanTypeConical,
    ScanTypeLobeSwitching,
    ScanTypeRaster,
    ScanTypeCircularVerticalSector,
    ScanTypeCircularConical,
    ScanTypeSectorConical,
    ScanTypeAgileBeam,
    ScanTypeUnidirectionalVerticalSector,
    ScanTypeUnidirectionalHorizontalSector,
    ScanTypeUnidirectionalSector,
    ScanTypeBidirectionalSector,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScanCharacteristicsScanType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ScanTypeInvalid => serializer.serialize_str("SCAN_TYPE_INVALID"),
            Self::ScanTypeCircular => serializer.serialize_str("SCAN_TYPE_CIRCULAR"),
            Self::ScanTypeBidirectionalHorizontalSector => {
                serializer.serialize_str("SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR")
            }
            Self::ScanTypeBidirectionalVerticalSector => {
                serializer.serialize_str("SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR")
            }
            Self::ScanTypeNonScanning => serializer.serialize_str("SCAN_TYPE_NON_SCANNING"),
            Self::ScanTypeIrregular => serializer.serialize_str("SCAN_TYPE_IRREGULAR"),
            Self::ScanTypeConical => serializer.serialize_str("SCAN_TYPE_CONICAL"),
            Self::ScanTypeLobeSwitching => serializer.serialize_str("SCAN_TYPE_LOBE_SWITCHING"),
            Self::ScanTypeRaster => serializer.serialize_str("SCAN_TYPE_RASTER"),
            Self::ScanTypeCircularVerticalSector => {
                serializer.serialize_str("SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR")
            }
            Self::ScanTypeCircularConical => serializer.serialize_str("SCAN_TYPE_CIRCULAR_CONICAL"),
            Self::ScanTypeSectorConical => serializer.serialize_str("SCAN_TYPE_SECTOR_CONICAL"),
            Self::ScanTypeAgileBeam => serializer.serialize_str("SCAN_TYPE_AGILE_BEAM"),
            Self::ScanTypeUnidirectionalVerticalSector => {
                serializer.serialize_str("SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR")
            }
            Self::ScanTypeUnidirectionalHorizontalSector => {
                serializer.serialize_str("SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR")
            }
            Self::ScanTypeUnidirectionalSector => {
                serializer.serialize_str("SCAN_TYPE_UNIDIRECTIONAL_SECTOR")
            }
            Self::ScanTypeBidirectionalSector => {
                serializer.serialize_str("SCAN_TYPE_BIDIRECTIONAL_SECTOR")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScanCharacteristicsScanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SCAN_TYPE_INVALID" => Ok(Self::ScanTypeInvalid),
            "SCAN_TYPE_CIRCULAR" => Ok(Self::ScanTypeCircular),
            "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR" => {
                Ok(Self::ScanTypeBidirectionalHorizontalSector)
            }
            "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR" => {
                Ok(Self::ScanTypeBidirectionalVerticalSector)
            }
            "SCAN_TYPE_NON_SCANNING" => Ok(Self::ScanTypeNonScanning),
            "SCAN_TYPE_IRREGULAR" => Ok(Self::ScanTypeIrregular),
            "SCAN_TYPE_CONICAL" => Ok(Self::ScanTypeConical),
            "SCAN_TYPE_LOBE_SWITCHING" => Ok(Self::ScanTypeLobeSwitching),
            "SCAN_TYPE_RASTER" => Ok(Self::ScanTypeRaster),
            "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR" => Ok(Self::ScanTypeCircularVerticalSector),
            "SCAN_TYPE_CIRCULAR_CONICAL" => Ok(Self::ScanTypeCircularConical),
            "SCAN_TYPE_SECTOR_CONICAL" => Ok(Self::ScanTypeSectorConical),
            "SCAN_TYPE_AGILE_BEAM" => Ok(Self::ScanTypeAgileBeam),
            "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR" => {
                Ok(Self::ScanTypeUnidirectionalVerticalSector)
            }
            "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR" => {
                Ok(Self::ScanTypeUnidirectionalHorizontalSector)
            }
            "SCAN_TYPE_UNIDIRECTIONAL_SECTOR" => Ok(Self::ScanTypeUnidirectionalSector),
            "SCAN_TYPE_BIDIRECTIONAL_SECTOR" => Ok(Self::ScanTypeBidirectionalSector),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScanCharacteristicsScanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScanTypeInvalid => write!(f, "SCAN_TYPE_INVALID"),
            Self::ScanTypeCircular => write!(f, "SCAN_TYPE_CIRCULAR"),
            Self::ScanTypeBidirectionalHorizontalSector => {
                write!(f, "SCAN_TYPE_BIDIRECTIONAL_HORIZONTAL_SECTOR")
            }
            Self::ScanTypeBidirectionalVerticalSector => {
                write!(f, "SCAN_TYPE_BIDIRECTIONAL_VERTICAL_SECTOR")
            }
            Self::ScanTypeNonScanning => write!(f, "SCAN_TYPE_NON_SCANNING"),
            Self::ScanTypeIrregular => write!(f, "SCAN_TYPE_IRREGULAR"),
            Self::ScanTypeConical => write!(f, "SCAN_TYPE_CONICAL"),
            Self::ScanTypeLobeSwitching => write!(f, "SCAN_TYPE_LOBE_SWITCHING"),
            Self::ScanTypeRaster => write!(f, "SCAN_TYPE_RASTER"),
            Self::ScanTypeCircularVerticalSector => write!(f, "SCAN_TYPE_CIRCULAR_VERTICAL_SECTOR"),
            Self::ScanTypeCircularConical => write!(f, "SCAN_TYPE_CIRCULAR_CONICAL"),
            Self::ScanTypeSectorConical => write!(f, "SCAN_TYPE_SECTOR_CONICAL"),
            Self::ScanTypeAgileBeam => write!(f, "SCAN_TYPE_AGILE_BEAM"),
            Self::ScanTypeUnidirectionalVerticalSector => {
                write!(f, "SCAN_TYPE_UNIDIRECTIONAL_VERTICAL_SECTOR")
            }
            Self::ScanTypeUnidirectionalHorizontalSector => {
                write!(f, "SCAN_TYPE_UNIDIRECTIONAL_HORIZONTAL_SECTOR")
            }
            Self::ScanTypeUnidirectionalSector => write!(f, "SCAN_TYPE_UNIDIRECTIONAL_SECTOR"),
            Self::ScanTypeBidirectionalSector => write!(f, "SCAN_TYPE_BIDIRECTIONAL_SECTOR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
