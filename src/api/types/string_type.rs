pub use crate::prelude::*;

/// The StringType represents static string values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StringType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl StringType {
    pub fn builder() -> StringTypeBuilder {
        <StringTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StringTypeBuilder {
    value: Option<String>,
}

impl StringTypeBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StringType`].
    pub fn build(self) -> Result<StringType, BuildError> {
        Ok(StringType { value: self.value })
    }
}
