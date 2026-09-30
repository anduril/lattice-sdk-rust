pub use crate::prelude::*;

/// RTSP ingress connection details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RtspIngress {
    /// The upstream RTSP URL. Lattice will pull from the supplied URL.
    /// The URL must be prefixed with `rtsp://`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RtspIngress {
    pub fn builder() -> RtspIngressBuilder {
        <RtspIngressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RtspIngressBuilder {
    url: Option<String>,
}

impl RtspIngressBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RtspIngress`].
    pub fn build(self) -> Result<RtspIngress, BuildError> {
        Ok(RtspIngress { url: self.url })
    }
}
