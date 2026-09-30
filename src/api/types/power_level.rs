pub use crate::prelude::*;

/// Represents the power level of a system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PowerLevel {
    /// Total power capacity of the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub capacity: Option<f64>,
    /// Remaining power capacity of the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub remaining: Option<f64>,
    /// Percent of power remaining.
    #[serde(rename = "percentRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percent_remaining: Option<f64>,
    /// Voltage of the power source subsystem, as reported by the power source. If the source does not report this value
    /// this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub voltage: Option<f64>,
    /// Current in amps of the power source subsystem, as reported by the power source. If the source does not
    /// report this value this field will be null.
    #[serde(rename = "currentAmps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub current_amps: Option<f64>,
    /// Estimated minutes until empty. Calculated with consumption at the moment, as reported by the power source. If the source does not
    /// report this value this field will be null.
    #[serde(rename = "runTimeToEmptyMins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub run_time_to_empty_mins: Option<f64>,
    /// Fuel consumption rate in liters per second.
    #[serde(rename = "consumptionRateLPerS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub consumption_rate_l_per_s: Option<f64>,
}

impl PowerLevel {
    pub fn builder() -> PowerLevelBuilder {
        <PowerLevelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PowerLevelBuilder {
    capacity: Option<f64>,
    remaining: Option<f64>,
    percent_remaining: Option<f64>,
    voltage: Option<f64>,
    current_amps: Option<f64>,
    run_time_to_empty_mins: Option<f64>,
    consumption_rate_l_per_s: Option<f64>,
}

impl PowerLevelBuilder {
    pub fn capacity(mut self, value: f64) -> Self {
        self.capacity = Some(value);
        self
    }

    pub fn remaining(mut self, value: f64) -> Self {
        self.remaining = Some(value);
        self
    }

    pub fn percent_remaining(mut self, value: f64) -> Self {
        self.percent_remaining = Some(value);
        self
    }

    pub fn voltage(mut self, value: f64) -> Self {
        self.voltage = Some(value);
        self
    }

    pub fn current_amps(mut self, value: f64) -> Self {
        self.current_amps = Some(value);
        self
    }

    pub fn run_time_to_empty_mins(mut self, value: f64) -> Self {
        self.run_time_to_empty_mins = Some(value);
        self
    }

    pub fn consumption_rate_l_per_s(mut self, value: f64) -> Self {
        self.consumption_rate_l_per_s = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PowerLevel`].
    pub fn build(self) -> Result<PowerLevel, BuildError> {
        Ok(PowerLevel {
            capacity: self.capacity,
            remaining: self.remaining,
            percent_remaining: self.percent_remaining,
            voltage: self.voltage,
            current_amps: self.current_amps,
            run_time_to_empty_mins: self.run_time_to_empty_mins,
            consumption_rate_l_per_s: self.consumption_rate_l_per_s,
        })
    }
}
