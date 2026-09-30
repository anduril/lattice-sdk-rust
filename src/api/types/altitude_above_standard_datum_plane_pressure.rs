pub use crate::prelude::*;

/// The altitude reading measured above the standard datum plane (29.92 inHg or 1013.2 hPA). This is also known as
/// pressure altitude or QNE in aviation terms. This altitude should be used when flying at high altitudes and
/// above the transition level (18,000 in the USA and Canada), ensuring the use of a common reference altitude.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AltitudeAboveStandardDatumPlanePressure {
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

impl AltitudeAboveStandardDatumPlanePressure {
    pub fn builder() -> AltitudeAboveStandardDatumPlanePressureBuilder {
        <AltitudeAboveStandardDatumPlanePressureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AltitudeAboveStandardDatumPlanePressureBuilder {
    provenance: Option<AltitudeProvenance>,
    value_meters: Option<f64>,
}

impl AltitudeAboveStandardDatumPlanePressureBuilder {
    pub fn provenance(mut self, value: AltitudeProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn value_meters(mut self, value: f64) -> Self {
        self.value_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AltitudeAboveStandardDatumPlanePressure`].
    pub fn build(self) -> Result<AltitudeAboveStandardDatumPlanePressure, BuildError> {
        Ok(AltitudeAboveStandardDatumPlanePressure {
            provenance: self.provenance,
            value_meters: self.value_meters,
        })
    }
}
