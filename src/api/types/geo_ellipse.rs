pub use crate::prelude::*;

/// An ellipse shaped geo-entity.
/// For a circle, the major and minor axis would be the same values.
/// This shape is NOT Geo-JSON compatible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoEllipse {
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
    /// Optional height above entity position to extrude in meters. A non-zero value creates an elliptic cylinder
    #[serde(rename = "heightM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub height_m: Option<f64>,
}

impl GeoEllipse {
    pub fn builder() -> GeoEllipseBuilder {
        <GeoEllipseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoEllipseBuilder {
    semi_major_axis_m: Option<f64>,
    semi_minor_axis_m: Option<f64>,
    orientation_d: Option<f64>,
    height_m: Option<f64>,
}

impl GeoEllipseBuilder {
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

    pub fn height_m(mut self, value: f64) -> Self {
        self.height_m = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoEllipse`].
    pub fn build(self) -> Result<GeoEllipse, BuildError> {
        Ok(GeoEllipse {
            semi_major_axis_m: self.semi_major_axis_m,
            semi_minor_axis_m: self.semi_minor_axis_m,
            orientation_d: self.orientation_d,
            height_m: self.height_m,
        })
    }
}
