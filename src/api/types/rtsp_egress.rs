pub use crate::prelude::*;

/// RTSP egress connection details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RtspEgress {
    /// The RTSP URL the downstream consumer should pull from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl RtspEgress {
    pub fn builder() -> RtspEgressBuilder {
        <RtspEgressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RtspEgressBuilder {
    url: Option<String>,
}

impl RtspEgressBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RtspEgress`].
    pub fn build(self) -> Result<RtspEgress, BuildError> {
        Ok(RtspEgress { url: self.url })
    }
}
