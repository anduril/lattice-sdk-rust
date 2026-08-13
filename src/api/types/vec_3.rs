pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub z: Option<f64>,
}

impl Vec3 {
    pub fn builder() -> Vec3Builder {
        <Vec3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Vec3Builder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}

impl Vec3Builder {
    pub fn x(mut self, value: f64) -> Self {
        self.x = Some(value);
        self
    }

    pub fn y(mut self, value: f64) -> Self {
        self.y = Some(value);
        self
    }

    pub fn z(mut self, value: f64) -> Self {
        self.z = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Vec3`].
    pub fn build(self) -> Result<Vec3, BuildError> {
        Ok(Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        })
    }
}
