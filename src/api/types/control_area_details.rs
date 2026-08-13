pub use crate::prelude::*;

/// Determines the type of control area being represented by the geo-entity,
/// in which an asset can, or cannot, operate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ControlAreaDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ControlAreaDetailsType>,
}

impl ControlAreaDetails {
    pub fn builder() -> ControlAreaDetailsBuilder {
        <ControlAreaDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ControlAreaDetailsBuilder {
    r#type: Option<ControlAreaDetailsType>,
}

impl ControlAreaDetailsBuilder {
    pub fn r#type(mut self, value: ControlAreaDetailsType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ControlAreaDetails`].
    pub fn build(self) -> Result<ControlAreaDetails, BuildError> {
        Ok(ControlAreaDetails {
            r#type: self.r#type,
        })
    }
}
