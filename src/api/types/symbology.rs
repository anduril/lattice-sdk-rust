pub use crate::prelude::*;

/// Symbology associated with an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Symbology {
    #[serde(rename = "milStd2525C")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mil_std2525c: Option<MilStd2525C>,
}

impl Symbology {
    pub fn builder() -> SymbologyBuilder {
        <SymbologyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SymbologyBuilder {
    mil_std2525c: Option<MilStd2525C>,
}

impl SymbologyBuilder {
    pub fn mil_std2525c(mut self, value: MilStd2525C) -> Self {
        self.mil_std2525c = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Symbology`].
    pub fn build(self) -> Result<Symbology, BuildError> {
        Ok(Symbology {
            mil_std2525c: self.mil_std2525c,
        })
    }
}
