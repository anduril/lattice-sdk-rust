pub use crate::prelude::*;

/// Orbit Mean Elements data, analogous to the Orbit Mean Elements Message in CCSDS 502.0-B-3
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrbitMeanElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OrbitMeanElementsMetadata>,
    #[serde(rename = "meanKeplerianElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_keplerian_elements: Option<MeanKeplerianElements>,
    #[serde(rename = "tleParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tle_parameters: Option<TleParameters>,
}

impl OrbitMeanElements {
    pub fn builder() -> OrbitMeanElementsBuilder {
        <OrbitMeanElementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrbitMeanElementsBuilder {
    metadata: Option<OrbitMeanElementsMetadata>,
    mean_keplerian_elements: Option<MeanKeplerianElements>,
    tle_parameters: Option<TleParameters>,
}

impl OrbitMeanElementsBuilder {
    pub fn metadata(mut self, value: OrbitMeanElementsMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mean_keplerian_elements(mut self, value: MeanKeplerianElements) -> Self {
        self.mean_keplerian_elements = Some(value);
        self
    }

    pub fn tle_parameters(mut self, value: TleParameters) -> Self {
        self.tle_parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrbitMeanElements`].
    pub fn build(self) -> Result<OrbitMeanElements, BuildError> {
        Ok(OrbitMeanElements {
            metadata: self.metadata,
            mean_keplerian_elements: self.mean_keplerian_elements,
            tle_parameters: self.tle_parameters,
        })
    }
}
