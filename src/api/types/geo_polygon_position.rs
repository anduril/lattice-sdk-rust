pub use crate::prelude::*;

/// A position in a GeoPolygon with an optional extruded height.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoPolygonPosition {
    /// base position. if no altitude set, its on the ground.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// optional height above base position to extrude in meters.
    /// for a given polygon, all points should have a height or none of them.
    /// strictly GeoJSON compatible polygons will not have this set.
    #[serde(rename = "heightM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub height_m: Option<f64>,
}

impl GeoPolygonPosition {
    pub fn builder() -> GeoPolygonPositionBuilder {
        <GeoPolygonPositionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoPolygonPositionBuilder {
    position: Option<Position>,
    height_m: Option<f64>,
}

impl GeoPolygonPositionBuilder {
    pub fn position(mut self, value: Position) -> Self {
        self.position = Some(value);
        self
    }

    pub fn height_m(mut self, value: f64) -> Self {
        self.height_m = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoPolygonPosition`].
    pub fn build(self) -> Result<GeoPolygonPosition, BuildError> {
        Ok(GeoPolygonPosition {
            position: self.position,
            height_m: self.height_m,
        })
    }
}
