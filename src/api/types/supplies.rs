pub use crate::prelude::*;

/// Represents the state of supplies associated with an entity (available but not in condition to use immediately)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Supplies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub munitions: Option<Vec<Munition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<Vec<Fuel>>,
}

impl Supplies {
    pub fn builder() -> SuppliesBuilder {
        <SuppliesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SuppliesBuilder {
    munitions: Option<Vec<Munition>>,
    fuel: Option<Vec<Fuel>>,
}

impl SuppliesBuilder {
    pub fn munitions(mut self, value: Vec<Munition>) -> Self {
        self.munitions = Some(value);
        self
    }

    pub fn fuel(mut self, value: Vec<Fuel>) -> Self {
        self.fuel = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Supplies`].
    pub fn build(self) -> Result<Supplies, BuildError> {
        Ok(Supplies {
            munitions: self.munitions,
            fuel: self.fuel,
        })
    }
}
