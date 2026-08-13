pub use crate::prelude::*;

/// Alert level (Warning, Caution, or Advisory).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertLevel {
    AlertLevelInvalid,
    AlertLevelAdvisory,
    AlertLevelCaution,
    AlertLevelWarning,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AlertLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AlertLevelInvalid => serializer.serialize_str("ALERT_LEVEL_INVALID"),
            Self::AlertLevelAdvisory => serializer.serialize_str("ALERT_LEVEL_ADVISORY"),
            Self::AlertLevelCaution => serializer.serialize_str("ALERT_LEVEL_CAUTION"),
            Self::AlertLevelWarning => serializer.serialize_str("ALERT_LEVEL_WARNING"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AlertLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALERT_LEVEL_INVALID" => Ok(Self::AlertLevelInvalid),
            "ALERT_LEVEL_ADVISORY" => Ok(Self::AlertLevelAdvisory),
            "ALERT_LEVEL_CAUTION" => Ok(Self::AlertLevelCaution),
            "ALERT_LEVEL_WARNING" => Ok(Self::AlertLevelWarning),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlertLevelInvalid => write!(f, "ALERT_LEVEL_INVALID"),
            Self::AlertLevelAdvisory => write!(f, "ALERT_LEVEL_ADVISORY"),
            Self::AlertLevelCaution => write!(f, "ALERT_LEVEL_CAUTION"),
            Self::AlertLevelWarning => write!(f, "ALERT_LEVEL_WARNING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
