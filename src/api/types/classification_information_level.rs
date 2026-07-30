pub use crate::prelude::*;

/// Classification level to be applied to the information in question.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassificationInformationLevel {
    ClassificationLevelsInvalid,
    ClassificationLevelsUnclassified,
    ClassificationLevelsControlledUnclassified,
    ClassificationLevelsConfidential,
    ClassificationLevelsSecret,
    ClassificationLevelsTopSecret,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClassificationInformationLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClassificationLevelsInvalid => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_INVALID")
            }
            Self::ClassificationLevelsUnclassified => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_UNCLASSIFIED")
            }
            Self::ClassificationLevelsControlledUnclassified => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED")
            }
            Self::ClassificationLevelsConfidential => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_CONFIDENTIAL")
            }
            Self::ClassificationLevelsSecret => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_SECRET")
            }
            Self::ClassificationLevelsTopSecret => {
                serializer.serialize_str("CLASSIFICATION_LEVELS_TOP_SECRET")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClassificationInformationLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CLASSIFICATION_LEVELS_INVALID" => Ok(Self::ClassificationLevelsInvalid),
            "CLASSIFICATION_LEVELS_UNCLASSIFIED" => Ok(Self::ClassificationLevelsUnclassified),
            "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED" => {
                Ok(Self::ClassificationLevelsControlledUnclassified)
            }
            "CLASSIFICATION_LEVELS_CONFIDENTIAL" => Ok(Self::ClassificationLevelsConfidential),
            "CLASSIFICATION_LEVELS_SECRET" => Ok(Self::ClassificationLevelsSecret),
            "CLASSIFICATION_LEVELS_TOP_SECRET" => Ok(Self::ClassificationLevelsTopSecret),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClassificationInformationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClassificationLevelsInvalid => write!(f, "CLASSIFICATION_LEVELS_INVALID"),
            Self::ClassificationLevelsUnclassified => {
                write!(f, "CLASSIFICATION_LEVELS_UNCLASSIFIED")
            }
            Self::ClassificationLevelsControlledUnclassified => {
                write!(f, "CLASSIFICATION_LEVELS_CONTROLLED_UNCLASSIFIED")
            }
            Self::ClassificationLevelsConfidential => {
                write!(f, "CLASSIFICATION_LEVELS_CONFIDENTIAL")
            }
            Self::ClassificationLevelsSecret => write!(f, "CLASSIFICATION_LEVELS_SECRET"),
            Self::ClassificationLevelsTopSecret => write!(f, "CLASSIFICATION_LEVELS_TOP_SECRET"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
