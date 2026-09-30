pub use crate::prelude::*;

/// Current lifecycle status of the stream. See StreamStatus for the full state machine.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngressStreamStatus {
    StreamStatusUnspecified,
    StreamStatusLive,
    StreamStatusInactive,
    StreamStatusUnavailable,
    StreamStatusArchived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IngressStreamStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StreamStatusUnspecified => serializer.serialize_str("STREAM_STATUS_UNSPECIFIED"),
            Self::StreamStatusLive => serializer.serialize_str("STREAM_STATUS_LIVE"),
            Self::StreamStatusInactive => serializer.serialize_str("STREAM_STATUS_INACTIVE"),
            Self::StreamStatusUnavailable => serializer.serialize_str("STREAM_STATUS_UNAVAILABLE"),
            Self::StreamStatusArchived => serializer.serialize_str("STREAM_STATUS_ARCHIVED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IngressStreamStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "STREAM_STATUS_UNSPECIFIED" => Ok(Self::StreamStatusUnspecified),
            "STREAM_STATUS_LIVE" => Ok(Self::StreamStatusLive),
            "STREAM_STATUS_INACTIVE" => Ok(Self::StreamStatusInactive),
            "STREAM_STATUS_UNAVAILABLE" => Ok(Self::StreamStatusUnavailable),
            "STREAM_STATUS_ARCHIVED" => Ok(Self::StreamStatusArchived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IngressStreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StreamStatusUnspecified => write!(f, "STREAM_STATUS_UNSPECIFIED"),
            Self::StreamStatusLive => write!(f, "STREAM_STATUS_LIVE"),
            Self::StreamStatusInactive => write!(f, "STREAM_STATUS_INACTIVE"),
            Self::StreamStatusUnavailable => write!(f, "STREAM_STATUS_UNAVAILABLE"),
            Self::StreamStatusArchived => write!(f, "STREAM_STATUS_ARCHIVED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
