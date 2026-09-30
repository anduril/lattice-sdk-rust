pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Quaternion {
    /// x, y, z are vector portion, w is scalar
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub w: Option<f64>,
}

impl Quaternion {
    pub fn builder() -> QuaternionBuilder {
        <QuaternionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuaternionBuilder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    w: Option<f64>,
}

impl QuaternionBuilder {
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

    pub fn w(mut self, value: f64) -> Self {
        self.w = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Quaternion`].
    pub fn build(self) -> Result<Quaternion, BuildError> {
        Ok(Quaternion {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        })
    }
}
