pub use crate::prelude::*;

/// Represents the state of power sources connected to this entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PowerState {
    /// This is a map where the key is a unique id of the power source and the value is additional information about the
    /// power source.
    #[serde(rename = "sourceIdToState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id_to_state: Option<HashMap<String, PowerSource>>,
}

impl PowerState {
    pub fn builder() -> PowerStateBuilder {
        <PowerStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PowerStateBuilder {
    source_id_to_state: Option<HashMap<String, PowerSource>>,
}

impl PowerStateBuilder {
    pub fn source_id_to_state(mut self, value: HashMap<String, PowerSource>) -> Self {
        self.source_id_to_state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PowerState`].
    pub fn build(self) -> Result<PowerState, BuildError> {
        Ok(PowerState {
            source_id_to_state: self.source_id_to_state,
        })
    }
}
