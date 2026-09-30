pub use crate::prelude::*;

/// Uncertainty of entity position and velocity, if available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LocationUncertainty {
    /// Positional covariance represented by the upper triangle of the covariance matrix. It is valid to populate
    /// only the diagonal of the matrix if the full covariance matrix is unknown.
    #[serde(rename = "positionEnuCov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_enu_cov: Option<EntityManagerTMat3>,
    /// Velocity covariance represented by the upper triangle of the covariance matrix. It is valid to populate
    /// only the diagonal of the matrix if the full covariance matrix is unknown.
    #[serde(rename = "velocityEnuCov")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity_enu_cov: Option<EntityManagerTMat3>,
    /// An ellipse that describes the certainty probability and error boundary for a given geolocation.
    #[serde(rename = "positionErrorEllipse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_error_ellipse: Option<ErrorEllipse>,
}

impl LocationUncertainty {
    pub fn builder() -> LocationUncertaintyBuilder {
        <LocationUncertaintyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocationUncertaintyBuilder {
    position_enu_cov: Option<EntityManagerTMat3>,
    velocity_enu_cov: Option<EntityManagerTMat3>,
    position_error_ellipse: Option<ErrorEllipse>,
}

impl LocationUncertaintyBuilder {
    pub fn position_enu_cov(mut self, value: EntityManagerTMat3) -> Self {
        self.position_enu_cov = Some(value);
        self
    }

    pub fn velocity_enu_cov(mut self, value: EntityManagerTMat3) -> Self {
        self.velocity_enu_cov = Some(value);
        self
    }

    pub fn position_error_ellipse(mut self, value: ErrorEllipse) -> Self {
        self.position_error_ellipse = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocationUncertainty`].
    pub fn build(self) -> Result<LocationUncertainty, BuildError> {
        Ok(LocationUncertainty {
            position_enu_cov: self.position_enu_cov,
            velocity_enu_cov: self.velocity_enu_cov,
            position_error_ellipse: self.position_error_ellipse,
        })
    }
}
