pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrimaryMembership {}

impl PrimaryMembership {
    pub fn builder() -> PrimaryMembershipBuilder {
        <PrimaryMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrimaryMembershipBuilder {}

impl PrimaryMembershipBuilder {
    /// Consumes the builder and constructs a [`PrimaryMembership`].
    pub fn build(self) -> Result<PrimaryMembership, BuildError> {
        Ok(PrimaryMembership {})
    }
}
