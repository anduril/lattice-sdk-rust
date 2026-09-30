pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Orbit {
    /// Orbit Mean Elements data, analogous to the Orbit Mean Elements Message in CCSDS 502.0-B-3
    #[serde(rename = "orbitMeanElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit_mean_elements: Option<OrbitMeanElements>,
}

impl Orbit {
    pub fn builder() -> OrbitBuilder {
        <OrbitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrbitBuilder {
    orbit_mean_elements: Option<OrbitMeanElements>,
}

impl OrbitBuilder {
    pub fn orbit_mean_elements(mut self, value: OrbitMeanElements) -> Self {
        self.orbit_mean_elements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Orbit`].
    pub fn build(self) -> Result<Orbit, BuildError> {
        Ok(Orbit {
            orbit_mean_elements: self.orbit_mean_elements,
        })
    }
}
