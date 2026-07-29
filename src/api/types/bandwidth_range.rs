pub use crate::prelude::*;

/// A component that describes the min and max bandwidths of a sensor
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BandwidthRange {
    #[serde(rename = "minimumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_bandwidth: Option<Bandwidth>,
    #[serde(rename = "maximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bandwidth: Option<Bandwidth>,
}

impl BandwidthRange {
    pub fn builder() -> BandwidthRangeBuilder {
        <BandwidthRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BandwidthRangeBuilder {
    minimum_bandwidth: Option<Bandwidth>,
    maximum_bandwidth: Option<Bandwidth>,
}

impl BandwidthRangeBuilder {
    pub fn minimum_bandwidth(mut self, value: Bandwidth) -> Self {
        self.minimum_bandwidth = Some(value);
        self
    }

    pub fn maximum_bandwidth(mut self, value: Bandwidth) -> Self {
        self.maximum_bandwidth = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BandwidthRange`].
    pub fn build(self) -> Result<BandwidthRange, BuildError> {
        Ok(BandwidthRange {
            minimum_bandwidth: self.minimum_bandwidth,
            maximum_bandwidth: self.maximum_bandwidth,
        })
    }
}
