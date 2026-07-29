pub use crate::prelude::*;

/// A target relationship is the inverse of TrackedBy; a one-way relation
/// from sensor to target, indicating track(s) currently prioritized by a robot.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActiveTarget {}

impl ActiveTarget {
    pub fn builder() -> ActiveTargetBuilder {
        <ActiveTargetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActiveTargetBuilder {}

impl ActiveTargetBuilder {
    /// Consumes the builder and constructs a [`ActiveTarget`].
    pub fn build(self) -> Result<ActiveTarget, BuildError> {
        Ok(ActiveTarget {})
    }
}
