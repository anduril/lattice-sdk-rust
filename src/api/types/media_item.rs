pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MediaItem {
    /// A unique identifier for this mediaItem.
    #[serde(rename = "itemIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_identifier: Option<String>,
    /// The type of media for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MediaItemType>,
    /// The path, relative to the environment base URL, where media related to an entity can be accessed
    #[serde(rename = "relativePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}

impl MediaItem {
    pub fn builder() -> MediaItemBuilder {
        <MediaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaItemBuilder {
    item_identifier: Option<String>,
    r#type: Option<MediaItemType>,
    relative_path: Option<String>,
}

impl MediaItemBuilder {
    pub fn item_identifier(mut self, value: impl Into<String>) -> Self {
        self.item_identifier = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: MediaItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn relative_path(mut self, value: impl Into<String>) -> Self {
        self.relative_path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaItem`].
    pub fn build(self) -> Result<MediaItem, BuildError> {
        Ok(MediaItem {
            item_identifier: self.item_identifier,
            r#type: self.r#type,
            relative_path: self.relative_path,
        })
    }
}
