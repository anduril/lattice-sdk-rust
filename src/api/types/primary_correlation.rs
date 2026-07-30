pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrimaryCorrelation {
    /// The secondary entity IDs part of this correlation.
    #[serde(rename = "secondaryEntityIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_entity_ids: Option<Vec<String>>,
}

impl PrimaryCorrelation {
    pub fn builder() -> PrimaryCorrelationBuilder {
        <PrimaryCorrelationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrimaryCorrelationBuilder {
    secondary_entity_ids: Option<Vec<String>>,
}

impl PrimaryCorrelationBuilder {
    pub fn secondary_entity_ids(mut self, value: Vec<String>) -> Self {
        self.secondary_entity_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrimaryCorrelation`].
    pub fn build(self) -> Result<PrimaryCorrelation, BuildError> {
        Ok(PrimaryCorrelation {
            secondary_entity_ids: self.secondary_entity_ids,
        })
    }
}
