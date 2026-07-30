pub use crate::prelude::*;

/// An ellipsoid shaped geo-entity.
/// Principal axis lengths are defined in entity body space
/// This shape is NOT Geo-JSON compatible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoEllipsoid {
    /// Defines the distance from the center point to the surface along the forward axis
    #[serde(rename = "forwardAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub forward_axis_m: Option<f64>,
    /// Defines the distance from the center point to the surface along the side axis
    #[serde(rename = "sideAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub side_axis_m: Option<f64>,
    /// Defines the distance from the center point to the surface along the up axis
    #[serde(rename = "upAxisM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub up_axis_m: Option<f64>,
}

impl GeoEllipsoid {
    pub fn builder() -> GeoEllipsoidBuilder {
        <GeoEllipsoidBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoEllipsoidBuilder {
    forward_axis_m: Option<f64>,
    side_axis_m: Option<f64>,
    up_axis_m: Option<f64>,
}

impl GeoEllipsoidBuilder {
    pub fn forward_axis_m(mut self, value: f64) -> Self {
        self.forward_axis_m = Some(value);
        self
    }

    pub fn side_axis_m(mut self, value: f64) -> Self {
        self.side_axis_m = Some(value);
        self
    }

    pub fn up_axis_m(mut self, value: f64) -> Self {
        self.up_axis_m = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoEllipsoid`].
    pub fn build(self) -> Result<GeoEllipsoid, BuildError> {
        Ok(GeoEllipsoid {
            forward_axis_m: self.forward_axis_m,
            side_axis_m: self.side_axis_m,
            up_axis_m: self.up_axis_m,
        })
    }
}
