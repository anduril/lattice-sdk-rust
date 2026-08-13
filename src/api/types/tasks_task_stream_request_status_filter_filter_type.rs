pub use crate::prelude::*;

/// The type of filter to apply.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskStreamRequestStatusFilterFilterType {
    FilterTypeInvalid,
    FilterTypeInclusive,
    FilterTypeExclusive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskStreamRequestStatusFilterFilterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FilterTypeInvalid => serializer.serialize_str("FILTER_TYPE_INVALID"),
            Self::FilterTypeInclusive => serializer.serialize_str("FILTER_TYPE_INCLUSIVE"),
            Self::FilterTypeExclusive => serializer.serialize_str("FILTER_TYPE_EXCLUSIVE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskStreamRequestStatusFilterFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "FILTER_TYPE_INVALID" => Ok(Self::FilterTypeInvalid),
            "FILTER_TYPE_INCLUSIVE" => Ok(Self::FilterTypeInclusive),
            "FILTER_TYPE_EXCLUSIVE" => Ok(Self::FilterTypeExclusive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskStreamRequestStatusFilterFilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FilterTypeInvalid => write!(f, "FILTER_TYPE_INVALID"),
            Self::FilterTypeInclusive => write!(f, "FILTER_TYPE_INCLUSIVE"),
            Self::FilterTypeExclusive => write!(f, "FILTER_TYPE_EXCLUSIVE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
