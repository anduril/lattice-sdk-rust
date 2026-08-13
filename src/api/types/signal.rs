pub use crate::prelude::*;

/// A component that describes an entity's signal characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Signal {
    #[serde(rename = "frequencyCenter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_center: Option<Frequency>,
    #[serde(rename = "frequencyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_range: Option<FrequencyRange>,
    /// Indicates the bandwidth of a signal (Hz).
    #[serde(rename = "bandwidthHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bandwidth_hz: Option<f64>,
    /// Indicates the signal to noise (SNR) of this signal.
    #[serde(rename = "signalToNoiseRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub signal_to_noise_ratio: Option<f64>,
    #[serde(rename = "lineOfBearing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_bearing: Option<LineOfBearing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Fixed>,
    /// Emitter notations associated with this entity.
    #[serde(rename = "emitterNotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitter_notations: Option<Vec<EmitterNotation>>,
    /// length in time of a single pulse
    #[serde(rename = "pulseWidthS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pulse_width_s: Option<f64>,
    /// length in time between the start of two pulses
    #[serde(rename = "pulseRepetitionInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_repetition_interval: Option<PulseRepetitionInterval>,
    /// describes how a signal is observing the environment
    #[serde(rename = "scanCharacteristics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_characteristics: Option<ScanCharacteristics>,
}

impl Signal {
    pub fn builder() -> SignalBuilder {
        <SignalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignalBuilder {
    frequency_center: Option<Frequency>,
    frequency_range: Option<FrequencyRange>,
    bandwidth_hz: Option<f64>,
    signal_to_noise_ratio: Option<f64>,
    line_of_bearing: Option<LineOfBearing>,
    fixed: Option<Fixed>,
    emitter_notations: Option<Vec<EmitterNotation>>,
    pulse_width_s: Option<f64>,
    pulse_repetition_interval: Option<PulseRepetitionInterval>,
    scan_characteristics: Option<ScanCharacteristics>,
}

impl SignalBuilder {
    pub fn frequency_center(mut self, value: Frequency) -> Self {
        self.frequency_center = Some(value);
        self
    }

    pub fn frequency_range(mut self, value: FrequencyRange) -> Self {
        self.frequency_range = Some(value);
        self
    }

    pub fn bandwidth_hz(mut self, value: f64) -> Self {
        self.bandwidth_hz = Some(value);
        self
    }

    pub fn signal_to_noise_ratio(mut self, value: f64) -> Self {
        self.signal_to_noise_ratio = Some(value);
        self
    }

    pub fn line_of_bearing(mut self, value: LineOfBearing) -> Self {
        self.line_of_bearing = Some(value);
        self
    }

    pub fn fixed(mut self, value: Fixed) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn emitter_notations(mut self, value: Vec<EmitterNotation>) -> Self {
        self.emitter_notations = Some(value);
        self
    }

    pub fn pulse_width_s(mut self, value: f64) -> Self {
        self.pulse_width_s = Some(value);
        self
    }

    pub fn pulse_repetition_interval(mut self, value: PulseRepetitionInterval) -> Self {
        self.pulse_repetition_interval = Some(value);
        self
    }

    pub fn scan_characteristics(mut self, value: ScanCharacteristics) -> Self {
        self.scan_characteristics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Signal`].
    pub fn build(self) -> Result<Signal, BuildError> {
        Ok(Signal {
            frequency_center: self.frequency_center,
            frequency_range: self.frequency_range,
            bandwidth_hz: self.bandwidth_hz,
            signal_to_noise_ratio: self.signal_to_noise_ratio,
            line_of_bearing: self.line_of_bearing,
            fixed: self.fixed,
            emitter_notations: self.emitter_notations,
            pulse_width_s: self.pulse_width_s,
            pulse_repetition_interval: self.pulse_repetition_interval,
            scan_characteristics: self.scan_characteristics,
        })
    }
}
