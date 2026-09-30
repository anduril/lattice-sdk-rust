pub use crate::prelude::*;

/// Altitude above the WGS84 defined ellipsoid. Often measured with a GNSS sensor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveWgs84Ellipsoid {
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

impl AltitudeAboveWgs84Ellipsoid {
    pub fn builder() -> AltitudeAboveWgs84EllipsoidBuilder {
        <AltitudeAboveWgs84EllipsoidBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveWgs84EllipsoidBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveWgs84EllipsoidBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveWgs84Ellipsoid`].
    pub fn build(self) -> Result<AltitudeAboveWgs84Ellipsoid, BuildError> {
        Ok(AltitudeAboveWgs84Ellipsoid {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
