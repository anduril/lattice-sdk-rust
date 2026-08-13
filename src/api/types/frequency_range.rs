pub use crate::prelude::*;

/// A component to represent a frequency range.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FrequencyRange {
    /// Indicates the lowest measured frequency of a signal (Hz).
    #[serde(rename = "minimumFrequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_hz: Option<Frequency>,
    /// Indicates the maximum measured frequency of a signal (Hz).
    #[serde(rename = "maximumFrequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_frequency_hz: Option<Frequency>,
}

impl FrequencyRange {
    pub fn builder() -> FrequencyRangeBuilder {
        <FrequencyRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrequencyRangeBuilder {
    minimum_frequency_hz: Option<Frequency>,
    maximum_frequency_hz: Option<Frequency>,
}

impl FrequencyRangeBuilder {
    pub fn minimum_frequency_hz(mut self, value: Frequency) -> Self {
        self.minimum_frequency_hz = Some(value);
        self
    }

    pub fn maximum_frequency_hz(mut self, value: Frequency) -> Self {
        self.maximum_frequency_hz = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FrequencyRange`].
    pub fn build(self) -> Result<FrequencyRange, BuildError> {
        Ok(FrequencyRange {
            minimum_frequency_hz: self.minimum_frequency_hz,
            maximum_frequency_hz: self.maximum_frequency_hz,
        })
    }
}
