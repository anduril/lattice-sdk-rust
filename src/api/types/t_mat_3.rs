pub use crate::prelude::*;

/// A symmetric 3D matrix only representing the upper right triangle, useful for covariance matrices.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TMat3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mxx: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mxy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mxz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub myy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub myz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub mzz: Option<f64>,
}

impl TMat3 {
    pub fn builder() -> TMat3Builder {
        <TMat3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TMat3Builder {
    mxx: Option<f64>,
    mxy: Option<f64>,
    mxz: Option<f64>,
    myy: Option<f64>,
    myz: Option<f64>,
    mzz: Option<f64>,
}

impl TMat3Builder {
    pub fn mxx(mut self, value: f64) -> Self {
        self.mxx = Some(value);
        self
    }

    pub fn mxy(mut self, value: f64) -> Self {
        self.mxy = Some(value);
        self
    }

    pub fn mxz(mut self, value: f64) -> Self {
        self.mxz = Some(value);
        self
    }

    pub fn myy(mut self, value: f64) -> Self {
        self.myy = Some(value);
        self
    }

    pub fn myz(mut self, value: f64) -> Self {
        self.myz = Some(value);
        self
    }

    pub fn mzz(mut self, value: f64) -> Self {
        self.mzz = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TMat3`].
    pub fn build(self) -> Result<TMat3, BuildError> {
        Ok(TMat3 {
            mxx: self.mxx,
            mxy: self.mxy,
            mxz: self.mxz,
            myy: self.myy,
            myz: self.myz,
            mzz: self.mzz,
        })
    }
}
