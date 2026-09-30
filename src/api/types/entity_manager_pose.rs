pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityManagerPose {
    /// Geospatial location defined by this Pose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<Position>,
    /// The quaternion to transform a point in the Pose frame to the ENU frame. The Pose frame could be Body, Turret,
    /// etc and is determined by the context in which this Pose is used.
    /// The normal convention for defining orientation is to list the frames of transformation, for example
    /// att_gimbal_to_enu is the quaternion which transforms a point in the gimbal frame to the body frame, but
    /// in this case we truncate to att_enu because the Pose frame isn't defined. A potentially better name for this
    /// field would have been att_pose_to_enu.
    ///
    /// Implementations of this quaternion should left multiply this quaternion to transform a point from the Pose frame
    /// to the enu frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Quaternion>,
}

impl EntityManagerPose {
    pub fn builder() -> EntityManagerPoseBuilder {
        <EntityManagerPoseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityManagerPoseBuilder {
    pos: Option<Position>,
    orientation: Option<Quaternion>,
}

impl EntityManagerPoseBuilder {
    pub fn pos(mut self, value: Position) -> Self {
        self.pos = Some(value);
        self
    }

    pub fn orientation(mut self, value: Quaternion) -> Self {
        self.orientation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityManagerPose`].
    pub fn build(self) -> Result<EntityManagerPose, BuildError> {
        Ok(EntityManagerPose {
            pos: self.pos,
            orientation: self.orientation,
        })
    }
}
