pub use crate::prelude::*;

/// An alert informs operators of critical events related to system performance and mission
/// execution. An alert is produced as a result of one or more alert conditions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Alert {
    /// Short, machine-readable code that describes this alert. This code is intended to provide systems off-asset
    /// with a lookup key to retrieve more detailed information about the alert.
    #[serde(rename = "alertCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_code: Option<String>,
    /// Human-readable description of this alert. The description is intended for display in the UI for human
    /// understanding and should not be used for machine processing. If the description is fixed and the vehicle controller
    /// provides no dynamic substitutions, then prefer lookup based on alert_code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Alert level (Warning, Caution, or Advisory).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<AlertLevel>,
    /// Time at which this alert was activated.
    #[serde(rename = "activatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub activated_time: Option<DateTime<FixedOffset>>,
    /// Set of conditions which have activated this alert.
    #[serde(rename = "activeConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_conditions: Option<Vec<AlertCondition>>,
}

impl Alert {
    pub fn builder() -> AlertBuilder {
        <AlertBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertBuilder {
    alert_code: Option<String>,
    description: Option<String>,
    level: Option<AlertLevel>,
    activated_time: Option<DateTime<FixedOffset>>,
    active_conditions: Option<Vec<AlertCondition>>,
}

impl AlertBuilder {
    pub fn alert_code(mut self, value: impl Into<String>) -> Self {
        self.alert_code = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn level(mut self, value: AlertLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn activated_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.activated_time = Some(value);
        self
    }

    pub fn active_conditions(mut self, value: Vec<AlertCondition>) -> Self {
        self.active_conditions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Alert`].
    pub fn build(self) -> Result<Alert, BuildError> {
        Ok(Alert {
            alert_code: self.alert_code,
            description: self.description,
            level: self.level,
            activated_time: self.activated_time,
            active_conditions: self.active_conditions,
        })
    }
}
