pub use crate::prelude::*;

/// List of sensors available for an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Sensors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensors: Option<Vec<Sensor>>,
}

impl Sensors {
    pub fn builder() -> SensorsBuilder {
        <SensorsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SensorsBuilder {
    sensors: Option<Vec<Sensor>>,
}

impl SensorsBuilder {
    pub fn sensors(mut self, value: Vec<Sensor>) -> Self {
        self.sensors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Sensors`].
    pub fn build(self) -> Result<Sensors, BuildError> {
        Ok(Sensors {
            sensors: self.sensors,
        })
    }
}
