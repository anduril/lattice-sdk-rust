pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrelationMetadata {
    /// Who or what added this entity to the (de)correlation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Provenance>,
    /// Indicates how the correlation will be distributed. Because a correlation is composed of
    /// multiple secondaries, each of which may have been correlated with different replication
    /// modes, the distribution of the correlation is composed of distributions of the individual
    /// entities within the correlation set.
    /// For example, if there are two secondary entities A and B correlated against a primary C,
    /// with A having been correlated globally and B having been correlated locally, then the
    /// correlation set that is distributed globally than what is known locally in the node.
    #[serde(rename = "replicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<CorrelationMetadataReplicationMode>,
    /// What type of (de)correlation was this entity added with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CorrelationMetadataType>,
}

impl CorrelationMetadata {
    pub fn builder() -> CorrelationMetadataBuilder {
        <CorrelationMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrelationMetadataBuilder {
    provenance: Option<Provenance>,
    replication_mode: Option<CorrelationMetadataReplicationMode>,
    r#type: Option<CorrelationMetadataType>,
}

impl CorrelationMetadataBuilder {
    pub fn provenance(mut self, value: Provenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn replication_mode(mut self, value: CorrelationMetadataReplicationMode) -> Self {
        self.replication_mode = Some(value);
        self
    }

    pub fn r#type(mut self, value: CorrelationMetadataType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrelationMetadata`].
    pub fn build(self) -> Result<CorrelationMetadata, BuildError> {
        Ok(CorrelationMetadata {
            provenance: self.provenance,
            replication_mode: self.replication_mode,
            r#type: self.r#type,
        })
    }
}
