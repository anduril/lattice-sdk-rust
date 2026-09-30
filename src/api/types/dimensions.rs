pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Dimensions {
    /// Length of the entity in meters
    #[serde(rename = "lengthM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub length_m: Option<f64>,
}

impl Dimensions {
    pub fn builder() -> DimensionsBuilder {
        <DimensionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DimensionsBuilder {
    length_m: Option<f64>,
}

impl DimensionsBuilder {
    pub fn length_m(mut self, value: f64) -> Self {
        self.length_m = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Dimensions`].
    pub fn build(self) -> Result<Dimensions, BuildError> {
        Ok(Dimensions {
            length_m: self.length_m,
        })
    }
}
