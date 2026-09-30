pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityEventEventType {
    EventTypeInvalid,
    EventTypeCreated,
    EventTypeUpdate,
    EventTypeDeleted,
    EventTypePreexisting,
    EventTypePostExpiryOverride,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EntityEventEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EventTypeInvalid => serializer.serialize_str("EVENT_TYPE_INVALID"),
            Self::EventTypeCreated => serializer.serialize_str("EVENT_TYPE_CREATED"),
            Self::EventTypeUpdate => serializer.serialize_str("EVENT_TYPE_UPDATE"),
            Self::EventTypeDeleted => serializer.serialize_str("EVENT_TYPE_DELETED"),
            Self::EventTypePreexisting => serializer.serialize_str("EVENT_TYPE_PREEXISTING"),
            Self::EventTypePostExpiryOverride => {
                serializer.serialize_str("EVENT_TYPE_POST_EXPIRY_OVERRIDE")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EntityEventEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "EVENT_TYPE_INVALID" => Ok(Self::EventTypeInvalid),
            "EVENT_TYPE_CREATED" => Ok(Self::EventTypeCreated),
            "EVENT_TYPE_UPDATE" => Ok(Self::EventTypeUpdate),
            "EVENT_TYPE_DELETED" => Ok(Self::EventTypeDeleted),
            "EVENT_TYPE_PREEXISTING" => Ok(Self::EventTypePreexisting),
            "EVENT_TYPE_POST_EXPIRY_OVERRIDE" => Ok(Self::EventTypePostExpiryOverride),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EntityEventEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventTypeInvalid => write!(f, "EVENT_TYPE_INVALID"),
            Self::EventTypeCreated => write!(f, "EVENT_TYPE_CREATED"),
            Self::EventTypeUpdate => write!(f, "EVENT_TYPE_UPDATE"),
            Self::EventTypeDeleted => write!(f, "EVENT_TYPE_DELETED"),
            Self::EventTypePreexisting => write!(f, "EVENT_TYPE_PREEXISTING"),
            Self::EventTypePostExpiryOverride => write!(f, "EVENT_TYPE_POST_EXPIRY_OVERRIDE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
