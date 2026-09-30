pub use crate::prelude::*;

/// Settings for RTSP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RtspSettings {
    /// The upstream RTSP URL the service should pull frames from. Must use
    /// the `rtsp://` scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RtspSettings {
    pub fn builder() -> RtspSettingsBuilder {
        <RtspSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RtspSettingsBuilder {
    url: Option<String>,
}

impl RtspSettingsBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RtspSettings`].
    pub fn build(self) -> Result<RtspSettings, BuildError> {
        Ok(RtspSettings { url: self.url })
    }
}
