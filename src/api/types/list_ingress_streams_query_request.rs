pub use crate::prelude::*;

/// Query parameters for listIngressStreams
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIngressStreamsQueryRequest {
    /// Desired number of ingress streams per page. Defaults to 50 if left blank,
    /// and capped at 100. The response may contain fewer than requested.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// To retrieve the next page, pass the `nextPageToken` from the previous
    /// response. Leave empty for the first page.
    ///
    /// Keep the rest of the request identical between pages, otherwise the
    /// server may reject it.
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListIngressStreamsQueryRequest {
    pub fn builder() -> ListIngressStreamsQueryRequestBuilder {
        <ListIngressStreamsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIngressStreamsQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl ListIngressStreamsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListIngressStreamsQueryRequest`].
    pub fn build(self) -> Result<ListIngressStreamsQueryRequest, BuildError> {
        Ok(ListIngressStreamsQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
        })
    }
}
