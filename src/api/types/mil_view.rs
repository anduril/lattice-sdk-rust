pub use crate::prelude::*;

/// Provides the disposition, environment, and nationality of an Entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MilView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<MilViewDisposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<MilViewEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<MilViewNationality>,
}

impl MilView {
    pub fn builder() -> MilViewBuilder {
        <MilViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MilViewBuilder {
    disposition: Option<MilViewDisposition>,
    environment: Option<MilViewEnvironment>,
    nationality: Option<MilViewNationality>,
}

impl MilViewBuilder {
    pub fn disposition(mut self, value: MilViewDisposition) -> Self {
        self.disposition = Some(value);
        self
    }

    pub fn environment(mut self, value: MilViewEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn nationality(mut self, value: MilViewNationality) -> Self {
        self.nationality = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MilView`].
    pub fn build(self) -> Result<MilView, BuildError> {
        Ok(MilView {
            disposition: self.disposition,
            environment: self.environment,
            nationality: self.nationality,
        })
    }
}
