pub use crate::prelude::*;

/// The PositionType represents any fixed LLA point in space.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PositionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Position>,
}

impl PositionType {
    pub fn builder() -> PositionTypeBuilder {
        <PositionTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionTypeBuilder {
    value: Option<Position>,
}

impl PositionTypeBuilder {
    pub fn value(mut self, value: Position) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionType`].
    pub fn build(self) -> Result<PositionType, BuildError> {
        Ok(PositionType { value: self.value })
    }
}
