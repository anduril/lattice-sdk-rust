pub use crate::prelude::*;

/// Sensor Field Of View closely resembling fov.proto SensorFieldOfView.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FieldOfView {
    /// The Id for one instance of a FieldOfView, persisted across multiple updates to provide continuity during
    /// smoothing. This is relevant for sensors where the dwell schedule is on the order of
    /// milliseconds, making multiple FOVs a requirement for proper display of search beams.
    #[serde(rename = "fovId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov_id: Option<i64>,
    /// The Id of the mount the sensor is on.
    #[serde(rename = "mountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_id: Option<String>,
    /// The field of view the sensor projected onto the ground.
    #[serde(rename = "projectedFrustum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_frustum: Option<ProjectedFrustum>,
    /// Center ray of the frustum projected onto the ground.
    #[serde(rename = "projectedCenterRay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_center_ray: Option<Position>,
    /// The origin and direction of the center ray for this sensor relative to the ENU frame. A ray which is aligned with
    /// the positive X axis in the sensor frame will be transformed into the ray along the sensor direction in the ENU
    /// frame when transformed by the quaternion contained in this pose.
    #[serde(rename = "centerRayPose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center_ray_pose: Option<EntityManagerPose>,
    /// Horizontal field of view in radians.
    #[serde(rename = "horizontalFov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub horizontal_fov: Option<f64>,
    /// Vertical field of view in radians.
    #[serde(rename = "verticalFov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vertical_fov: Option<f64>,
    /// Sensor range in meters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub range: Option<f64>,
    /// The mode that this sensor is currently in, used to display for context in the UI. Some sensors can emit multiple
    /// sensor field of views with different modes, for example a radar can simultaneously search broadly and perform
    /// tighter bounded tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FieldOfViewMode>,
}

impl FieldOfView {
    pub fn builder() -> FieldOfViewBuilder {
        <FieldOfViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldOfViewBuilder {
    fov_id: Option<i64>,
    mount_id: Option<String>,
    projected_frustum: Option<ProjectedFrustum>,
    projected_center_ray: Option<Position>,
    center_ray_pose: Option<EntityManagerPose>,
    horizontal_fov: Option<f64>,
    vertical_fov: Option<f64>,
    range: Option<f64>,
    mode: Option<FieldOfViewMode>,
}

impl FieldOfViewBuilder {
    pub fn fov_id(mut self, value: i64) -> Self {
        self.fov_id = Some(value);
        self
    }

    pub fn mount_id(mut self, value: impl Into<String>) -> Self {
        self.mount_id = Some(value.into());
        self
    }

    pub fn projected_frustum(mut self, value: ProjectedFrustum) -> Self {
        self.projected_frustum = Some(value);
        self
    }

    pub fn projected_center_ray(mut self, value: Position) -> Self {
        self.projected_center_ray = Some(value);
        self
    }

    pub fn center_ray_pose(mut self, value: EntityManagerPose) -> Self {
        self.center_ray_pose = Some(value);
        self
    }

    pub fn horizontal_fov(mut self, value: f64) -> Self {
        self.horizontal_fov = Some(value);
        self
    }

    pub fn vertical_fov(mut self, value: f64) -> Self {
        self.vertical_fov = Some(value);
        self
    }

    pub fn range(mut self, value: f64) -> Self {
        self.range = Some(value);
        self
    }

    pub fn mode(mut self, value: FieldOfViewMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FieldOfView`].
    pub fn build(self) -> Result<FieldOfView, BuildError> {
        Ok(FieldOfView {
            fov_id: self.fov_id,
            mount_id: self.mount_id,
            projected_frustum: self.projected_frustum,
            projected_center_ray: self.projected_center_ray,
            center_ray_pose: self.center_ray_pose,
            horizontal_fov: self.horizontal_fov,
            vertical_fov: self.vertical_fov,
            range: self.range,
            mode: self.mode,
        })
    }
}
