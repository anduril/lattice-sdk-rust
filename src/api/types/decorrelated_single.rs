pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DecorrelatedSingle {
    /// The entity that was decorrelated against.
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Metadata about the decorrelation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}

impl DecorrelatedSingle {
    pub fn builder() -> DecorrelatedSingleBuilder {
        <DecorrelatedSingleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecorrelatedSingleBuilder {
    entity_id: Option<String>,
    metadata: Option<CorrelationMetadata>,
}

impl DecorrelatedSingleBuilder {
    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: CorrelationMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DecorrelatedSingle`].
    pub fn build(self) -> Result<DecorrelatedSingle, BuildError> {
        Ok(DecorrelatedSingle {
            entity_id: self.entity_id,
            metadata: self.metadata,
        })
    }
}
