pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResponse {
    #[serde(default)]
    pub path_metadatas: Vec<PathMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListResponse {
    pub fn builder() -> ListResponseBuilder {
        <ListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseBuilder {
    path_metadatas: Option<Vec<PathMetadata>>,
    next_page_token: Option<String>,
}

impl ListResponseBuilder {
    pub fn path_metadatas(mut self, value: Vec<PathMetadata>) -> Self {
        self.path_metadatas = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path_metadatas`](ListResponseBuilder::path_metadatas)
    pub fn build(self) -> Result<ListResponse, BuildError> {
        Ok(ListResponse {
            path_metadatas: self
                .path_metadatas
                .ok_or_else(|| BuildError::missing_field("path_metadatas"))?,
            next_page_token: self.next_page_token,
        })
    }
}
