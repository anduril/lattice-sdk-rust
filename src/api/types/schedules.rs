pub use crate::prelude::*;

/// Schedules associated with this entity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Schedules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
}

impl Schedules {
    pub fn builder() -> SchedulesBuilder {
        <SchedulesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulesBuilder {
    schedules: Option<Vec<Schedule>>,
}

impl SchedulesBuilder {
    pub fn schedules(mut self, value: Vec<Schedule>) -> Self {
        self.schedules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Schedules`].
    pub fn build(self) -> Result<Schedules, BuildError> {
        Ok(Schedules {
            schedules: self.schedules,
        })
    }
}
