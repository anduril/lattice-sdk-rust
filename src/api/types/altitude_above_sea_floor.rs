pub use crate::prelude::*;

/// The altitude above the sea floor, generally measured with a sonar.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveSeaFloor {
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

impl AltitudeAboveSeaFloor {
    pub fn builder() -> AltitudeAboveSeaFloorBuilder {
        <AltitudeAboveSeaFloorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveSeaFloorBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveSeaFloorBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveSeaFloor`].
    pub fn build(self) -> Result<AltitudeAboveSeaFloor, BuildError> {
        Ok(AltitudeAboveSeaFloor {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
