pub use crate::prelude::*;

/// Response for QueryIngressStreams.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIngressStreamsResponse {
    /// Up to `pageSize` streams, newest first. Streams created through this API come
    /// first, then streams discovered on the mesh, and each group starts with its
    /// newest stream.
    #[serde(rename = "ingressStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_streams: Option<Vec<IngressStream>>,
    /// Pass this back as `pageToken` to get the next page. Empty when there are no
    /// more pages.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl QueryIngressStreamsResponse {
    pub fn builder() -> QueryIngressStreamsResponseBuilder {
        <QueryIngressStreamsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIngressStreamsResponseBuilder {
    ingress_streams: Option<Vec<IngressStream>>,
    next_page_token: Option<String>,
}

impl QueryIngressStreamsResponseBuilder {
    pub fn ingress_streams(mut self, value: Vec<IngressStream>) -> Self {
        self.ingress_streams = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIngressStreamsResponse`].
    pub fn build(self) -> Result<QueryIngressStreamsResponse, BuildError> {
        Ok(QueryIngressStreamsResponse {
            ingress_streams: self.ingress_streams,
            next_page_token: self.next_page_token,
        })
    }
}
