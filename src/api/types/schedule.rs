pub use crate::prelude::*;

/// A Schedule associated with this entity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Schedule {
    /// expression that represents this schedule's "ON" state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<CronWindow>>,
    /// A unique identifier for this schedule.
    #[serde(rename = "scheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// The schedule type
    #[serde(rename = "scheduleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<ScheduleScheduleType>,
}

impl Schedule {
    pub fn builder() -> ScheduleBuilder {
        <ScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleBuilder {
    windows: Option<Vec<CronWindow>>,
    schedule_id: Option<String>,
    schedule_type: Option<ScheduleScheduleType>,
}

impl ScheduleBuilder {
    pub fn windows(mut self, value: Vec<CronWindow>) -> Self {
        self.windows = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn schedule_type(mut self, value: ScheduleScheduleType) -> Self {
        self.schedule_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Schedule`].
    pub fn build(self) -> Result<Schedule, BuildError> {
        Ok(Schedule {
            windows: self.windows,
            schedule_id: self.schedule_id,
            schedule_type: self.schedule_type,
        })
    }
}
