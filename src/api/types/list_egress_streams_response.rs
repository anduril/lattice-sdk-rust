pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEgressStreamsResponse {
    /// The egress streams on this page. Up to `pageSize` entries
    /// (defaults to 50, capped at 100). Ordered by egress stream create time.
    #[serde(rename = "egressStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_streams: Option<Vec<EgressStream>>,
    /// Pass this back as `pageToken` to retrieve the next page.
    /// Empty when there are no more pages.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListEgressStreamsResponse {
    pub fn builder() -> ListEgressStreamsResponseBuilder {
        <ListEgressStreamsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEgressStreamsResponseBuilder {
    egress_streams: Option<Vec<EgressStream>>,
    next_page_token: Option<String>,
}

impl ListEgressStreamsResponseBuilder {
    pub fn egress_streams(mut self, value: Vec<EgressStream>) -> Self {
        self.egress_streams = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEgressStreamsResponse`].
    pub fn build(self) -> Result<ListEgressStreamsResponse, BuildError> {
        Ok(ListEgressStreamsResponse {
            egress_streams: self.egress_streams,
            next_page_token: self.next_page_token,
        })
    }
}
