pub use crate::prelude::*;

/// Represents RF configurations supported on this sensor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RfConfiguration {
    /// Frequency ranges that are available for this sensor.
    #[serde(rename = "frequencyRangeHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_range_hz: Option<Vec<FrequencyRange>>,
    /// Bandwidth ranges that are available for this sensor.
    #[serde(rename = "bandwidthRangeHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_range_hz: Option<Vec<BandwidthRange>>,
}

impl RfConfiguration {
    pub fn builder() -> RfConfigurationBuilder {
        <RfConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RfConfigurationBuilder {
    frequency_range_hz: Option<Vec<FrequencyRange>>,
    bandwidth_range_hz: Option<Vec<BandwidthRange>>,
}

impl RfConfigurationBuilder {
    pub fn frequency_range_hz(mut self, value: Vec<FrequencyRange>) -> Self {
        self.frequency_range_hz = Some(value);
        self
    }

    pub fn bandwidth_range_hz(mut self, value: Vec<BandwidthRange>) -> Self {
        self.bandwidth_range_hz = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RfConfiguration`].
    pub fn build(self) -> Result<RfConfiguration, BuildError> {
        Ok(RfConfiguration {
            frequency_range_hz: self.frequency_range_hz,
            bandwidth_range_hz: self.bandwidth_range_hz,
        })
    }
}
