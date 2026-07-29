pub use crate::prelude::*;

/// Indicates ellipse characteristics and probability that an entity lies within the defined ellipse.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ErrorEllipse {
    /// Defines the probability in percentage that an entity lies within the given ellipse: 0-1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub probability: Option<f64>,
    /// Defines the distance from the center point of the ellipse to the furthest distance on the perimeter in meters.
    #[serde(rename = "semiMajorAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub semi_major_axis_m: Option<f64>,
    /// Defines the distance from the center point of the ellipse to the shortest distance on the perimeter in meters.
    #[serde(rename = "semiMinorAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub semi_minor_axis_m: Option<f64>,
    /// The orientation of the semi-major relative to true north in degrees from clockwise: 0-180 due to symmetry across the semi-minor axis.
    #[serde(rename = "orientationD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub orientation_d: Option<f64>,
}

impl ErrorEllipse {
    pub fn builder() -> ErrorEllipseBuilder {
        <ErrorEllipseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorEllipseBuilder {
    probability: Option<f64>,
    semi_major_axis_m: Option<f64>,
    semi_minor_axis_m: Option<f64>,
    orientation_d: Option<f64>,
}

impl ErrorEllipseBuilder {
    pub fn probability(mut self, value: f64) -> Self {
        self.probability = Some(value);
        self
    }

    pub fn semi_major_axis_m(mut self, value: f64) -> Self {
        self.semi_major_axis_m = Some(value);
        self
    }

    pub fn semi_minor_axis_m(mut self, value: f64) -> Self {
        self.semi_minor_axis_m = Some(value);
        self
    }

    pub fn orientation_d(mut self, value: f64) -> Self {
        self.orientation_d = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorEllipse`].
    pub fn build(self) -> Result<ErrorEllipse, BuildError> {
        Ok(ErrorEllipse {
            probability: self.probability,
            semi_major_axis_m: self.semi_major_axis_m,
            semi_minor_axis_m: self.semi_minor_axis_m,
            orientation_d: self.orientation_d,
        })
    }
}
