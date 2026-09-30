pub use crate::prelude::*;

/// A component that describes the scanning characteristics of a signal
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScanCharacteristics {
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<ScanCharacteristicsScanType>,
    #[serde(rename = "scanPeriodS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub scan_period_s: Option<f64>,
}

impl ScanCharacteristics {
    pub fn builder() -> ScanCharacteristicsBuilder {
        <ScanCharacteristicsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScanCharacteristicsBuilder {
    scan_type: Option<ScanCharacteristicsScanType>,
    scan_period_s: Option<f64>,
}

impl ScanCharacteristicsBuilder {
    pub fn scan_type(mut self, value: ScanCharacteristicsScanType) -> Self {
        self.scan_type = Some(value);
        self
    }

    pub fn scan_period_s(mut self, value: f64) -> Self {
        self.scan_period_s = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScanCharacteristics`].
    pub fn build(self) -> Result<ScanCharacteristics, BuildError> {
        Ok(ScanCharacteristics {
            scan_type: self.scan_type,
            scan_period_s: self.scan_period_s,
        })
    }
}
