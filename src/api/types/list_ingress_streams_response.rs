pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIngressStreamsResponse {
    /// The ingress streams on this page. Up to `pageSize` entries
    /// (defaults to 50, capped at 100). Ordered by ingress stream create time.
    #[serde(rename = "ingressStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_streams: Option<Vec<IngressStream>>,
    /// Pass this back as `pageToken` to retrieve the next page.
    /// Empty when there are no more pages.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListIngressStreamsResponse {
    pub fn builder() -> ListIngressStreamsResponseBuilder {
        <ListIngressStreamsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIngressStreamsResponseBuilder {
    ingress_streams: Option<Vec<IngressStream>>,
    next_page_token: Option<String>,
}

impl ListIngressStreamsResponseBuilder {
    pub fn ingress_streams(mut self, value: Vec<IngressStream>) -> Self {
        self.ingress_streams = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListIngressStreamsResponse`].
    pub fn build(self) -> Result<ListIngressStreamsResponse, BuildError> {
        Ok(ListIngressStreamsResponse {
            ingress_streams: self.ingress_streams,
            next_page_token: self.next_page_token,
        })
    }
}
