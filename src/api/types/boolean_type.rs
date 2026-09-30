pub use crate::prelude::*;

/// The BooleanType represents a static boolean value.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooleanType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

impl BooleanType {
    pub fn builder() -> BooleanTypeBuilder {
        <BooleanTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooleanTypeBuilder {
    value: Option<bool>,
}

impl BooleanTypeBuilder {
    pub fn value(mut self, value: bool) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooleanType`].
    pub fn build(self) -> Result<BooleanType, BuildError> {
        Ok(BooleanType { value: self.value })
    }
}
