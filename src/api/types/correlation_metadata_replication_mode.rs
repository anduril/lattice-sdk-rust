pub use crate::prelude::*;

/// Indicates how the correlation will be distributed. Because a correlation is composed of
/// multiple secondaries, each of which may have been correlated with different replication
/// modes, the distribution of the correlation is composed of distributions of the individual
/// entities within the correlation set.
/// For example, if there are two secondary entities A and B correlated against a primary C,
/// with A having been correlated globally and B having been correlated locally, then the
/// correlation set that is distributed globally than what is known locally in the node.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CorrelationMetadataReplicationMode {
    CorrelationReplicationModeInvalid,
    CorrelationReplicationModeLocal,
    CorrelationReplicationModeGlobal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CorrelationMetadataReplicationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CorrelationReplicationModeInvalid => {
                serializer.serialize_str("CORRELATION_REPLICATION_MODE_INVALID")
            }
            Self::CorrelationReplicationModeLocal => {
                serializer.serialize_str("CORRELATION_REPLICATION_MODE_LOCAL")
            }
            Self::CorrelationReplicationModeGlobal => {
                serializer.serialize_str("CORRELATION_REPLICATION_MODE_GLOBAL")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CorrelationMetadataReplicationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CORRELATION_REPLICATION_MODE_INVALID" => Ok(Self::CorrelationReplicationModeInvalid),
            "CORRELATION_REPLICATION_MODE_LOCAL" => Ok(Self::CorrelationReplicationModeLocal),
            "CORRELATION_REPLICATION_MODE_GLOBAL" => Ok(Self::CorrelationReplicationModeGlobal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CorrelationMetadataReplicationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorrelationReplicationModeInvalid => {
                write!(f, "CORRELATION_REPLICATION_MODE_INVALID")
            }
            Self::CorrelationReplicationModeLocal => {
                write!(f, "CORRELATION_REPLICATION_MODE_LOCAL")
            }
            Self::CorrelationReplicationModeGlobal => {
                write!(f, "CORRELATION_REPLICATION_MODE_GLOBAL")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
