pub use crate::prelude::*;

/// The altitude below sea surface, generally measured with a pressure depth sensor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeBelowSeaSurface {
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

impl AltitudeBelowSeaSurface {
    pub fn builder() -> AltitudeBelowSeaSurfaceBuilder {
        <AltitudeBelowSeaSurfaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeBelowSeaSurfaceBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeBelowSeaSurfaceBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeBelowSeaSurface`].
    pub fn build(self) -> Result<AltitudeBelowSeaSurface, BuildError> {
        Ok(AltitudeBelowSeaSurface {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
