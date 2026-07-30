pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AltitudeProvenance {
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<AltitudeProvenanceSourceType>,
}

impl AltitudeProvenance {
    pub fn builder() -> AltitudeProvenanceBuilder {
        <AltitudeProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeProvenanceBuilder {
    source_type: Option<AltitudeProvenanceSourceType>,
}

impl AltitudeProvenanceBuilder {
    pub fn source_type(mut self, value: AltitudeProvenanceSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeProvenance`].
    pub fn build(self) -> Result<AltitudeProvenance, BuildError> {
        Ok(AltitudeProvenance {
            source_type: self.source_type,
        })
    }
}
