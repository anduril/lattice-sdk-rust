pub use crate::prelude::*;

/// A component for describing frequency.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Frequency {
    /// Indicates a frequency of a signal (Hz) with its standard deviation.
    #[serde(rename = "frequencyHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_hz: Option<Measurement>,
}

impl Frequency {
    pub fn builder() -> FrequencyBuilder {
        <FrequencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrequencyBuilder {
    frequency_hz: Option<Measurement>,
}

impl FrequencyBuilder {
    pub fn frequency_hz(mut self, value: Measurement) -> Self {
        self.frequency_hz = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Frequency`].
    pub fn build(self) -> Result<Frequency, BuildError> {
        Ok(Frequency {
            frequency_hz: self.frequency_hz,
        })
    }
}
