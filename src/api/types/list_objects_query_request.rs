pub use crate::prelude::*;

/// Query parameters for listObjects
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListObjectsQueryRequest {
    /// Filters the objects based on the specified prefix path. If no path is specified, all objects are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Sets the age for the oldest objects to query across the environment.
    #[serde(rename = "sinceTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_timestamp: Option<DateTime<FixedOffset>>,
    /// Base64 and URL-encoded cursor returned by the service to continue paging.
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Lists objects across all environment nodes in a Lattice Mesh.
    #[serde(rename = "allObjectsInMesh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_objects_in_mesh: Option<bool>,
    /// Sets the maximum number of items that should be returned on a single page.
    #[serde(rename = "maxPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_page_size: Option<i64>,
}

impl ListObjectsQueryRequest {
    pub fn builder() -> ListObjectsQueryRequestBuilder {
        <ListObjectsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListObjectsQueryRequestBuilder {
    prefix: Option<String>,
    since_timestamp: Option<DateTime<FixedOffset>>,
    page_token: Option<String>,
    all_objects_in_mesh: Option<bool>,
    max_page_size: Option<i64>,
}

impl ListObjectsQueryRequestBuilder {
    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn since_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_timestamp = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn all_objects_in_mesh(mut self, value: bool) -> Self {
        self.all_objects_in_mesh = Some(value);
        self
    }

    pub fn max_page_size(mut self, value: i64) -> Self {
        self.max_page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListObjectsQueryRequest`].
    pub fn build(self) -> Result<ListObjectsQueryRequest, BuildError> {
        Ok(ListObjectsQueryRequest {
            prefix: self.prefix,
            since_timestamp: self.since_timestamp,
            page_token: self.page_token,
            all_objects_in_mesh: self.all_objects_in_mesh,
            max_page_size: self.max_page_size,
        })
    }
}
