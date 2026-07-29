pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrbitMeanElementsMetadataMeanElementTheory {
    MeanElementTheoryInvalid,
    MeanElementTheorySgp4,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrbitMeanElementsMetadataMeanElementTheory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MeanElementTheoryInvalid => {
                serializer.serialize_str("MEAN_ELEMENT_THEORY_INVALID")
            }
            Self::MeanElementTheorySgp4 => serializer.serialize_str("MEAN_ELEMENT_THEORY_SGP4"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrbitMeanElementsMetadataMeanElementTheory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "MEAN_ELEMENT_THEORY_INVALID" => Ok(Self::MeanElementTheoryInvalid),
            "MEAN_ELEMENT_THEORY_SGP4" => Ok(Self::MeanElementTheorySgp4),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrbitMeanElementsMetadataMeanElementTheory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MeanElementTheoryInvalid => write!(f, "MEAN_ELEMENT_THEORY_INVALID"),
            Self::MeanElementTheorySgp4 => write!(f, "MEAN_ELEMENT_THEORY_SGP4"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
