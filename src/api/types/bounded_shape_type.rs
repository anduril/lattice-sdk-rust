pub use crate::prelude::*;

/// The BoundedShapeType represents any static fully-enclosed shape.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BoundedShapeType {
    #[serde(rename = "polygonValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_value: Option<GeoPolygon>,
}

impl BoundedShapeType {
    pub fn builder() -> BoundedShapeTypeBuilder {
        <BoundedShapeTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoundedShapeTypeBuilder {
    polygon_value: Option<GeoPolygon>,
}

impl BoundedShapeTypeBuilder {
    pub fn polygon_value(mut self, value: GeoPolygon) -> Self {
        self.polygon_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BoundedShapeType`].
    pub fn build(self) -> Result<BoundedShapeType, BuildError> {
        Ok(BoundedShapeType {
            polygon_value: self.polygon_value,
        })
    }
}
