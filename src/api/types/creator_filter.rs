pub use crate::prelude::*;

/// Matches streams by who created them. Set exactly one option. The server rejects
/// the request with INVALID_ARGUMENT (HTTP 400) if you set none.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatorFilter {
    /// Set to true to return only streams you created, or false to return only
    /// streams you didn't create. Requires valid credentials. When you set it,
    /// each stream's `egressIds` lists only your own egress sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned: Option<bool>,
}

impl CreatorFilter {
    pub fn builder() -> CreatorFilterBuilder {
        <CreatorFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatorFilterBuilder {
    owned: Option<bool>,
}

impl CreatorFilterBuilder {
    pub fn owned(mut self, value: bool) -> Self {
        self.owned = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatorFilter`].
    pub fn build(self) -> Result<CreatorFilter, BuildError> {
        Ok(CreatorFilter { owned: self.owned })
    }
}
