pub use crate::prelude::*;

/// Kinematics of the entity, including its location, location uncertainty, motion, attitude, and the time the
/// kinematics were measured.
///
/// Only one of the fields on this message is expected to be set when publishing an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Kinematics {
    /// Kinematics measured in a geodetic (WGS84 latitude/longitude/altitude and ENU) reference frame.
    #[serde(rename = "kinematicsGeodetic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinematics_geodetic: Option<KinematicsGeodetic>,
    /// Kinematics measured in a geocentric (ECEF) reference frame.
    #[serde(rename = "kinematicsGeocentric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinematics_geocentric: Option<KinematicsGeocentric>,
}

impl Kinematics {
    pub fn builder() -> KinematicsBuilder {
        <KinematicsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KinematicsBuilder {
    kinematics_geodetic: Option<KinematicsGeodetic>,
    kinematics_geocentric: Option<KinematicsGeocentric>,
}

impl KinematicsBuilder {
    pub fn kinematics_geodetic(mut self, value: KinematicsGeodetic) -> Self {
        self.kinematics_geodetic = Some(value);
        self
    }

    pub fn kinematics_geocentric(mut self, value: KinematicsGeocentric) -> Self {
        self.kinematics_geocentric = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Kinematics`].
    pub fn build(self) -> Result<Kinematics, BuildError> {
        Ok(Kinematics {
            kinematics_geodetic: self.kinematics_geodetic,
            kinematics_geocentric: self.kinematics_geocentric,
        })
    }
}
