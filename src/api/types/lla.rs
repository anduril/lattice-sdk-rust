pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Lla {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lon: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is2d: Option<bool>,
    /// Meaning of alt.
    /// altitude in meters above either WGS84 or EGM96, use altitude_reference to
    /// determine what zero means.
    #[serde(rename = "altitudeReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_reference: Option<LlaAltitudeReference>,
}

impl Lla {
    pub fn builder() -> LlaBuilder {
        <LlaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlaBuilder {
    lon: Option<f64>,
    lat: Option<f64>,
    alt: Option<f64>,
    is2d: Option<bool>,
    altitude_reference: Option<LlaAltitudeReference>,
}

impl LlaBuilder {
    pub fn lon(mut self, value: f64) -> Self {
        self.lon = Some(value);
        self
    }

    pub fn lat(mut self, value: f64) -> Self {
        self.lat = Some(value);
        self
    }

    pub fn alt(mut self, value: f64) -> Self {
        self.alt = Some(value);
        self
    }

    pub fn is2d(mut self, value: bool) -> Self {
        self.is2d = Some(value);
        self
    }

    pub fn altitude_reference(mut self, value: LlaAltitudeReference) -> Self {
        self.altitude_reference = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Lla`].
    pub fn build(self) -> Result<Lla, BuildError> {
        Ok(Lla {
            lon: self.lon,
            lat: self.lat,
            alt: self.alt,
            is2d: self.is2d,
            altitude_reference: self.altitude_reference,
        })
    }
}
