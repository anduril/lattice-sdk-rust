pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayloadConfiguration {
    /// Identifying ID for the capability.
    /// This ID may be used multiple times to represent payloads that are the same capability but have different operational states
    #[serde(rename = "capabilityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    /// The number of payloads currently available in the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// The target environments the configuration is effective against.
    #[serde(rename = "effectiveEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_environment: Option<Vec<PayloadConfigurationEffectiveEnvironmentItem>>,
    /// The operational state of this payload.
    #[serde(rename = "payloadOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_operational_state: Option<PayloadConfigurationPayloadOperationalState>,
    /// A human readable description of the payload
    #[serde(rename = "payloadDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_description: Option<String>,
}

impl PayloadConfiguration {
    pub fn builder() -> PayloadConfigurationBuilder {
        <PayloadConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayloadConfigurationBuilder {
    capability_id: Option<String>,
    quantity: Option<i64>,
    effective_environment: Option<Vec<PayloadConfigurationEffectiveEnvironmentItem>>,
    payload_operational_state: Option<PayloadConfigurationPayloadOperationalState>,
    payload_description: Option<String>,
}

impl PayloadConfigurationBuilder {
    pub fn capability_id(mut self, value: impl Into<String>) -> Self {
        self.capability_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn effective_environment(
        mut self,
        value: Vec<PayloadConfigurationEffectiveEnvironmentItem>,
    ) -> Self {
        self.effective_environment = Some(value);
        self
    }

    pub fn payload_operational_state(
        mut self,
        value: PayloadConfigurationPayloadOperationalState,
    ) -> Self {
        self.payload_operational_state = Some(value);
        self
    }

    pub fn payload_description(mut self, value: impl Into<String>) -> Self {
        self.payload_description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayloadConfiguration`].
    pub fn build(self) -> Result<PayloadConfiguration, BuildError> {
        Ok(PayloadConfiguration {
            capability_id: self.capability_id,
            quantity: self.quantity,
            effective_environment: self.effective_environment,
            payload_operational_state: self.payload_operational_state,
            payload_description: self.payload_description,
        })
    }
}
