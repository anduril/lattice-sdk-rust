pub use crate::prelude::*;

/// A MergedFrom relationship is a uni-directional relationship indicating that this entity is a merged entity whose
/// data has at least partially been merged from the related entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergedFrom {}

impl MergedFrom {
    pub fn builder() -> MergedFromBuilder {
        <MergedFromBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergedFromBuilder {}

impl MergedFromBuilder {
    /// Consumes the builder and constructs a [`MergedFrom`].
    pub fn build(self) -> Result<MergedFrom, BuildError> {
        Ok(MergedFrom {})
    }
}
