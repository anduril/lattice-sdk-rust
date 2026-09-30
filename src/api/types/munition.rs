pub use crate::prelude::*;

/// Munition describes an entity's munitions stores
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Munition {
    /// Unique munition identifier
    #[serde(rename = "munitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub munition_id: Option<String>,
    /// Long form name of the munition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of units
    #[serde(rename = "quantityUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_units: Option<i64>,
}

impl Munition {
    pub fn builder() -> MunitionBuilder {
        <MunitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MunitionBuilder {
    munition_id: Option<String>,
    name: Option<String>,
    quantity_units: Option<i64>,
}

impl MunitionBuilder {
    pub fn munition_id(mut self, value: impl Into<String>) -> Self {
        self.munition_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn quantity_units(mut self, value: i64) -> Self {
        self.quantity_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Munition`].
    pub fn build(self) -> Result<Munition, BuildError> {
        Ok(Munition {
            munition_id: self.munition_id,
            name: self.name,
            quantity_units: self.quantity_units,
        })
    }
}
