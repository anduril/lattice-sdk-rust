pub use crate::prelude::*;

/// A polygon shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.6, only canonical representations accepted
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoPolygon {
    /// An array of LinearRings where the first item is the exterior ring and subsequent items are interior rings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rings: Option<Vec<LinearRing>>,
    /// An extension hint that this polygon is a rectangle. When true this implies several things:
    /// * exactly 1 linear ring with 5 points (starting corner, 3 other corners and start again)
    /// * each point has the same altitude corresponding with the plane of the rectangle
    /// * each point has the same height (either all present and equal, or all not present)
    #[serde(rename = "isRectangle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rectangle: Option<bool>,
}

impl GeoPolygon {
    pub fn builder() -> GeoPolygonBuilder {
        <GeoPolygonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoPolygonBuilder {
    rings: Option<Vec<LinearRing>>,
    is_rectangle: Option<bool>,
}

impl GeoPolygonBuilder {
    pub fn rings(mut self, value: Vec<LinearRing>) -> Self {
        self.rings = Some(value);
        self
    }

    pub fn is_rectangle(mut self, value: bool) -> Self {
        self.is_rectangle = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoPolygon`].
    pub fn build(self) -> Result<GeoPolygon, BuildError> {
        Ok(GeoPolygon {
            rings: self.rings,
            is_rectangle: self.is_rectangle,
        })
    }
}
