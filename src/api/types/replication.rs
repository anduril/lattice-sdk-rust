pub use crate::prelude::*;

/// Any metadata associated with the replication of a task.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Replication {
    /// The time by which this task should be assumed to be stale.
    #[serde(rename = "staleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub stale_time: Option<DateTime<FixedOffset>>,
}

impl Replication {
    pub fn builder() -> ReplicationBuilder {
        <ReplicationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplicationBuilder {
    stale_time: Option<DateTime<FixedOffset>>,
}

impl ReplicationBuilder {
    pub fn stale_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.stale_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Replication`].
    pub fn build(self) -> Result<Replication, BuildError> {
        Ok(Replication {
            stale_time: self.stale_time,
        })
    }
}
