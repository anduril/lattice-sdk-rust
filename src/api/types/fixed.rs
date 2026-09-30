pub use crate::prelude::*;

/// A fix of a signal. No extra fields but it is expected that location should be populated when using this report.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Fixed {}

impl Fixed {
    pub fn builder() -> FixedBuilder {
        <FixedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedBuilder {}

impl FixedBuilder {
    /// Consumes the builder and constructs a [`Fixed`].
    pub fn build(self) -> Result<Fixed, BuildError> {
        Ok(Fixed {})
    }
}
