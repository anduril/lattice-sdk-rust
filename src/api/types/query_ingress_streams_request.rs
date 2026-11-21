pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIngressStreamsRequest {
    /// How many streams to return per page. Defaults to 50, and values above 100
    /// count as 100. A page can contain fewer streams than you asked for.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// To get the next page, pass the `nextPageToken` from the previous response.
    /// Leave it empty for the first page. Send the same filter on every page: the
    /// server rejects a token if the filter changed, or if the token came from
    /// ListIngressStreams.
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Which streams to return. Leave it out to return every stream you can see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<IngressStreamFilter>,
}

impl QueryIngressStreamsRequest {
    pub fn builder() -> QueryIngressStreamsRequestBuilder {
        <QueryIngressStreamsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIngressStreamsRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    filter: Option<IngressStreamFilter>,
}

impl QueryIngressStreamsRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn filter(mut self, value: IngressStreamFilter) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIngressStreamsRequest`].
    pub fn build(self) -> Result<QueryIngressStreamsRequest, BuildError> {
        Ok(QueryIngressStreamsRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            filter: self.filter,
        })
    }
}
