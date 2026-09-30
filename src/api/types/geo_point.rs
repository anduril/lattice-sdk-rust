pub use crate::prelude::*;

/// A point shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

impl GeoPoint {
    pub fn builder() -> GeoPointBuilder {
        <GeoPointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoPointBuilder {
    position: Option<Position>,
}

impl GeoPointBuilder {
    pub fn position(mut self, value: Position) -> Self {
        self.position = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoPoint`].
    pub fn build(self) -> Result<GeoPoint, BuildError> {
        Ok(GeoPoint {
            position: self.position,
        })
    }
}
