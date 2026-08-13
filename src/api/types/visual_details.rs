pub use crate::prelude::*;

/// Visual details associated with the display of an entity in the client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VisualDetails {
    /// The range rings to display around an entity.
    #[serde(rename = "rangeRings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_rings: Option<RangeRings>,
}

impl VisualDetails {
    pub fn builder() -> VisualDetailsBuilder {
        <VisualDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VisualDetailsBuilder {
    range_rings: Option<RangeRings>,
}

impl VisualDetailsBuilder {
    pub fn range_rings(mut self, value: RangeRings) -> Self {
        self.range_rings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VisualDetails`].
    pub fn build(self) -> Result<VisualDetails, BuildError> {
        Ok(VisualDetails {
            range_rings: self.range_rings,
        })
    }
}
