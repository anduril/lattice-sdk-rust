pub use crate::prelude::*;

/// Individual payload configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PayloadConfiguration>,
}

impl Payload {
    pub fn builder() -> PayloadBuilder {
        <PayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayloadBuilder {
    config: Option<PayloadConfiguration>,
}

impl PayloadBuilder {
    pub fn config(mut self, value: PayloadConfiguration) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Payload`].
    pub fn build(self) -> Result<Payload, BuildError> {
        Ok(Payload {
            config: self.config,
        })
    }
}
