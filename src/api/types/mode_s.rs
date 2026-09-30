pub use crate::prelude::*;

/// Describes the Mode S codes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModeS {
    /// Mode S identifier which comprises of 8 alphanumeric characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Mode S ICAO aircraft address. Expected values are between 1 and 16777214 decimal. The Mode S address is
    /// considered unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<i64>,
}

impl ModeS {
    pub fn builder() -> ModeSBuilder {
        <ModeSBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModeSBuilder {
    id: Option<String>,
    address: Option<i64>,
}

impl ModeSBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(mut self, value: i64) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModeS`].
    pub fn build(self) -> Result<ModeS, BuildError> {
        Ok(ModeS {
            id: self.id,
            address: self.address,
        })
    }
}
