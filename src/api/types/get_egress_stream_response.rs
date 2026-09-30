pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEgressStreamResponse {
    /// The egress stream corresponding to the requested `egressId`.
    #[serde(rename = "egressStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_stream: Option<EgressStream>,
}

impl GetEgressStreamResponse {
    pub fn builder() -> GetEgressStreamResponseBuilder {
        <GetEgressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEgressStreamResponseBuilder {
    egress_stream: Option<EgressStream>,
}

impl GetEgressStreamResponseBuilder {
    pub fn egress_stream(mut self, value: EgressStream) -> Self {
        self.egress_stream = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEgressStreamResponse`].
    pub fn build(self) -> Result<GetEgressStreamResponse, BuildError> {
        Ok(GetEgressStreamResponse {
            egress_stream: self.egress_stream,
        })
    }
}
