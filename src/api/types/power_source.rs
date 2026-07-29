pub use crate::prelude::*;

/// Represents the state of a single power source that is connected to this entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PowerSource {
    /// Status of the power source.
    #[serde(rename = "powerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_status: Option<PowerSourcePowerStatus>,
    /// Used to determine the type of power source.
    #[serde(rename = "powerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_type: Option<PowerSourcePowerType>,
    /// Power level of the system. If absent, the power level is assumed to be unknown.
    #[serde(rename = "powerLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_level: Option<PowerLevel>,
    /// Set of human-readable messages with status of the power system. Typically this would be used in an error state
    /// to provide additional error information. This can also be used for informational messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// Whether the power source is offloadable. If the value is missing (as opposed to false) then the entity does not
    /// report whether the power source is offloadable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offloadable: Option<bool>,
}

impl PowerSource {
    pub fn builder() -> PowerSourceBuilder {
        <PowerSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PowerSourceBuilder {
    power_status: Option<PowerSourcePowerStatus>,
    power_type: Option<PowerSourcePowerType>,
    power_level: Option<PowerLevel>,
    messages: Option<Vec<String>>,
    offloadable: Option<bool>,
}

impl PowerSourceBuilder {
    pub fn power_status(mut self, value: PowerSourcePowerStatus) -> Self {
        self.power_status = Some(value);
        self
    }

    pub fn power_type(mut self, value: PowerSourcePowerType) -> Self {
        self.power_type = Some(value);
        self
    }

    pub fn power_level(mut self, value: PowerLevel) -> Self {
        self.power_level = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<String>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn offloadable(mut self, value: bool) -> Self {
        self.offloadable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PowerSource`].
    pub fn build(self) -> Result<PowerSource, BuildError> {
        Ok(PowerSource {
            power_status: self.power_status,
            power_type: self.power_type,
            power_level: self.power_level,
            messages: self.messages,
            offloadable: self.offloadable,
        })
    }
}
