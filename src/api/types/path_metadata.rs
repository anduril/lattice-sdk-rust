pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PathMetadata {
    #[serde(default)]
    pub content_identifier: ContentIdentifier,
    #[serde(default)]
    pub size_bytes: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiry_time: Option<DateTime<FixedOffset>>,
}

impl PathMetadata {
    pub fn builder() -> PathMetadataBuilder {
        <PathMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PathMetadataBuilder {
    content_identifier: Option<ContentIdentifier>,
    size_bytes: Option<i64>,
    last_updated_at: Option<DateTime<FixedOffset>>,
    expiry_time: Option<DateTime<FixedOffset>>,
}

impl PathMetadataBuilder {
    pub fn content_identifier(mut self, value: ContentIdentifier) -> Self {
        self.content_identifier = Some(value);
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn last_updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_updated_at = Some(value);
        self
    }

    pub fn expiry_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiry_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PathMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_identifier`](PathMetadataBuilder::content_identifier)
    /// - [`size_bytes`](PathMetadataBuilder::size_bytes)
    /// - [`last_updated_at`](PathMetadataBuilder::last_updated_at)
    pub fn build(self) -> Result<PathMetadata, BuildError> {
        Ok(PathMetadata {
            content_identifier: self
                .content_identifier
                .ok_or_else(|| BuildError::missing_field("content_identifier"))?,
            size_bytes: self
                .size_bytes
                .ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            last_updated_at: self
                .last_updated_at
                .ok_or_else(|| BuildError::missing_field("last_updated_at"))?,
            expiry_time: self.expiry_time,
        })
    }
}
