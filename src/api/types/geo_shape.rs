pub use crate::prelude::*;

/// A component that describes the shape of a geo-entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<GeoPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<GeoLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<GeoPolygon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipse: Option<GeoEllipse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<GeoEllipsoid>,
}

impl GeoShape {
    pub fn builder() -> GeoShapeBuilder {
        <GeoShapeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoShapeBuilder {
    point: Option<GeoPoint>,
    line: Option<GeoLine>,
    polygon: Option<GeoPolygon>,
    ellipse: Option<GeoEllipse>,
    ellipsoid: Option<GeoEllipsoid>,
}

impl GeoShapeBuilder {
    pub fn point(mut self, value: GeoPoint) -> Self {
        self.point = Some(value);
        self
    }

    pub fn line(mut self, value: GeoLine) -> Self {
        self.line = Some(value);
        self
    }

    pub fn polygon(mut self, value: GeoPolygon) -> Self {
        self.polygon = Some(value);
        self
    }

    pub fn ellipse(mut self, value: GeoEllipse) -> Self {
        self.ellipse = Some(value);
        self
    }

    pub fn ellipsoid(mut self, value: GeoEllipsoid) -> Self {
        self.ellipsoid = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoShape`].
    pub fn build(self) -> Result<GeoShape, BuildError> {
        Ok(GeoShape {
            point: self.point,
            line: self.line,
            polygon: self.polygon,
            ellipse: self.ellipse,
            ellipsoid: self.ellipsoid,
        })
    }
}
