pub use crate::prelude::*;

/// Available for Entities that have a single or primary Location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Location {
    /// see Position definition for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// Velocity in an ENU reference frame centered on the corresponding position. All units are meters per second.
    #[serde(rename = "velocityEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_enu: Option<Enu>,
    /// Speed is the magnitude of velocity_enu vector [sqrt(e^2 + n^2 + u^2)] when present, measured in m/s.
    #[serde(rename = "speedMps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed_mps: Option<f64>,
    /// The entity's acceleration in meters/s^2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration: Option<Enu>,
    /// quaternion to translate from entity body frame to it's ENU frame
    #[serde(rename = "attitudeEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attitude_enu: Option<Quaternion>,
}

impl Location {
    pub fn builder() -> LocationBuilder {
        <LocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocationBuilder {
    position: Option<Position>,
    velocity_enu: Option<Enu>,
    speed_mps: Option<f64>,
    acceleration: Option<Enu>,
    attitude_enu: Option<Quaternion>,
}

impl LocationBuilder {
    pub fn position(mut self, value: Position) -> Self {
        self.position = Some(value);
        self
    }

    pub fn velocity_enu(mut self, value: Enu) -> Self {
        self.velocity_enu = Some(value);
        self
    }

    pub fn speed_mps(mut self, value: f64) -> Self {
        self.speed_mps = Some(value);
        self
    }

    pub fn acceleration(mut self, value: Enu) -> Self {
        self.acceleration = Some(value);
        self
    }

    pub fn attitude_enu(mut self, value: Quaternion) -> Self {
        self.attitude_enu = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Location`].
    pub fn build(self) -> Result<Location, BuildError> {
        Ok(Location {
            position: self.position,
            velocity_enu: self.velocity_enu,
            speed_mps: self.speed_mps,
            acceleration: self.acceleration,
            attitude_enu: self.attitude_enu,
        })
    }
}
