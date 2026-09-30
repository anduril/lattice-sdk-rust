pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateIngressStreamRequest {
    /// Caller-supplied identifier for the new stream. If omitted, the service generates a GUID.
    /// If supplied, a consistent and recognizable pattern is recommended. A common convention
    /// is a group prefix (organization, platform, or asset) followed by a specific identifier
    /// using underscore or dot as a separator, for example, `drone_1`, `vessel_2`, or
    /// `teamalpha.drone1`.
    ///
    /// When supplied, an ingressId must be between 4 and 36 characters long and use only
    /// printable ASCII characters with no spaces; the 36-character ceiling leaves room for a
    /// full GUID. A value outside that length range, or one containing spaces, control
    /// characters, or non-ASCII characters, is rejected, as is an ingressId that another
    /// ingress stream is already using.
    #[serde(rename = "ingressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_id: Option<String>,
    /// Human-readable title for the stream. A title is required: surrounding whitespace is
    /// trimmed before it is stored, and what remains must be non-empty, valid UTF-8, and no
    /// longer than 64 characters. Otherwise the request is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Receive an MPEG-TS push from the producer. The service allocates a UDP port and
    /// returns the URL the producer must push to in CreateIngressStreamResponse.
    ///
    /// MPEG-TS ingress is supported only at the edge, in closed networks. When Lattice
    /// runs in a cloud environment reached over the public internet, MPEG-TS ingress may
    /// be disabled per deployment. When it is disabled, a request that selects `mpegTs` is
    /// rejected with a gRPC error rather than accepted, so callers should be prepared to
    /// fall back to RTSP or SRT. An MPEG-TS stream created at the edge can still be listed
    /// and inspected on the IngressStream read model even when cloud ingress is disabled.
    #[serde(rename = "mpegTs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_ts: Option<MpegTsSettings>,
    /// Pull from a caller-supplied RTSP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtsp: Option<RtspSettings>,
    /// Receive an SRT push from the producer. The service returns a URL and sessionId
    /// in CreateIngressStreamResponse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srt: Option<SrtSettings>,
}

impl CreateIngressStreamRequest {
    pub fn builder() -> CreateIngressStreamRequestBuilder {
        <CreateIngressStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIngressStreamRequestBuilder {
    ingress_id: Option<String>,
    title: Option<String>,
    mpeg_ts: Option<MpegTsSettings>,
    rtsp: Option<RtspSettings>,
    srt: Option<SrtSettings>,
}

impl CreateIngressStreamRequestBuilder {
    pub fn ingress_id(mut self, value: impl Into<String>) -> Self {
        self.ingress_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn mpeg_ts(mut self, value: MpegTsSettings) -> Self {
        self.mpeg_ts = Some(value);
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

    /// Consumes the builder and constructs a [`CreateIngressStreamRequest`].
    pub fn build(self) -> Result<CreateIngressStreamRequest, BuildError> {
        Ok(CreateIngressStreamRequest {
            ingress_id: self.ingress_id,
            title: self.title,
            mpeg_ts: self.mpeg_ts,
            rtsp: self.rtsp,
            srt: self.srt,
        })
    }
}
