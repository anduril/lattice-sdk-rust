pub use crate::prelude::*;

/// The HeadingType represents the heading in degrees for an entity's
/// attitudeEnu quaternion to be compared against. Defaults between a range of 0 to 360
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HeadingType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl HeadingType {
    pub fn builder() -> HeadingTypeBuilder {
        <HeadingTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HeadingTypeBuilder {
    value: Option<i64>,
}

impl HeadingTypeBuilder {
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HeadingType`].
    pub fn build(self) -> Result<HeadingType, BuildError> {
        Ok(HeadingType { value: self.value })
    }
}
