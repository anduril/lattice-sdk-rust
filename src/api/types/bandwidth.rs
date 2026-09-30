pub use crate::prelude::*;

/// Describes the bandwidth of a signal
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Bandwidth {
    #[serde(rename = "bandwidthHz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bandwidth_hz: Option<f64>,
}

impl Bandwidth {
    pub fn builder() -> BandwidthBuilder {
        <BandwidthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BandwidthBuilder {
    bandwidth_hz: Option<f64>,
}

impl BandwidthBuilder {
    pub fn bandwidth_hz(mut self, value: f64) -> Self {
        self.bandwidth_hz = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bandwidth`].
    pub fn build(self) -> Result<Bandwidth, BuildError> {
        Ok(Bandwidth {
            bandwidth_hz: self.bandwidth_hz,
        })
    }
}
