pub use crate::prelude::*;

/// Camera zoom state for a camera sensor. Unset means the entity does not report zoom.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CameraConfiguration {
    /// Percentage of the camera's zoom range, 0-100. 0 is fully zoomed out, 100 fully zoomed in.
    #[serde(rename = "zoomPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub zoom_percentage: Option<f64>,
}

impl CameraConfiguration {
    pub fn builder() -> CameraConfigurationBuilder {
        <CameraConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CameraConfigurationBuilder {
    zoom_percentage: Option<f64>,
}

impl CameraConfigurationBuilder {
    pub fn zoom_percentage(mut self, value: f64) -> Self {
        self.zoom_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CameraConfiguration`].
    pub fn build(self) -> Result<CameraConfiguration, BuildError> {
        Ok(CameraConfiguration {
            zoom_percentage: self.zoom_percentage,
        })
    }
}
