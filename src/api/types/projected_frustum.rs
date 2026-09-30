pub use crate::prelude::*;

/// Represents a frustum in which which all four corner points project onto the ground. All points in this message
/// are optional, if the projection to the ground fails then they will not be populated.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectedFrustum {
    /// Upper left point of the frustum.
    #[serde(rename = "upperLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_left: Option<Position>,
    /// Upper right point of the frustum.
    #[serde(rename = "upperRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_right: Option<Position>,
    /// Bottom right point of the frustum.
    #[serde(rename = "bottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<Position>,
    /// Bottom left point of the frustum.
    #[serde(rename = "bottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<Position>,
}

impl ProjectedFrustum {
    pub fn builder() -> ProjectedFrustumBuilder {
        <ProjectedFrustumBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectedFrustumBuilder {
    upper_left: Option<Position>,
    upper_right: Option<Position>,
    bottom_right: Option<Position>,
    bottom_left: Option<Position>,
}

impl ProjectedFrustumBuilder {
    pub fn upper_left(mut self, value: Position) -> Self {
        self.upper_left = Some(value);
        self
    }

    pub fn upper_right(mut self, value: Position) -> Self {
        self.upper_right = Some(value);
        self
    }

    pub fn bottom_right(mut self, value: Position) -> Self {
        self.bottom_right = Some(value);
        self
    }

    pub fn bottom_left(mut self, value: Position) -> Self {
        self.bottom_left = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectedFrustum`].
    pub fn build(self) -> Result<ProjectedFrustum, BuildError> {
        Ok(ProjectedFrustum {
            upper_left: self.upper_left,
            upper_right: self.upper_right,
            bottom_right: self.bottom_right,
            bottom_left: self.bottom_left,
        })
    }
}
