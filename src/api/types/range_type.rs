pub use crate::prelude::*;

/// The RangeType represents a numeric range.
/// Whether endpoints are included are based on the comparator used.
/// Both endpoints must be of the same numeric type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RangeType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NumericType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NumericType>,
}

impl RangeType {
    pub fn builder() -> RangeTypeBuilder {
        <RangeTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RangeTypeBuilder {
    start: Option<NumericType>,
    end: Option<NumericType>,
}

impl RangeTypeBuilder {
    pub fn start(mut self, value: NumericType) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: NumericType) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RangeType`].
    pub fn build(self) -> Result<RangeType, BuildError> {
        Ok(RangeType {
            start: self.start,
            end: self.end,
        })
    }
}
