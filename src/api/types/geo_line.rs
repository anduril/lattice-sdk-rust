pub use crate::prelude::*;

/// A line shaped geo-entity.
/// See https://datatracker.ietf.org/doc/html/rfc7946#section-3.1.4
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
}

impl GeoLine {
    pub fn builder() -> GeoLineBuilder {
        <GeoLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoLineBuilder {
    positions: Option<Vec<Position>>,
}

impl GeoLineBuilder {
    pub fn positions(mut self, value: Vec<Position>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoLine`].
    pub fn build(self) -> Result<GeoLine, BuildError> {
        Ok(GeoLine {
            positions: self.positions,
        })
    }
}
