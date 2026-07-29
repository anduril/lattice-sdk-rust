pub use crate::prelude::*;

/// Metadata about entity overrides present.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Overrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<Vec<Box<Override>>>,
}

impl Overrides {
    pub fn builder() -> OverridesBuilder {
        <OverridesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OverridesBuilder {
    r#override: Option<Vec<Box<Override>>>,
}

impl OverridesBuilder {
    pub fn r#override(mut self, value: Vec<Box<Override>>) -> Self {
        self.r#override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Overrides`].
    pub fn build(self) -> Result<Overrides, BuildError> {
        Ok(Overrides {
            r#override: self.r#override,
        })
    }
}
