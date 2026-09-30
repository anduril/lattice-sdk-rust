pub use crate::prelude::*;

/// Location measurement in reference to the center of the earth using the ECEF
/// coordinate system. This is in the WGS84 coordinate frame.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationGeocentricEcef {
    /// The plane of the equator, passing through extending from 90°W longitude (negative)
    /// to 90°E longitude (positive).
    #[serde(rename = "xMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x_meters: Option<f64>,
    /// The plane of the equator, passing through the origin and extending from 180° longitude
    /// (negative) to the prime meridian.
    #[serde(rename = "yMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y_meters: Option<f64>,
    /// The line between the North and South Poles, with positive values increasing northward.
    #[serde(rename = "zMeters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub z_meters: Option<f64>,
}

impl LocationGeocentricEcef {
    pub fn builder() -> LocationGeocentricEcefBuilder {
        <LocationGeocentricEcefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocationGeocentricEcefBuilder {
    x_meters: Option<f64>,
    y_meters: Option<f64>,
    z_meters: Option<f64>,
}

impl LocationGeocentricEcefBuilder {
    pub fn x_meters(mut self, value: f64) -> Self {
        self.x_meters = Some(value);
        self
    }

    pub fn y_meters(mut self, value: f64) -> Self {
        self.y_meters = Some(value);
        self
    }

    pub fn z_meters(mut self, value: f64) -> Self {
        self.z_meters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocationGeocentricEcef`].
    pub fn build(self) -> Result<LocationGeocentricEcef, BuildError> {
        Ok(LocationGeocentricEcef {
            x_meters: self.x_meters,
            y_meters: self.y_meters,
            z_meters: self.z_meters,
        })
    }
}
