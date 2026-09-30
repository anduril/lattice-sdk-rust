pub use crate::prelude::*;

/// The list_comparator specifies how to compose the boolean results from the child statement
/// for each member of the specified list.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOperationListComparator {
    ListComparatorInvalid,
    ListComparatorAnyOf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOperationListComparator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ListComparatorInvalid => serializer.serialize_str("LIST_COMPARATOR_INVALID"),
            Self::ListComparatorAnyOf => serializer.serialize_str("LIST_COMPARATOR_ANY_OF"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListOperationListComparator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LIST_COMPARATOR_INVALID" => Ok(Self::ListComparatorInvalid),
            "LIST_COMPARATOR_ANY_OF" => Ok(Self::ListComparatorAnyOf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListOperationListComparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListComparatorInvalid => write!(f, "LIST_COMPARATOR_INVALID"),
            Self::ListComparatorAnyOf => write!(f, "LIST_COMPARATOR_ANY_OF"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
