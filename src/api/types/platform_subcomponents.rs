pub use crate::prelude::*;

/// Describes a PlatformSubcomponents group type. Comprised of entities which
/// are subcomponents of the parent platform and the parent platform itself.
/// Subcomponents are assumed to be positionally related to the parent group.
/// relationship to a radar entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformSubcomponents {}

impl PlatformSubcomponents {
    pub fn builder() -> PlatformSubcomponentsBuilder {
        <PlatformSubcomponentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformSubcomponentsBuilder {}

impl PlatformSubcomponentsBuilder {
    /// Consumes the builder and constructs a [`PlatformSubcomponents`].
    pub fn build(self) -> Result<PlatformSubcomponents, BuildError> {
        Ok(PlatformSubcomponents {})
    }
}
