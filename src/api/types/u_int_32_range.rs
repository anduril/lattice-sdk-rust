pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UInt32Range {
    #[serde(rename = "lowerBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<i64>,
    #[serde(rename = "upperBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<i64>,
}

impl UInt32Range {
    pub fn builder() -> UInt32RangeBuilder {
        <UInt32RangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UInt32RangeBuilder {
    lower_bound: Option<i64>,
    upper_bound: Option<i64>,
}

impl UInt32RangeBuilder {
    pub fn lower_bound(mut self, value: i64) -> Self {
        self.lower_bound = Some(value);
        self
    }

    pub fn upper_bound(mut self, value: i64) -> Self {
        self.upper_bound = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UInt32Range`].
    pub fn build(self) -> Result<UInt32Range, BuildError> {
        Ok(UInt32Range {
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
        })
    }
}
