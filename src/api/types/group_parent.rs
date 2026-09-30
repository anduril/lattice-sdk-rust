pub use crate::prelude::*;

/// A GroupParent relationship is a uni-directional relationship indicating that this entity is a member of
/// the Entity Group represented by the related entity. The presence of this relationship alone determines that
/// the type of group that this entity is a member of is an Entity Group.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupParent {}

impl GroupParent {
    pub fn builder() -> GroupParentBuilder {
        <GroupParentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupParentBuilder {}

impl GroupParentBuilder {
    /// Consumes the builder and constructs a [`GroupParent`].
    pub fn build(self) -> Result<GroupParent, BuildError> {
        Ok(GroupParent {})
    }
}
