pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KinematicsGeodetic {
    /// The location of this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationGeodetic>,
    /// Location uncertainty of this measurement, measured in the ENU frame. When there are multiple altitude
    /// measurements, this represents the most certain.
    #[serde(rename = "locationUncertaintyEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uncertainty_enu: Option<TMat3>,
    /// Velocity in the ENU frame, measured in meters per second.
    #[serde(rename = "velocityEnuMPerS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_enu_m_per_s: Option<Vec3>,
    /// A 3x3 covariance matrix representing the uncertainty of the velocity measurement.
    #[serde(rename = "velocityUncertaintyEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_uncertainty_enu: Option<TMat3>,
    /// The entity's acceleration in meters per second squared.
    #[serde(rename = "accelerationMPerS2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_m_per_s2: Option<Vec3>,
    /// Quaternion that rotates the X unit vector in the entity's body frame (assumed to be front-left-up) [1,0,0]
    /// to the entity's orientation unit vector in the ENU frame at the entity's location.
    #[serde(rename = "attitudeEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attitude_enu: Option<Quaternion>,
    /// The time when these kinematics were measured by the sensor. For tracks, this represents when the sensor made
    /// the observation that produced these kinematics. For asset pose data, this represents the system time when the
    /// pose was captured.
    #[serde(rename = "measurementTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub measurement_time: Option<DateTime<FixedOffset>>,
}

impl KinematicsGeodetic {
    pub fn builder() -> KinematicsGeodeticBuilder {
        <KinematicsGeodeticBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KinematicsGeodeticBuilder {
    location: Option<LocationGeodetic>,
    location_uncertainty_enu: Option<TMat3>,
    velocity_enu_m_per_s: Option<Vec3>,
    velocity_uncertainty_enu: Option<TMat3>,
    acceleration_m_per_s2: Option<Vec3>,
    attitude_enu: Option<Quaternion>,
    measurement_time: Option<DateTime<FixedOffset>>,
}

impl KinematicsGeodeticBuilder {
    pub fn location(mut self, value: LocationGeodetic) -> Self {
        self.location = Some(value);
        self
    }

    pub fn location_uncertainty_enu(mut self, value: TMat3) -> Self {
        self.location_uncertainty_enu = Some(value);
        self
    }

    pub fn velocity_enu_m_per_s(mut self, value: Vec3) -> Self {
        self.velocity_enu_m_per_s = Some(value);
        self
    }

    pub fn velocity_uncertainty_enu(mut self, value: TMat3) -> Self {
        self.velocity_uncertainty_enu = Some(value);
        self
    }

    pub fn acceleration_m_per_s2(mut self, value: Vec3) -> Self {
        self.acceleration_m_per_s2 = Some(value);
        self
    }

    pub fn attitude_enu(mut self, value: Quaternion) -> Self {
        self.attitude_enu = Some(value);
        self
    }

    pub fn measurement_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.measurement_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KinematicsGeodetic`].
    pub fn build(self) -> Result<KinematicsGeodetic, BuildError> {
        Ok(KinematicsGeodetic {
            location: self.location,
            location_uncertainty_enu: self.location_uncertainty_enu,
            velocity_enu_m_per_s: self.velocity_enu_m_per_s,
            velocity_uncertainty_enu: self.velocity_uncertainty_enu,
            acceleration_m_per_s2: self.acceleration_m_per_s2,
            attitude_enu: self.attitude_enu,
            measurement_time: self.measurement_time,
        })
    }
}
