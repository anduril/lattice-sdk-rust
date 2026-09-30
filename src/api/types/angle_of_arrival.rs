pub use crate::prelude::*;

/// The direction from which the signal is received
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AngleOfArrival {
    /// Origin (LLA) and attitude (relative to ENU) of a ray pointing towards the detection. The attitude represents a
    /// forward-left-up (FLU) frame where the x-axis (1, 0, 0) is pointing towards the target.
    #[serde(rename = "relativePose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_pose: Option<Pose>,
    /// Bearing/elevation covariance matrix where bearing is defined in radians CCW+ about the z-axis from the x-axis of FLU frame
    /// and elevation is positive down from the FL/XY plane.
    /// mxx = bearing variance in rad^2
    /// mxy = bearing/elevation covariance in rad^2
    /// myy = elevation variance in rad^2
    #[serde(rename = "bearingElevationCovarianceRad2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing_elevation_covariance_rad2: Option<TMat2>,
}

impl AngleOfArrival {
    pub fn builder() -> AngleOfArrivalBuilder {
        <AngleOfArrivalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AngleOfArrivalBuilder {
    relative_pose: Option<Pose>,
    bearing_elevation_covariance_rad2: Option<TMat2>,
}

impl AngleOfArrivalBuilder {
    pub fn relative_pose(mut self, value: Pose) -> Self {
        self.relative_pose = Some(value);
        self
    }

    pub fn bearing_elevation_covariance_rad2(mut self, value: TMat2) -> Self {
        self.bearing_elevation_covariance_rad2 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AngleOfArrival`].
    pub fn build(self) -> Result<AngleOfArrival, BuildError> {
        Ok(AngleOfArrival {
            relative_pose: self.relative_pose,
            bearing_elevation_covariance_rad2: self.bearing_elevation_covariance_rad2,
        })
    }
}
