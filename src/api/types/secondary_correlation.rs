pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SecondaryCorrelation {
    /// The primary of this correlation.
    #[serde(rename = "primaryEntityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_entity_id: Option<String>,
    /// Metadata about the correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}

impl SecondaryCorrelation {
    pub fn builder() -> SecondaryCorrelationBuilder {
        <SecondaryCorrelationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecondaryCorrelationBuilder {
    primary_entity_id: Option<String>,
    metadata: Option<CorrelationMetadata>,
}

impl SecondaryCorrelationBuilder {
    pub fn primary_entity_id(mut self, value: impl Into<String>) -> Self {
        self.primary_entity_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: CorrelationMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecondaryCorrelation`].
    pub fn build(self) -> Result<SecondaryCorrelation, BuildError> {
        Ok(SecondaryCorrelation {
            primary_entity_id: self.primary_entity_id,
            metadata: self.metadata,
        })
    }
}
