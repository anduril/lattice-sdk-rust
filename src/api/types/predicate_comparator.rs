pub use crate::prelude::*;

/// The comparator determines the manner in which the entity field and static value are compared.
/// Comparators may only be applied to certain values. For example, the WITHIN comparator cannot
/// be used for a boolean value comparison.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateComparator {
    ComparatorInvalid,
    ComparatorMatchAll,
    ComparatorEquality,
    ComparatorIn,
    ComparatorLessThan,
    ComparatorGreaterThan,
    ComparatorLessThanEqualTo,
    ComparatorGreaterThanEqualTo,
    ComparatorWithin,
    ComparatorExists,
    ComparatorCaseInsensitiveEquality,
    ComparatorCaseInsensitiveEqualityIn,
    ComparatorRangeClosed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PredicateComparator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ComparatorInvalid => serializer.serialize_str("COMPARATOR_INVALID"),
            Self::ComparatorMatchAll => serializer.serialize_str("COMPARATOR_MATCH_ALL"),
            Self::ComparatorEquality => serializer.serialize_str("COMPARATOR_EQUALITY"),
            Self::ComparatorIn => serializer.serialize_str("COMPARATOR_IN"),
            Self::ComparatorLessThan => serializer.serialize_str("COMPARATOR_LESS_THAN"),
            Self::ComparatorGreaterThan => serializer.serialize_str("COMPARATOR_GREATER_THAN"),
            Self::ComparatorLessThanEqualTo => {
                serializer.serialize_str("COMPARATOR_LESS_THAN_EQUAL_TO")
            }
            Self::ComparatorGreaterThanEqualTo => {
                serializer.serialize_str("COMPARATOR_GREATER_THAN_EQUAL_TO")
            }
            Self::ComparatorWithin => serializer.serialize_str("COMPARATOR_WITHIN"),
            Self::ComparatorExists => serializer.serialize_str("COMPARATOR_EXISTS"),
            Self::ComparatorCaseInsensitiveEquality => {
                serializer.serialize_str("COMPARATOR_CASE_INSENSITIVE_EQUALITY")
            }
            Self::ComparatorCaseInsensitiveEqualityIn => {
                serializer.serialize_str("COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN")
            }
            Self::ComparatorRangeClosed => serializer.serialize_str("COMPARATOR_RANGE_CLOSED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PredicateComparator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "COMPARATOR_INVALID" => Ok(Self::ComparatorInvalid),
            "COMPARATOR_MATCH_ALL" => Ok(Self::ComparatorMatchAll),
            "COMPARATOR_EQUALITY" => Ok(Self::ComparatorEquality),
            "COMPARATOR_IN" => Ok(Self::ComparatorIn),
            "COMPARATOR_LESS_THAN" => Ok(Self::ComparatorLessThan),
            "COMPARATOR_GREATER_THAN" => Ok(Self::ComparatorGreaterThan),
            "COMPARATOR_LESS_THAN_EQUAL_TO" => Ok(Self::ComparatorLessThanEqualTo),
            "COMPARATOR_GREATER_THAN_EQUAL_TO" => Ok(Self::ComparatorGreaterThanEqualTo),
            "COMPARATOR_WITHIN" => Ok(Self::ComparatorWithin),
            "COMPARATOR_EXISTS" => Ok(Self::ComparatorExists),
            "COMPARATOR_CASE_INSENSITIVE_EQUALITY" => Ok(Self::ComparatorCaseInsensitiveEquality),
            "COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN" => {
                Ok(Self::ComparatorCaseInsensitiveEqualityIn)
            }
            "COMPARATOR_RANGE_CLOSED" => Ok(Self::ComparatorRangeClosed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PredicateComparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComparatorInvalid => write!(f, "COMPARATOR_INVALID"),
            Self::ComparatorMatchAll => write!(f, "COMPARATOR_MATCH_ALL"),
            Self::ComparatorEquality => write!(f, "COMPARATOR_EQUALITY"),
            Self::ComparatorIn => write!(f, "COMPARATOR_IN"),
            Self::ComparatorLessThan => write!(f, "COMPARATOR_LESS_THAN"),
            Self::ComparatorGreaterThan => write!(f, "COMPARATOR_GREATER_THAN"),
            Self::ComparatorLessThanEqualTo => write!(f, "COMPARATOR_LESS_THAN_EQUAL_TO"),
            Self::ComparatorGreaterThanEqualTo => write!(f, "COMPARATOR_GREATER_THAN_EQUAL_TO"),
            Self::ComparatorWithin => write!(f, "COMPARATOR_WITHIN"),
            Self::ComparatorExists => write!(f, "COMPARATOR_EXISTS"),
            Self::ComparatorCaseInsensitiveEquality => {
                write!(f, "COMPARATOR_CASE_INSENSITIVE_EQUALITY")
            }
            Self::ComparatorCaseInsensitiveEqualityIn => {
                write!(f, "COMPARATOR_CASE_INSENSITIVE_EQUALITY_IN")
            }
            Self::ComparatorRangeClosed => write!(f, "COMPARATOR_RANGE_CLOSED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
