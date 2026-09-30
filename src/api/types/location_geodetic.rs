pub use crate::prelude::*;

/// Geodetic location measurement in reference to the WGS84 ellipsoid. This also optionally
/// provides other altitude reference frames.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationGeodetic {
    /// WGS84 latitude in decimal degrees.
    #[serde(rename = "latitudeDegrees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude_degrees: Option<f64>,
    /// WGS84 longitude in decimal degrees.
    #[serde(rename = "longitudeDegrees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude_degrees: Option<f64>,
    /// Altitude measurement in reference to the WGS84 defined ellipsoid. This is expected to
    /// always be set if an altitude measurement is available and should be derived from the
    /// most accurate altitude measurement available. If this is a 2D measurement, then this
    /// message should not be set. If you are unable to calculate this value, then this
    /// message should also not be set.
    #[serde(rename = "universalAltitudeHae")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_altitude_hae: Option<AltitudeAboveWgs84Ellipsoid>,
    /// This allows for multiple additional altitudes to be conveyed.
    /// e.g. Barometric Pressure and Radar Altimeter readings
    /// for an aircraft
    #[serde(rename = "additionalAltitudes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_altitudes: Option<Vec<Altitude>>,
}

impl LocationGeodetic {
    pub fn builder() -> LocationGeodeticBuilder {
        <LocationGeodeticBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocationGeodeticBuilder {
    latitude_degrees: Option<f64>,
    longitude_degrees: Option<f64>,
    universal_altitude_hae: Option<AltitudeAboveWgs84Ellipsoid>,
    additional_altitudes: Option<Vec<Altitude>>,
}

impl LocationGeodeticBuilder {
    pub fn latitude_degrees(mut self, value: f64) -> Self {
        self.latitude_degrees = Some(value);
        self
    }

    pub fn longitude_degrees(mut self, value: f64) -> Self {
        self.longitude_degrees = Some(value);
        self
    }

    pub fn universal_altitude_hae(mut self, value: AltitudeAboveWgs84Ellipsoid) -> Self {
        self.universal_altitude_hae = Some(value);
        self
    }

    pub fn additional_altitudes(mut self, value: Vec<Altitude>) -> Self {
        self.additional_altitudes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocationGeodetic`].
    pub fn build(self) -> Result<LocationGeodetic, BuildError> {
        Ok(LocationGeodetic {
            latitude_degrees: self.latitude_degrees,
            longitude_degrees: self.longitude_degrees,
            universal_altitude_hae: self.universal_altitude_hae,
            additional_altitudes: self.additional_altitudes,
        })
    }
}
