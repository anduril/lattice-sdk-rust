pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrelationMembership {
    /// The ID of the correlation set this entity belongs to.
    #[serde(rename = "correlationSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_set_id: Option<String>,
    /// This entity is the primary of a correlation set meaning that it serves as the representative
    /// entity of the correlation set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<PrimaryMembership>,
    /// This entity is not the primary of the correlation set. Note that there may not
    /// be a primary at all.
    #[serde(rename = "nonPrimary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_primary: Option<NonPrimaryMembership>,
    /// Additional metadata on this correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}

impl CorrelationMembership {
    pub fn builder() -> CorrelationMembershipBuilder {
        <CorrelationMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrelationMembershipBuilder {
    correlation_set_id: Option<String>,
    primary: Option<PrimaryMembership>,
    non_primary: Option<NonPrimaryMembership>,
    metadata: Option<CorrelationMetadata>,
}

impl CorrelationMembershipBuilder {
    pub fn correlation_set_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_set_id = Some(value.into());
        self
    }

    pub fn primary(mut self, value: PrimaryMembership) -> Self {
        self.primary = Some(value);
        self
    }

    pub fn non_primary(mut self, value: NonPrimaryMembership) -> Self {
        self.non_primary = Some(value);
        self
    }

    pub fn metadata(mut self, value: CorrelationMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrelationMembership`].
    pub fn build(self) -> Result<CorrelationMembership, BuildError> {
        Ok(CorrelationMembership {
            correlation_set_id: self.correlation_set_id,
            primary: self.primary,
            non_primary: self.non_primary,
            metadata: self.metadata,
        })
    }
}
