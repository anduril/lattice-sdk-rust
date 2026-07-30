pub use crate::prelude::*;

/// The validity of the response from the Mode 4 interrogation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransponderCodesMode4InterrogationResponse {
    InterrogationResponseInvalid,
    InterrogationResponseCorrect,
    InterrogationResponseIncorrect,
    InterrogationResponseNoResponse,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TransponderCodesMode4InterrogationResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InterrogationResponseInvalid => {
                serializer.serialize_str("INTERROGATION_RESPONSE_INVALID")
            }
            Self::InterrogationResponseCorrect => {
                serializer.serialize_str("INTERROGATION_RESPONSE_CORRECT")
            }
            Self::InterrogationResponseIncorrect => {
                serializer.serialize_str("INTERROGATION_RESPONSE_INCORRECT")
            }
            Self::InterrogationResponseNoResponse => {
                serializer.serialize_str("INTERROGATION_RESPONSE_NO_RESPONSE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TransponderCodesMode4InterrogationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "INTERROGATION_RESPONSE_INVALID" => Ok(Self::InterrogationResponseInvalid),
            "INTERROGATION_RESPONSE_CORRECT" => Ok(Self::InterrogationResponseCorrect),
            "INTERROGATION_RESPONSE_INCORRECT" => Ok(Self::InterrogationResponseIncorrect),
            "INTERROGATION_RESPONSE_NO_RESPONSE" => Ok(Self::InterrogationResponseNoResponse),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TransponderCodesMode4InterrogationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterrogationResponseInvalid => write!(f, "INTERROGATION_RESPONSE_INVALID"),
            Self::InterrogationResponseCorrect => write!(f, "INTERROGATION_RESPONSE_CORRECT"),
            Self::InterrogationResponseIncorrect => write!(f, "INTERROGATION_RESPONSE_INCORRECT"),
            Self::InterrogationResponseNoResponse => {
                write!(f, "INTERROGATION_RESPONSE_NO_RESPONSE")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
