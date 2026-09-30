pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MilViewDisposition {
    DispositionUnknown,
    DispositionFriendly,
    DispositionHostile,
    DispositionSuspicious,
    DispositionAssumedFriendly,
    DispositionNeutral,
    DispositionPending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MilViewDisposition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DispositionUnknown => serializer.serialize_str("DISPOSITION_UNKNOWN"),
            Self::DispositionFriendly => serializer.serialize_str("DISPOSITION_FRIENDLY"),
            Self::DispositionHostile => serializer.serialize_str("DISPOSITION_HOSTILE"),
            Self::DispositionSuspicious => serializer.serialize_str("DISPOSITION_SUSPICIOUS"),
            Self::DispositionAssumedFriendly => {
                serializer.serialize_str("DISPOSITION_ASSUMED_FRIENDLY")
            }
            Self::DispositionNeutral => serializer.serialize_str("DISPOSITION_NEUTRAL"),
            Self::DispositionPending => serializer.serialize_str("DISPOSITION_PENDING"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MilViewDisposition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DISPOSITION_UNKNOWN" => Ok(Self::DispositionUnknown),
            "DISPOSITION_FRIENDLY" => Ok(Self::DispositionFriendly),
            "DISPOSITION_HOSTILE" => Ok(Self::DispositionHostile),
            "DISPOSITION_SUSPICIOUS" => Ok(Self::DispositionSuspicious),
            "DISPOSITION_ASSUMED_FRIENDLY" => Ok(Self::DispositionAssumedFriendly),
            "DISPOSITION_NEUTRAL" => Ok(Self::DispositionNeutral),
            "DISPOSITION_PENDING" => Ok(Self::DispositionPending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MilViewDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DispositionUnknown => write!(f, "DISPOSITION_UNKNOWN"),
            Self::DispositionFriendly => write!(f, "DISPOSITION_FRIENDLY"),
            Self::DispositionHostile => write!(f, "DISPOSITION_HOSTILE"),
            Self::DispositionSuspicious => write!(f, "DISPOSITION_SUSPICIOUS"),
            Self::DispositionAssumedFriendly => write!(f, "DISPOSITION_ASSUMED_FRIENDLY"),
            Self::DispositionNeutral => write!(f, "DISPOSITION_NEUTRAL"),
            Self::DispositionPending => write!(f, "DISPOSITION_PENDING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
