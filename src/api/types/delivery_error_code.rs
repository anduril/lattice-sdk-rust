pub use crate::prelude::*;

/// Error code for Delivery error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeliveryErrorCode {
    DeliveryErrorCodeInvalid,
    DeliveryErrorCodeUnavailable,
    DeliveryErrorCodeTimeout,
    DeliveryErrorCodeRejected,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeliveryErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DeliveryErrorCodeInvalid => {
                serializer.serialize_str("DELIVERY_ERROR_CODE_INVALID")
            }
            Self::DeliveryErrorCodeUnavailable => {
                serializer.serialize_str("DELIVERY_ERROR_CODE_UNAVAILABLE")
            }
            Self::DeliveryErrorCodeTimeout => {
                serializer.serialize_str("DELIVERY_ERROR_CODE_TIMEOUT")
            }
            Self::DeliveryErrorCodeRejected => {
                serializer.serialize_str("DELIVERY_ERROR_CODE_REJECTED")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeliveryErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DELIVERY_ERROR_CODE_INVALID" => Ok(Self::DeliveryErrorCodeInvalid),
            "DELIVERY_ERROR_CODE_UNAVAILABLE" => Ok(Self::DeliveryErrorCodeUnavailable),
            "DELIVERY_ERROR_CODE_TIMEOUT" => Ok(Self::DeliveryErrorCodeTimeout),
            "DELIVERY_ERROR_CODE_REJECTED" => Ok(Self::DeliveryErrorCodeRejected),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeliveryErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeliveryErrorCodeInvalid => write!(f, "DELIVERY_ERROR_CODE_INVALID"),
            Self::DeliveryErrorCodeUnavailable => write!(f, "DELIVERY_ERROR_CODE_UNAVAILABLE"),
            Self::DeliveryErrorCodeTimeout => write!(f, "DELIVERY_ERROR_CODE_TIMEOUT"),
            Self::DeliveryErrorCodeRejected => write!(f, "DELIVERY_ERROR_CODE_REJECTED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
