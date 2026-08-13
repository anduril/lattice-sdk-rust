pub use crate::prelude::*;

/// Describes the relationship between the entity being tracked ("tracked entity") and the entity that is
/// performing the tracking ("tracking entity").
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackedBy {
    /// Sensor details of the tracking entity's sensors that were active and tracking the tracked entity. This may be
    /// a subset of the total sensors available on the tracking entity.
    #[serde(rename = "activelyTrackingSensors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actively_tracking_sensors: Option<Sensors>,
    /// Latest time that any sensor in actively_tracking_sensors detected the tracked entity.
    #[serde(rename = "lastMeasurementTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_measurement_timestamp: Option<DateTime<FixedOffset>>,
}

impl TrackedBy {
    pub fn builder() -> TrackedByBuilder {
        <TrackedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackedByBuilder {
    actively_tracking_sensors: Option<Sensors>,
    last_measurement_timestamp: Option<DateTime<FixedOffset>>,
}

impl TrackedByBuilder {
    pub fn actively_tracking_sensors(mut self, value: Sensors) -> Self {
        self.actively_tracking_sensors = Some(value);
        self
    }

    pub fn last_measurement_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_measurement_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrackedBy`].
    pub fn build(self) -> Result<TrackedBy, BuildError> {
        Ok(TrackedBy {
            actively_tracking_sensors: self.actively_tracking_sensors,
            last_measurement_timestamp: self.last_measurement_timestamp,
        })
    }
}
