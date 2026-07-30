pub use crate::prelude::*;

/// The TimestampType represents a static timestamp value.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimestampType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub value: Option<DateTime<FixedOffset>>,
}

impl TimestampType {
    pub fn builder() -> TimestampTypeBuilder {
        <TimestampTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimestampTypeBuilder {
    value: Option<DateTime<FixedOffset>>,
}

impl TimestampTypeBuilder {
    pub fn value(mut self, value: DateTime<FixedOffset>) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimestampType`].
    pub fn build(self) -> Result<TimestampType, BuildError> {
        Ok(TimestampType { value: self.value })
    }
}
