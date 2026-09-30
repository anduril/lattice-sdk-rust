pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateIngressStreamResponse {
    /// Identifier of the newly created ingress stream. Echoes the caller-supplied
    /// `ingressId` if one was provided, otherwise a service-generated GUID.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<String>,
    /// Connection details for an MPEG-TS push. Only returned when the request selected
    /// `mpegTs` and MPEG-TS ingress is enabled for the deployment. MPEG-TS ingress is
    /// supported only at the edge, in closed networks; in a cloud environment reached over
    /// the public internet it may be disabled per deployment, in which case the create
    /// request is rejected and this field is never populated.
    #[serde(rename = "mpegTs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_ts: Option<MpegTsIngress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtIngress>,
}

impl CreateIngressStreamResponse {
    pub fn builder() -> CreateIngressStreamResponseBuilder {
        <CreateIngressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIngressStreamResponseBuilder {
    ingress_id: Option<String>,
    mpeg_ts: Option<MpegTsIngress>,
    srt: Option<SrtIngress>,
}

impl CreateIngressStreamResponseBuilder {
    pub fn ingress_id(mut self, value: impl Into<String>) -> Self {
        self.ingress_id = Some(value.into());
        self
    }

    pub fn mpeg_ts(mut self, value: MpegTsIngress) -> Self {
        self.mpeg_ts = Some(value);
        self
    }

    pub fn srt(mut self, value: SrtIngress) -> Self {
        self.srt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateIngressStreamResponse`].
    pub fn build(self) -> Result<CreateIngressStreamResponse, BuildError> {
        Ok(CreateIngressStreamResponse {
            ingress_id: self.ingress_id,
            mpeg_ts: self.mpeg_ts,
            srt: self.srt,
        })
    }
}
