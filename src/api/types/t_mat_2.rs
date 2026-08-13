pub use crate::prelude::*;

/// symmetric 2d matrix only representing the upper right triangle, useful for
/// covariance matrices
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TMat2 {
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
    pub myy: Option<f64>,
}

impl TMat2 {
    pub fn builder() -> TMat2Builder {
        <TMat2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TMat2Builder {
    mxx: Option<f64>,
    mxy: Option<f64>,
    myy: Option<f64>,
}

impl TMat2Builder {
    pub fn mxx(mut self, value: f64) -> Self {
        self.mxx = Some(value);
        self
    }

    pub fn mxy(mut self, value: f64) -> Self {
        self.mxy = Some(value);
        self
    }

    pub fn myy(mut self, value: f64) -> Self {
        self.myy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TMat2`].
    pub fn build(self) -> Result<TMat2, BuildError> {
        Ok(TMat2 {
            mxx: self.mxx,
            mxy: self.mxy,
            myy: self.myy,
        })
    }
}
