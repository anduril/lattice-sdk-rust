pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pose {
    /// Geospatial location defined by this Pose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<Lla>,
    /// The quaternion to transform a point in the Pose frame to the ENU frame. The Pose frame could be Body, Turret,
    /// etc and is determined by the context in which this Pose is used.
    /// The normal convention for defining orientation is to list the frames of transformation, for example
    /// att_gimbal_to_enu is the quaternion which transforms a point in the gimbal frame to the body frame, but
    /// in this case we truncate to att_enu because the Pose frame isn't defined. A potentially better name for this
    /// field would have been att_pose_to_enu.
    ///
    /// Implementations of this quaternion should left multiply this quaternion to transform a point from the Pose frame
    /// to the enu frame.
    ///
    /// Point<Pose\> posePt{1,0,0};
    /// Rotation<Enu, Pose\> attPoseToEnu{};
    /// Point<Enu\> = attPoseToEnu*posePt;
    ///
    /// This transformed point represents some vector in ENU space that is aligned with the x axis of the attPoseToEnu
    /// matrix.
    ///
    /// An alternative matrix expression is as follows:
    /// ptEnu = M x ptPose
    #[serde(rename = "attEnu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub att_enu: Option<Quaternion>,
}

impl Pose {
    pub fn builder() -> PoseBuilder {
        <PoseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PoseBuilder {
    pos: Option<Lla>,
    att_enu: Option<Quaternion>,
}

impl PoseBuilder {
    pub fn pos(mut self, value: Lla) -> Self {
        self.pos = Some(value);
        self
    }

    pub fn att_enu(mut self, value: Quaternion) -> Self {
        self.att_enu = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pose`].
    pub fn build(self) -> Result<Pose, BuildError> {
        Ok(Pose {
            pos: self.pos,
            att_enu: self.att_enu,
        })
    }
}
