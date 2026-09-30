pub use crate::prelude::*;

/// An ingress stream represents a single source feeding frames into Lattice.
/// Ingress streams are replicated across Lattice and visible anywhere in the deployment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngressStream {
    /// Unique identifier for the ingress stream.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<String>,
    /// Human-readable title supplied at creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Current lifecycle status of the stream. See StreamStatus for the full state machine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IngressStreamStatus>,
    #[serde(rename = "mpegTs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_ts: Option<MpegTsIngress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<RtspIngress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtIngress>,
    /// Wall-clock time the stream was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Wall-clock time the stream's status (STREAM_STATUS) was changed. The status can change based on the activity or
    /// the deletion of the stream.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// Identifiers of the egress streams currently consuming this ingress stream.
    #[serde(rename = "egressIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ids: Option<Vec<String>>,
}

impl IngressStream {
    pub fn builder() -> IngressStreamBuilder {
        <IngressStreamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngressStreamBuilder {
    ingress_id: Option<String>,
    title: Option<String>,
    status: Option<IngressStreamStatus>,
    mpeg_ts: Option<MpegTsIngress>,
    rtsp: Option<RtspIngress>,
    srt: Option<SrtIngress>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    egress_ids: Option<Vec<String>>,
}

impl IngressStreamBuilder {
    pub fn ingress_id(mut self, value: impl Into<String>) -> Self {
        self.ingress_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn status(mut self, value: IngressStreamStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn mpeg_ts(mut self, value: MpegTsIngress) -> Self {
        self.mpeg_ts = Some(value);
        self
    }

    pub fn rtsp(mut self, value: RtspIngress) -> Self {
        self.rtsp = Some(value);
        self
    }

    pub fn srt(mut self, value: SrtIngress) -> Self {
        self.srt = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn egress_ids(mut self, value: Vec<String>) -> Self {
        self.egress_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngressStream`].
    pub fn build(self) -> Result<IngressStream, BuildError> {
        Ok(IngressStream {
            ingress_id: self.ingress_id,
            title: self.title,
            status: self.status,
            mpeg_ts: self.mpeg_ts,
            rtsp: self.rtsp,
            srt: self.srt,
            created_at: self.created_at,
            updated_at: self.updated_at,
            egress_ids: self.egress_ids,
        })
    }
}
