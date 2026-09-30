pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEgressStreamRequest {
    /// Identifier of the live ingress stream to re-publish as an egress stream.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<RtspSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtSettings>,
}

impl CreateEgressStreamRequest {
    pub fn builder() -> CreateEgressStreamRequestBuilder {
        <CreateEgressStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEgressStreamRequestBuilder {
    ingress_id: Option<String>,
    rtsp: Option<RtspSettings>,
    srt: Option<SrtSettings>,
}

impl CreateEgressStreamRequestBuilder {
    pub fn ingress_id(mut self, value: impl Into<String>) -> Self {
        self.ingress_id = Some(value.into());
        self
    }

    pub fn rtsp(mut self, value: RtspSettings) -> Self {
        self.rtsp = Some(value);
        self
    }

    pub fn srt(mut self, value: SrtSettings) -> Self {
        self.srt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEgressStreamRequest`].
    pub fn build(self) -> Result<CreateEgressStreamRequest, BuildError> {
        Ok(CreateEgressStreamRequest {
            ingress_id: self.ingress_id,
            rtsp: self.rtsp,
            srt: self.srt,
        })
    }
}
