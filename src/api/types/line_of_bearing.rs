pub use crate::prelude::*;

/// A line of bearing of a signal.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LineOfBearing {
    /// The direction pointing from this entity to the detection
    #[serde(rename = "angleOfArrival")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle_of_arrival: Option<AngleOfArrival>,
    /// The estimated distance of the detection
    #[serde(rename = "rangeEstimateM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_estimate_m: Option<Measurement>,
    /// The maximum distance of the detection
    #[serde(rename = "maxRangeM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_range_m: Option<Measurement>,
}

impl LineOfBearing {
    pub fn builder() -> LineOfBearingBuilder {
        <LineOfBearingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LineOfBearingBuilder {
    angle_of_arrival: Option<AngleOfArrival>,
    range_estimate_m: Option<Measurement>,
    max_range_m: Option<Measurement>,
}

impl LineOfBearingBuilder {
    pub fn angle_of_arrival(mut self, value: AngleOfArrival) -> Self {
        self.angle_of_arrival = Some(value);
        self
    }

    pub fn range_estimate_m(mut self, value: Measurement) -> Self {
        self.range_estimate_m = Some(value);
        self
    }

    pub fn max_range_m(mut self, value: Measurement) -> Self {
        self.max_range_m = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LineOfBearing`].
    pub fn build(self) -> Result<LineOfBearing, BuildError> {
        Ok(LineOfBearing {
            angle_of_arrival: self.angle_of_arrival,
            range_estimate_m: self.range_estimate_m,
            max_range_m: self.max_range_m,
        })
    }
}
