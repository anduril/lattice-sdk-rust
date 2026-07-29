pub use crate::prelude::*;

/// The calibrated pressure altitude reading measured above MSL. This is known as QNH or true altitude in aviation
/// terms. This is separated from AltitudeAboveMeanSeaLevelEGM96, as the two values are not guaranteed to be the
/// same due to temperature fluctuations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveMeanSeaLevelPressure {
    /// The provenance of the measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<AltitudeProvenance>,
    /// The altitude value in meters.
    #[serde(rename = "valueMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value_meters: Option<f64>,
}

impl AltitudeAboveMeanSeaLevelPressure {
    pub fn builder() -> AltitudeAboveMeanSeaLevelPressureBuilder {
        <AltitudeAboveMeanSeaLevelPressureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveMeanSeaLevelPressureBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveMeanSeaLevelPressureBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveMeanSeaLevelPressure`].
    pub fn build(self) -> Result<AltitudeAboveMeanSeaLevelPressure, BuildError> {
        Ok(AltitudeAboveMeanSeaLevelPressure {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
