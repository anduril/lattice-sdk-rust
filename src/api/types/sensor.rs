pub use crate::prelude::*;

/// Individual sensor configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Sensor {
    /// This generally is used to indicate a specific type at a more detailed granularity. E.g. COMInt or LWIR
    #[serde(rename = "sensorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_id: Option<String>,
    #[serde(rename = "operationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<SensorOperationalState>,
    /// The type of sensor
    #[serde(rename = "sensorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_type: Option<SensorSensorType>,
    /// A human readable description of the sensor
    #[serde(rename = "sensorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensor_description: Option<String>,
    /// RF configuration details of the sensor
    #[serde(rename = "rfConfiguraton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rf_configuraton: Option<RfConfiguration>,
    /// Time of the latest detection from the sensor
    #[serde(rename = "lastDetectionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_detection_timestamp: Option<DateTime<FixedOffset>>,
    /// Multiple fields of view for a single sensor component
    #[serde(rename = "fieldsOfView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_of_view: Option<Vec<FieldOfView>>,
}

impl Sensor {
    pub fn builder() -> SensorBuilder {
        <SensorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SensorBuilder {
    sensor_id: Option<String>,
    operational_state: Option<SensorOperationalState>,
    sensor_type: Option<SensorSensorType>,
    sensor_description: Option<String>,
    rf_configuraton: Option<RfConfiguration>,
    last_detection_timestamp: Option<DateTime<FixedOffset>>,
    fields_of_view: Option<Vec<FieldOfView>>,
}

impl SensorBuilder {
    pub fn sensor_id(mut self, value: impl Into<String>) -> Self {
        self.sensor_id = Some(value.into());
        self
    }

    pub fn operational_state(mut self, value: SensorOperationalState) -> Self {
        self.operational_state = Some(value);
        self
    }

    pub fn sensor_type(mut self, value: SensorSensorType) -> Self {
        self.sensor_type = Some(value);
        self
    }

    pub fn sensor_description(mut self, value: impl Into<String>) -> Self {
        self.sensor_description = Some(value.into());
        self
    }

    pub fn rf_configuraton(mut self, value: RfConfiguration) -> Self {
        self.rf_configuraton = Some(value);
        self
    }

    pub fn last_detection_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_detection_timestamp = Some(value);
        self
    }

    pub fn fields_of_view(mut self, value: Vec<FieldOfView>) -> Self {
        self.fields_of_view = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Sensor`].
    pub fn build(self) -> Result<Sensor, BuildError> {
        Ok(Sensor {
            sensor_id: self.sensor_id,
            operational_state: self.operational_state,
            sensor_type: self.sensor_type,
            sensor_description: self.sensor_description,
            rf_configuraton: self.rf_configuraton,
            last_detection_timestamp: self.last_detection_timestamp,
            fields_of_view: self.fields_of_view,
        })
    }
}
