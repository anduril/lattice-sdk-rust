pub use crate::prelude::*;

/// Range rings allow visual assessment of map distance at varying zoom levels.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RangeRings {
    /// The minimum range ring distance, specified in meters.
    #[serde(rename = "minDistanceM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_distance_m: Option<f64>,
    /// The maximum range ring distance, specified in meters.
    #[serde(rename = "maxDistanceM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_distance_m: Option<f64>,
    /// The count of range rings.
    #[serde(rename = "ringCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_count: Option<i64>,
    /// The color of range rings, specified in hex string.
    #[serde(rename = "ringLineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_line_color: Option<Color>,
}

impl RangeRings {
    pub fn builder() -> RangeRingsBuilder {
        <RangeRingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RangeRingsBuilder {
    min_distance_m: Option<f64>,
    max_distance_m: Option<f64>,
    ring_count: Option<i64>,
    ring_line_color: Option<Color>,
}

impl RangeRingsBuilder {
    pub fn min_distance_m(mut self, value: f64) -> Self {
        self.min_distance_m = Some(value);
        self
    }

    pub fn max_distance_m(mut self, value: f64) -> Self {
        self.max_distance_m = Some(value);
        self
    }

    pub fn ring_count(mut self, value: i64) -> Self {
        self.ring_count = Some(value);
        self
    }

    pub fn ring_line_color(mut self, value: Color) -> Self {
        self.ring_line_color = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RangeRings`].
    pub fn build(self) -> Result<RangeRings, BuildError> {
        Ok(RangeRings {
            min_distance_m: self.min_distance_m,
            max_distance_m: self.max_distance_m,
            ring_count: self.ring_count,
            ring_line_color: self.ring_line_color,
        })
    }
}
