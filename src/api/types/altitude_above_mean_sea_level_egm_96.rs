pub use crate::prelude::*;

/// The altitude relative to mean sea level represented by the EGM96 geoid. This is often calculated using a terrain
/// conversion of from a GNSS device.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveMeanSeaLevelEgm96 {
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

impl AltitudeAboveMeanSeaLevelEgm96 {
    pub fn builder() -> AltitudeAboveMeanSeaLevelEgm96Builder {
        <AltitudeAboveMeanSeaLevelEgm96Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveMeanSeaLevelEgm96Builder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveMeanSeaLevelEgm96Builder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveMeanSeaLevelEgm96`].
    pub fn build(self) -> Result<AltitudeAboveMeanSeaLevelEgm96, BuildError> {
        Ok(AltitudeAboveMeanSeaLevelEgm96 {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
