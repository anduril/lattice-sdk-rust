pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NonPrimaryMembership {}

impl NonPrimaryMembership {
    pub fn builder() -> NonPrimaryMembershipBuilder {
        <NonPrimaryMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NonPrimaryMembershipBuilder {}

impl NonPrimaryMembershipBuilder {
    /// Consumes the builder and constructs a [`NonPrimaryMembership`].
    pub fn build(self) -> Result<NonPrimaryMembership, BuildError> {
        Ok(NonPrimaryMembership {})
    }
}
