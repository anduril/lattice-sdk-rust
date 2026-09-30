pub use crate::prelude::*;

/// An egress stream publishes a single stream to a downstream consumer over a chosen
/// transport.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EgressStream {
    /// Service-generated identifier for the egress stream.
    #[serde(rename = "egressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_id: Option<String>,
    /// Identifier of the ingress stream this egress stream publishes.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<RtspEgress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtEgress>,
}

impl EgressStream {
    pub fn builder() -> EgressStreamBuilder {
        <EgressStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EgressStreamBuilder {
    egress_id: Option<String>,
    ingress_id: Option<String>,
    rtsp: Option<RtspEgress>,
    srt: Option<SrtEgress>,
}

impl EgressStreamBuilder {
    pub fn egress_id(mut self, value: impl Into<String>) -> Self {
        self.egress_id = Some(value.into());
        self
    }

    pub fn ingress_id(mut self, value: impl Into<String>) -> Self {
        self.ingress_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`EgressStream`].
    pub fn build(self) -> Result<EgressStream, BuildError> {
        Ok(EgressStream {
            egress_id: self.egress_id,
            ingress_id: self.ingress_id,
            rtsp: self.rtsp,
            srt: self.srt,
        })
    }
}
