pub use crate::prelude::*;

/// A component that describes some measured value with error.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Measurement {
    /// The value of the measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
    /// Estimated one standard deviation in same unit as the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sigma: Option<f64>,
}

impl Measurement {
    pub fn builder() -> MeasurementBuilder {
        <MeasurementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeasurementBuilder {
    value: Option<f64>,
    sigma: Option<f64>,
}

impl MeasurementBuilder {
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn sigma(mut self, value: f64) -> Self {
        self.sigma = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Measurement`].
    pub fn build(self) -> Result<Measurement, BuildError> {
        Ok(Measurement {
            value: self.value,
            sigma: self.sigma,
        })
    }
}
