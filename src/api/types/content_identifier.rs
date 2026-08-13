pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContentIdentifier {
    /// A valid path must not contain the following:
    /// - Spaces or Tabs
    /// - Special characters other than underscore (_), dash (-), period (.) and slash (/)
    /// - Non-ASCII characters such as accents or symbols
    /// Paths must not start with a leading space.
    #[serde(default)]
    pub path: String,
    /// The SHA-256 checksum of this object.
    #[serde(default)]
    pub checksum: String,
}

impl ContentIdentifier {
    pub fn builder() -> ContentIdentifierBuilder {
        <ContentIdentifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentIdentifierBuilder {
    path: Option<String>,
    checksum: Option<String>,
}

impl ContentIdentifierBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContentIdentifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](ContentIdentifierBuilder::path)
    /// - [`checksum`](ContentIdentifierBuilder::checksum)
    pub fn build(self) -> Result<ContentIdentifier, BuildError> {
        Ok(ContentIdentifier {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            checksum: self
                .checksum
                .ok_or_else(|| BuildError::missing_field("checksum"))?,
        })
    }
}
