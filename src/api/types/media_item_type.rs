pub use crate::prelude::*;

/// The type of media for this item.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaItemType {
    MediaTypeInvalid,
    MediaTypeImage,
    MediaTypeVideo,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MediaItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MediaTypeInvalid => serializer.serialize_str("MEDIA_TYPE_INVALID"),
            Self::MediaTypeImage => serializer.serialize_str("MEDIA_TYPE_IMAGE"),
            Self::MediaTypeVideo => serializer.serialize_str("MEDIA_TYPE_VIDEO"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MediaItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MEDIA_TYPE_INVALID" => Ok(Self::MediaTypeInvalid),
            "MEDIA_TYPE_IMAGE" => Ok(Self::MediaTypeImage),
            "MEDIA_TYPE_VIDEO" => Ok(Self::MediaTypeVideo),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MediaItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MediaTypeInvalid => write!(f, "MEDIA_TYPE_INVALID"),
            Self::MediaTypeImage => write!(f, "MEDIA_TYPE_IMAGE"),
            Self::MediaTypeVideo => write!(f, "MEDIA_TYPE_VIDEO"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
