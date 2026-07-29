pub use crate::prelude::*;

/// A GroupChild relationship is a uni-directional relationship indicating that (1) this entity
/// represents an Entity Group and (2) the related entity is a child member of this group. The presence of this
/// relationship alone determines that the type of group is an Entity Group.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupChild {}

impl GroupChild {
    pub fn builder() -> GroupChildBuilder {
        <GroupChildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupChildBuilder {}

impl GroupChildBuilder {
    /// Consumes the builder and constructs a [`GroupChild`].
    pub fn build(self) -> Result<GroupChild, BuildError> {
        Ok(GroupChild {})
    }
}
