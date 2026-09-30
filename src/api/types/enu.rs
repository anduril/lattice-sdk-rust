pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Enu {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub e: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub n: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub u: Option<f64>,
}

impl Enu {
    pub fn builder() -> EnuBuilder {
        <EnuBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnuBuilder {
    e: Option<f64>,
    n: Option<f64>,
    u: Option<f64>,
}

impl EnuBuilder {
    pub fn e(mut self, value: f64) -> Self {
        self.e = Some(value);
        self
    }

    pub fn n(mut self, value: f64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn u(mut self, value: f64) -> Self {
        self.u = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Enu`].
    pub fn build(self) -> Result<Enu, BuildError> {
        Ok(Enu {
            e: self.e,
            n: self.n,
            u: self.u,
        })
    }
}
