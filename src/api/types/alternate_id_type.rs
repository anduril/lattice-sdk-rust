pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlternateIdType {
    AltIdTypeInvalid,
    AltIdTypeTrackId2,
    AltIdTypeTrackId1,
    AltIdTypeSpiId,
    AltIdTypeNitfFileTitle,
    AltIdTypeTrackRepoAlertId,
    AltIdTypeAssetId,
    AltIdTypeLink16TrackNumber,
    AltIdTypeLink16Ju,
    AltIdTypeNcctMessageId,
    AltIdTypeCallsign,
    AltIdTypeMmsiId,
    AltIdTypeVmfUrn,
    AltIdTypeImoId,
    AltIdTypeVmfTargetNumber,
    AltIdTypeSerialNumber,
    AltIdTypeRegistrationId,
    AltIdTypeIbsGid,
    AltIdTypeDodaac,
    AltIdTypeUic,
    AltIdTypeNoradCatId,
    AltIdTypeUnoosaName,
    AltIdTypeUnoosaId,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AlternateIdType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AltIdTypeInvalid => serializer.serialize_str("ALT_ID_TYPE_INVALID"),
            Self::AltIdTypeTrackId2 => serializer.serialize_str("ALT_ID_TYPE_TRACK_ID_2"),
            Self::AltIdTypeTrackId1 => serializer.serialize_str("ALT_ID_TYPE_TRACK_ID_1"),
            Self::AltIdTypeSpiId => serializer.serialize_str("ALT_ID_TYPE_SPI_ID"),
            Self::AltIdTypeNitfFileTitle => serializer.serialize_str("ALT_ID_TYPE_NITF_FILE_TITLE"),
            Self::AltIdTypeTrackRepoAlertId => {
                serializer.serialize_str("ALT_ID_TYPE_TRACK_REPO_ALERT_ID")
            }
            Self::AltIdTypeAssetId => serializer.serialize_str("ALT_ID_TYPE_ASSET_ID"),
            Self::AltIdTypeLink16TrackNumber => {
                serializer.serialize_str("ALT_ID_TYPE_LINK16_TRACK_NUMBER")
            }
            Self::AltIdTypeLink16Ju => serializer.serialize_str("ALT_ID_TYPE_LINK16_JU"),
            Self::AltIdTypeNcctMessageId => serializer.serialize_str("ALT_ID_TYPE_NCCT_MESSAGE_ID"),
            Self::AltIdTypeCallsign => serializer.serialize_str("ALT_ID_TYPE_CALLSIGN"),
            Self::AltIdTypeMmsiId => serializer.serialize_str("ALT_ID_TYPE_MMSI_ID"),
            Self::AltIdTypeVmfUrn => serializer.serialize_str("ALT_ID_TYPE_VMF_URN"),
            Self::AltIdTypeImoId => serializer.serialize_str("ALT_ID_TYPE_IMO_ID"),
            Self::AltIdTypeVmfTargetNumber => {
                serializer.serialize_str("ALT_ID_TYPE_VMF_TARGET_NUMBER")
            }
            Self::AltIdTypeSerialNumber => serializer.serialize_str("ALT_ID_TYPE_SERIAL_NUMBER"),
            Self::AltIdTypeRegistrationId => {
                serializer.serialize_str("ALT_ID_TYPE_REGISTRATION_ID")
            }
            Self::AltIdTypeIbsGid => serializer.serialize_str("ALT_ID_TYPE_IBS_GID"),
            Self::AltIdTypeDodaac => serializer.serialize_str("ALT_ID_TYPE_DODAAC"),
            Self::AltIdTypeUic => serializer.serialize_str("ALT_ID_TYPE_UIC"),
            Self::AltIdTypeNoradCatId => serializer.serialize_str("ALT_ID_TYPE_NORAD_CAT_ID"),
            Self::AltIdTypeUnoosaName => serializer.serialize_str("ALT_ID_TYPE_UNOOSA_NAME"),
            Self::AltIdTypeUnoosaId => serializer.serialize_str("ALT_ID_TYPE_UNOOSA_ID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AlternateIdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALT_ID_TYPE_INVALID" => Ok(Self::AltIdTypeInvalid),
            "ALT_ID_TYPE_TRACK_ID_2" => Ok(Self::AltIdTypeTrackId2),
            "ALT_ID_TYPE_TRACK_ID_1" => Ok(Self::AltIdTypeTrackId1),
            "ALT_ID_TYPE_SPI_ID" => Ok(Self::AltIdTypeSpiId),
            "ALT_ID_TYPE_NITF_FILE_TITLE" => Ok(Self::AltIdTypeNitfFileTitle),
            "ALT_ID_TYPE_TRACK_REPO_ALERT_ID" => Ok(Self::AltIdTypeTrackRepoAlertId),
            "ALT_ID_TYPE_ASSET_ID" => Ok(Self::AltIdTypeAssetId),
            "ALT_ID_TYPE_LINK16_TRACK_NUMBER" => Ok(Self::AltIdTypeLink16TrackNumber),
            "ALT_ID_TYPE_LINK16_JU" => Ok(Self::AltIdTypeLink16Ju),
            "ALT_ID_TYPE_NCCT_MESSAGE_ID" => Ok(Self::AltIdTypeNcctMessageId),
            "ALT_ID_TYPE_CALLSIGN" => Ok(Self::AltIdTypeCallsign),
            "ALT_ID_TYPE_MMSI_ID" => Ok(Self::AltIdTypeMmsiId),
            "ALT_ID_TYPE_VMF_URN" => Ok(Self::AltIdTypeVmfUrn),
            "ALT_ID_TYPE_IMO_ID" => Ok(Self::AltIdTypeImoId),
            "ALT_ID_TYPE_VMF_TARGET_NUMBER" => Ok(Self::AltIdTypeVmfTargetNumber),
            "ALT_ID_TYPE_SERIAL_NUMBER" => Ok(Self::AltIdTypeSerialNumber),
            "ALT_ID_TYPE_REGISTRATION_ID" => Ok(Self::AltIdTypeRegistrationId),
            "ALT_ID_TYPE_IBS_GID" => Ok(Self::AltIdTypeIbsGid),
            "ALT_ID_TYPE_DODAAC" => Ok(Self::AltIdTypeDodaac),
            "ALT_ID_TYPE_UIC" => Ok(Self::AltIdTypeUic),
            "ALT_ID_TYPE_NORAD_CAT_ID" => Ok(Self::AltIdTypeNoradCatId),
            "ALT_ID_TYPE_UNOOSA_NAME" => Ok(Self::AltIdTypeUnoosaName),
            "ALT_ID_TYPE_UNOOSA_ID" => Ok(Self::AltIdTypeUnoosaId),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AlternateIdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AltIdTypeInvalid => write!(f, "ALT_ID_TYPE_INVALID"),
            Self::AltIdTypeTrackId2 => write!(f, "ALT_ID_TYPE_TRACK_ID_2"),
            Self::AltIdTypeTrackId1 => write!(f, "ALT_ID_TYPE_TRACK_ID_1"),
            Self::AltIdTypeSpiId => write!(f, "ALT_ID_TYPE_SPI_ID"),
            Self::AltIdTypeNitfFileTitle => write!(f, "ALT_ID_TYPE_NITF_FILE_TITLE"),
            Self::AltIdTypeTrackRepoAlertId => write!(f, "ALT_ID_TYPE_TRACK_REPO_ALERT_ID"),
            Self::AltIdTypeAssetId => write!(f, "ALT_ID_TYPE_ASSET_ID"),
            Self::AltIdTypeLink16TrackNumber => write!(f, "ALT_ID_TYPE_LINK16_TRACK_NUMBER"),
            Self::AltIdTypeLink16Ju => write!(f, "ALT_ID_TYPE_LINK16_JU"),
            Self::AltIdTypeNcctMessageId => write!(f, "ALT_ID_TYPE_NCCT_MESSAGE_ID"),
            Self::AltIdTypeCallsign => write!(f, "ALT_ID_TYPE_CALLSIGN"),
            Self::AltIdTypeMmsiId => write!(f, "ALT_ID_TYPE_MMSI_ID"),
            Self::AltIdTypeVmfUrn => write!(f, "ALT_ID_TYPE_VMF_URN"),
            Self::AltIdTypeImoId => write!(f, "ALT_ID_TYPE_IMO_ID"),
            Self::AltIdTypeVmfTargetNumber => write!(f, "ALT_ID_TYPE_VMF_TARGET_NUMBER"),
            Self::AltIdTypeSerialNumber => write!(f, "ALT_ID_TYPE_SERIAL_NUMBER"),
            Self::AltIdTypeRegistrationId => write!(f, "ALT_ID_TYPE_REGISTRATION_ID"),
            Self::AltIdTypeIbsGid => write!(f, "ALT_ID_TYPE_IBS_GID"),
            Self::AltIdTypeDodaac => write!(f, "ALT_ID_TYPE_DODAAC"),
            Self::AltIdTypeUic => write!(f, "ALT_ID_TYPE_UIC"),
            Self::AltIdTypeNoradCatId => write!(f, "ALT_ID_TYPE_NORAD_CAT_ID"),
            Self::AltIdTypeUnoosaName => write!(f, "ALT_ID_TYPE_UNOOSA_NAME"),
            Self::AltIdTypeUnoosaId => write!(f, "ALT_ID_TYPE_UNOOSA_ID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
