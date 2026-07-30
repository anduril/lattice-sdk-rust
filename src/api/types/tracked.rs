pub use crate::prelude::*;

/// Available for Entities that are tracked.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tracked {
    /// Quality score, 0-15, nil if none
    #[serde(rename = "trackQualityWrapper")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_quality_wrapper: Option<i64>,
    /// Sensor hits aggregation on the tracked entity.
    #[serde(rename = "sensorHits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_hits: Option<i64>,
    /// Estimated number of objects or units that are represented by this entity. Known as Strength in certain contexts (Link16)
    /// if UpperBound == LowerBound; (strength = LowerBound)
    /// If both UpperBound and LowerBound are defined; strength is between LowerBound and UpperBound (represented as string "Strength: 4-5")
    /// If UpperBound is defined only (LowerBound unset), Strength ≤ UpperBound
    /// If LowerBound is defined only (UpperBound unset), LowerBound ≤ Strength
    /// 0 indicates unset.
    #[serde(rename = "numberOfObjects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_objects: Option<UInt32Range>,
    /// The radar cross section (RCS) is a measure of how detectable an object is by radar. A large RCS indicates an object is more easily
    /// detected. The unit is “decibels per square meter,” or dBsm
    #[serde(rename = "radarCrossSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub radar_cross_section: Option<f64>,
    /// Timestamp of the latest tracking measurement for this entity.
    #[serde(rename = "lastMeasurementTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_measurement_time: Option<DateTime<FixedOffset>>,
    /// The relative position of a track with respect to the entity that is tracking it. Used for tracks that do not yet have a 3D position.
    /// For this entity (A), being tracked by some entity (B), this LineOfBearing would express a ray from B to A.
    #[serde(rename = "lineOfBearing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_bearing: Option<LineOfBearing>,
}

impl Tracked {
    pub fn builder() -> TrackedBuilder {
        <TrackedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackedBuilder {
    track_quality_wrapper: Option<i64>,
    sensor_hits: Option<i64>,
    number_of_objects: Option<UInt32Range>,
    radar_cross_section: Option<f64>,
    last_measurement_time: Option<DateTime<FixedOffset>>,
    line_of_bearing: Option<LineOfBearing>,
}

impl TrackedBuilder {
    pub fn track_quality_wrapper(mut self, value: i64) -> Self {
        self.track_quality_wrapper = Some(value);
        self
    }

    pub fn sensor_hits(mut self, value: i64) -> Self {
        self.sensor_hits = Some(value);
        self
    }

    pub fn number_of_objects(mut self, value: UInt32Range) -> Self {
        self.number_of_objects = Some(value);
        self
    }

    pub fn radar_cross_section(mut self, value: f64) -> Self {
        self.radar_cross_section = Some(value);
        self
    }

    pub fn last_measurement_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_measurement_time = Some(value);
        self
    }

    pub fn line_of_bearing(mut self, value: LineOfBearing) -> Self {
        self.line_of_bearing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tracked`].
    pub fn build(self) -> Result<Tracked, BuildError> {
        Ok(Tracked {
            track_quality_wrapper: self.track_quality_wrapper,
            sensor_hits: self.sensor_hits,
            number_of_objects: self.number_of_objects,
            radar_cross_section: self.radar_cross_section,
            last_measurement_time: self.last_measurement_time,
            line_of_bearing: self.line_of_bearing,
        })
    }
}
