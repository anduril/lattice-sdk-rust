pub use crate::prelude::*;

/// List of payloads available for an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Payloads {
    #[serde(rename = "payloadConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_configurations: Option<Vec<Payload>>,
}

impl Payloads {
    pub fn builder() -> PayloadsBuilder {
        <PayloadsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayloadsBuilder {
    payload_configurations: Option<Vec<Payload>>,
}

impl PayloadsBuilder {
    pub fn payload_configurations(mut self, value: Vec<Payload>) -> Self {
        self.payload_configurations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Payloads`].
    pub fn build(self) -> Result<Payloads, BuildError> {
        Ok(Payloads {
            payload_configurations: self.payload_configurations,
        })
    }
}
