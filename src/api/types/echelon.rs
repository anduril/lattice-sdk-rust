pub use crate::prelude::*;

/// Describes a Echelon group type.  Comprised of entities which are members of the
/// same unit or echelon. Ex: A group of tanks within a armored company or that same company
/// as a member of a battalion.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Echelon {
    #[serde(rename = "armyEchelon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub army_echelon: Option<EchelonArmyEchelon>,
}

impl Echelon {
    pub fn builder() -> EchelonBuilder {
        <EchelonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EchelonBuilder {
    army_echelon: Option<EchelonArmyEchelon>,
}

impl EchelonBuilder {
    pub fn army_echelon(mut self, value: EchelonArmyEchelon) -> Self {
        self.army_echelon = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Echelon`].
    pub fn build(self) -> Result<Echelon, BuildError> {
        Ok(Echelon {
            army_echelon: self.army_echelon,
        })
    }
}
