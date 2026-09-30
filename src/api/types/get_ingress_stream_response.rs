pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIngressStreamResponse {
    /// The ingress stream corresponding to the requested `ingressId`.
    #[serde(rename = "ingressStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_stream: Option<IngressStream>,
}

impl GetIngressStreamResponse {
    pub fn builder() -> GetIngressStreamResponseBuilder {
        <GetIngressStreamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIngressStreamResponseBuilder {
    ingress_stream: Option<IngressStream>,
}

impl GetIngressStreamResponseBuilder {
    pub fn ingress_stream(mut self, value: IngressStream) -> Self {
        self.ingress_stream = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetIngressStreamResponse`].
    pub fn build(self) -> Result<GetIngressStreamResponse, BuildError> {
        Ok(GetIngressStreamResponse {
            ingress_stream: self.ingress_stream,
        })
    }
}
