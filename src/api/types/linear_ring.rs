pub use crate::prelude::*;

/// A closed ring of points. The first and last point must be the same.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LinearRing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<GeoPolygonPosition>>,
}

impl LinearRing {
    pub fn builder() -> LinearRingBuilder {
        <LinearRingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinearRingBuilder {
    positions: Option<Vec<GeoPolygonPosition>>,
}

impl LinearRingBuilder {
    pub fn positions(mut self, value: Vec<GeoPolygonPosition>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinearRing`].
    pub fn build(self) -> Result<LinearRing, BuildError> {
        Ok(LinearRing {
            positions: self.positions,
        })
    }
}
