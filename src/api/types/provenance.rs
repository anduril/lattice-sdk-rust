pub use crate::prelude::*;

/// Data provenance.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Provenance {
    /// Name of the integration that produced this entity
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    /// Source data type of this entity. Examples: ADSB, Link16, etc.
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// An ID that allows an element from a source to be uniquely identified
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The time, according to the source system, that the data in the entity was last modified. Generally, this should
    /// be the time that the source-reported time of validity of the data in the entity. This field must be
    /// updated with every change to the entity or else Entity Manager will discard the update.
    #[serde(rename = "sourceUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub source_update_time: Option<DateTime<FixedOffset>>,
    /// Description of the modification source. In the case of a user this is the email address.
    #[serde(rename = "sourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
}

impl Provenance {
    pub fn builder() -> ProvenanceBuilder {
        <ProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProvenanceBuilder {
    integration_name: Option<String>,
    data_type: Option<String>,
    source_id: Option<String>,
    source_update_time: Option<DateTime<FixedOffset>>,
    source_description: Option<String>,
}

impl ProvenanceBuilder {
    pub fn integration_name(mut self, value: impl Into<String>) -> Self {
        self.integration_name = Some(value.into());
        self
    }

    pub fn data_type(mut self, value: impl Into<String>) -> Self {
        self.data_type = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn source_update_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.source_update_time = Some(value);
        self
    }

    pub fn source_description(mut self, value: impl Into<String>) -> Self {
        self.source_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Provenance`].
    pub fn build(self) -> Result<Provenance, BuildError> {
        Ok(Provenance {
            integration_name: self.integration_name,
            data_type: self.data_type,
            source_id: self.source_id,
            source_update_time: self.source_update_time,
            source_description: self.source_description,
        })
    }
}
