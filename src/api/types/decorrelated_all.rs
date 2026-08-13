pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DecorrelatedAll {
    /// Metadata about the decorrelation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CorrelationMetadata>,
}

impl DecorrelatedAll {
    pub fn builder() -> DecorrelatedAllBuilder {
        <DecorrelatedAllBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DecorrelatedAllBuilder {
    metadata: Option<CorrelationMetadata>,
}

impl DecorrelatedAllBuilder {
    pub fn metadata(mut self, value: CorrelationMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DecorrelatedAll`].
    pub fn build(self) -> Result<DecorrelatedAll, BuildError> {
        Ok(DecorrelatedAll {
            metadata: self.metadata,
        })
    }
}
