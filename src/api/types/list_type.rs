pub use crate::prelude::*;

/// A List of Values for use with the IN comparator.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Box<Value>>>,
}

impl ListType {
    pub fn builder() -> ListTypeBuilder {
        <ListTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTypeBuilder {
    values: Option<Vec<Box<Value>>>,
}

impl ListTypeBuilder {
    pub fn values(mut self, value: Vec<Box<Value>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListType`].
    pub fn build(self) -> Result<ListType, BuildError> {
        Ok(ListType {
            values: self.values,
        })
    }
}
