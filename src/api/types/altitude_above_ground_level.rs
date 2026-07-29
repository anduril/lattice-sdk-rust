pub use crate::prelude::*;

/// Altitude as AGL (Above ground level). This is also known as absolute altitude or QFE in aviation terms.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveGroundLevel {
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

impl AltitudeAboveGroundLevel {
    pub fn builder() -> AltitudeAboveGroundLevelBuilder {
        <AltitudeAboveGroundLevelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveGroundLevelBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveGroundLevelBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveGroundLevel`].
    pub fn build(self) -> Result<AltitudeAboveGroundLevel, BuildError> {
        Ok(AltitudeAboveGroundLevel {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
