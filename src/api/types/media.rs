pub use crate::prelude::*;

/// Media associated with an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<MediaItem>>,
}

impl Media {
    pub fn builder() -> MediaBuilder {
        <MediaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaBuilder {
    media: Option<Vec<MediaItem>>,
}

impl MediaBuilder {
    pub fn media(mut self, value: Vec<MediaItem>) -> Self {
        self.media = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Media`].
    pub fn build(self) -> Result<Media, BuildError> {
        Ok(Media { media: self.media })
    }
}
