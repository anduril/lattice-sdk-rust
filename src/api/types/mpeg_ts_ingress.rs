pub use crate::prelude::*;

/// MPEG-TS ingress connection details.
///
/// MPEG-TS ingress is supported only at the edge, in closed networks; in a cloud
/// environment reached over the public internet it may be disabled per deployment. These
/// details are populated only when a stream was successfully created with `mpegTs`. An
/// MPEG-TS stream created at the edge can still be listed and inspected on the
/// IngressStream read model even when cloud ingress is disabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MpegTsIngress {
    /// The URL that the producer should push the MPEG-TS stream to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MpegTsIngress {
    pub fn builder() -> MpegTsIngressBuilder {
        <MpegTsIngressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MpegTsIngressBuilder {
    url: Option<String>,
}

impl MpegTsIngressBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MpegTsIngress`].
    pub fn build(self) -> Result<MpegTsIngress, BuildError> {
        Ok(MpegTsIngress { url: self.url })
    }
}
