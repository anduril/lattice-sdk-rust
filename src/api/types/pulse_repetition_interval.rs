pub use crate::prelude::*;

/// A component that describe the length in time between two pulses
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PulseRepetitionInterval {
    #[serde(rename = "pulseRepetitionIntervalS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_repetition_interval_s: Option<Measurement>,
}

impl PulseRepetitionInterval {
    pub fn builder() -> PulseRepetitionIntervalBuilder {
        <PulseRepetitionIntervalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PulseRepetitionIntervalBuilder {
    pulse_repetition_interval_s: Option<Measurement>,
}

impl PulseRepetitionIntervalBuilder {
    pub fn pulse_repetition_interval_s(mut self, value: Measurement) -> Self {
        self.pulse_repetition_interval_s = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PulseRepetitionInterval`].
    pub fn build(self) -> Result<PulseRepetitionInterval, BuildError> {
        Ok(PulseRepetitionInterval {
            pulse_repetition_interval_s: self.pulse_repetition_interval_s,
        })
    }
}
