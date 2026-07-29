pub use crate::prelude::*;

/// The current status of the delivery.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeliveryStateStatus {
    DeliveryStatusInvalid,
    DeliveryStatusDelivered,
    DeliveryStatusPendingExecute,
    DeliveryStatusPendingCancel,
    DeliveryStatusPendingComplete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeliveryStateStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DeliveryStatusInvalid => serializer.serialize_str("DELIVERY_STATUS_INVALID"),
            Self::DeliveryStatusDelivered => serializer.serialize_str("DELIVERY_STATUS_DELIVERED"),
            Self::DeliveryStatusPendingExecute => {
                serializer.serialize_str("DELIVERY_STATUS_PENDING_EXECUTE")
            }
            Self::DeliveryStatusPendingCancel => {
                serializer.serialize_str("DELIVERY_STATUS_PENDING_CANCEL")
            }
            Self::DeliveryStatusPendingComplete => {
                serializer.serialize_str("DELIVERY_STATUS_PENDING_COMPLETE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeliveryStateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DELIVERY_STATUS_INVALID" => Ok(Self::DeliveryStatusInvalid),
            "DELIVERY_STATUS_DELIVERED" => Ok(Self::DeliveryStatusDelivered),
            "DELIVERY_STATUS_PENDING_EXECUTE" => Ok(Self::DeliveryStatusPendingExecute),
            "DELIVERY_STATUS_PENDING_CANCEL" => Ok(Self::DeliveryStatusPendingCancel),
            "DELIVERY_STATUS_PENDING_COMPLETE" => Ok(Self::DeliveryStatusPendingComplete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeliveryStateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeliveryStatusInvalid => write!(f, "DELIVERY_STATUS_INVALID"),
            Self::DeliveryStatusDelivered => write!(f, "DELIVERY_STATUS_DELIVERED"),
            Self::DeliveryStatusPendingExecute => write!(f, "DELIVERY_STATUS_PENDING_EXECUTE"),
            Self::DeliveryStatusPendingCancel => write!(f, "DELIVERY_STATUS_PENDING_CANCEL"),
            Self::DeliveryStatusPendingComplete => write!(f, "DELIVERY_STATUS_PENDING_COMPLETE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
