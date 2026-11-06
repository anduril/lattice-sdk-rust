pub use crate::prelude::*;

/// Matches text, treating upper and lower case as different. Set exactly one of
/// `exact`, `prefix`, or `contains`, and give it a non-empty value. The server
/// rejects the request with INVALID_ARGUMENT (HTTP 400) if you set none, more than
/// one, or an empty value.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StringFilter {
    /// Matches text that is exactly this value. Can't be combined with `prefix` or
    /// `contains`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    /// Matches text that starts with this value. Can't be combined with `exact` or
    /// `contains`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Matches text that includes this value anywhere. Can't be combined with
    /// `exact` or `prefix`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
}

impl StringFilter {
    pub fn builder() -> StringFilterBuilder {
        <StringFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StringFilterBuilder {
    exact: Option<String>,
    prefix: Option<String>,
    contains: Option<String>,
}

impl StringFilterBuilder {
    pub fn exact(mut self, value: impl Into<String>) -> Self {
        self.exact = Some(value.into());
        self
    }

    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn contains(mut self, value: impl Into<String>) -> Self {
        self.contains = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StringFilter`].
    pub fn build(self) -> Result<StringFilter, BuildError> {
        Ok(StringFilter {
            exact: self.exact,
            prefix: self.prefix,
            contains: self.contains,
        })
    }
}
