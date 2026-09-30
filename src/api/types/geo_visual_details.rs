pub use crate::prelude::*;

/// Details specific to displaying a geo-entity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeoVisualDetails {
    /// Describes the fill color of a geo-entity.
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<Color>,
    /// Describes the line color of a geo-entity.
    #[serde(rename = "lineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<Color>,
}

impl GeoVisualDetails {
    pub fn builder() -> GeoVisualDetailsBuilder {
        <GeoVisualDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeoVisualDetailsBuilder {
    fill_color: Option<Color>,
    line_color: Option<Color>,
}

impl GeoVisualDetailsBuilder {
    pub fn fill_color(mut self, value: Color) -> Self {
        self.fill_color = Some(value);
        self
    }

    pub fn line_color(mut self, value: Color) -> Self {
        self.line_color = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeoVisualDetails`].
    pub fn build(self) -> Result<GeoVisualDetails, BuildError> {
        Ok(GeoVisualDetails {
            fill_color: self.fill_color,
            line_color: self.line_color,
        })
    }
}
