pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEgressStreamResponse {
    /// Service-generated identifier for the new egress stream. Use it for subsequent
    /// `GetEgressStream` and `DeleteEgressStream` calls.
    #[serde(rename = "egressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<RtspEgress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtEgress>,
}

impl CreateEgressStreamResponse {
    pub fn builder() -> CreateEgressStreamResponseBuilder {
        <CreateEgressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEgressStreamResponseBuilder {
    egress_id: Option<String>,
    rtsp: Option<RtspEgress>,
    srt: Option<SrtEgress>,
}

impl CreateEgressStreamResponseBuilder {
    pub fn egress_id(mut self, value: impl Into<String>) -> Self {
        self.egress_id = Some(value.into());
        self
    }

    pub fn rtsp(mut self, value: RtspEgress) -> Self {
        self.rtsp = Some(value);
        self
    }

    pub fn srt(mut self, value: SrtEgress) -> Self {
        self.srt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEgressStreamResponse`].
    pub fn build(self) -> Result<CreateEgressStreamResponse, BuildError> {
        Ok(CreateEgressStreamResponse {
            egress_id: self.egress_id,
            rtsp: self.rtsp,
            srt: self.srt,
        })
    }
}
