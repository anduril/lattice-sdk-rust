pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KinematicsGeocentric {
    /// The location of the entity, measured in the ECEF reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationGeocentricEcef>,
    /// Location uncertainty of this measurement, measured in the ECEF frame.
    #[serde(rename = "locationUncertaintyEcef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uncertainty_ecef: Option<TMat3>,
    /// Velocity in the ECEF frame, measured in meters per second.
    #[serde(rename = "velocityEcefMPerS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_ecef_m_per_s: Option<Vec3>,
    /// A 3x3 covariance matrix representing the uncertainty of the velocity measurement.
    #[serde(rename = "velocityUncertaintyEcef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_uncertainty_ecef: Option<TMat3>,
    /// The entity's acceleration in meters per second squared.
    #[serde(rename = "accelerationMPerS2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_m_per_s2: Option<Vec3>,
    /// Quaternion that rotates the X unit vector in the entity's body frame (assumed to be front-left-up) [1,0,0]
    /// to the entity's orientation unit vector in the ECEF frame at the entity's location.
    #[serde(rename = "attitudeEcef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attitude_ecef: Option<Quaternion>,
    /// The time when these kinematics were measured by the sensor. For tracks, this represents when the sensor made
    /// the observation that produced these kinematics. For asset pose data, this represents the system time when the
    /// pose was captured.
    #[serde(rename = "measurementTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub measurement_time: Option<DateTime<FixedOffset>>,
}

impl KinematicsGeocentric {
    pub fn builder() -> KinematicsGeocentricBuilder {
        <KinematicsGeocentricBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KinematicsGeocentricBuilder {
    location: Option<LocationGeocentricEcef>,
    location_uncertainty_ecef: Option<TMat3>,
    velocity_ecef_m_per_s: Option<Vec3>,
    velocity_uncertainty_ecef: Option<TMat3>,
    acceleration_m_per_s2: Option<Vec3>,
    attitude_ecef: Option<Quaternion>,
    measurement_time: Option<DateTime<FixedOffset>>,
}

impl KinematicsGeocentricBuilder {
    pub fn location(mut self, value: LocationGeocentricEcef) -> Self {
        self.location = Some(value);
        self
    }

    pub fn location_uncertainty_ecef(mut self, value: TMat3) -> Self {
        self.location_uncertainty_ecef = Some(value);
        self
    }

    pub fn velocity_ecef_m_per_s(mut self, value: Vec3) -> Self {
        self.velocity_ecef_m_per_s = Some(value);
        self
    }

    pub fn velocity_uncertainty_ecef(mut self, value: TMat3) -> Self {
        self.velocity_uncertainty_ecef = Some(value);
        self
    }

    pub fn acceleration_m_per_s2(mut self, value: Vec3) -> Self {
        self.acceleration_m_per_s2 = Some(value);
        self
    }

    pub fn attitude_ecef(mut self, value: Quaternion) -> Self {
        self.attitude_ecef = Some(value);
        self
    }

    pub fn measurement_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.measurement_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KinematicsGeocentric`].
    pub fn build(self) -> Result<KinematicsGeocentric, BuildError> {
        Ok(KinematicsGeocentric {
            location: self.location,
            location_uncertainty_ecef: self.location_uncertainty_ecef,
            velocity_ecef_m_per_s: self.velocity_ecef_m_per_s,
            velocity_uncertainty_ecef: self.velocity_uncertainty_ecef,
            acceleration_m_per_s2: self.acceleration_m_per_s2,
            attitude_ecef: self.attitude_ecef,
            measurement_time: self.measurement_time,
        })
    }
}
