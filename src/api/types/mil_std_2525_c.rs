pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MilStd2525C {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidc: Option<String>,
}

impl MilStd2525C {
    pub fn builder() -> MilStd2525CBuilder {
        <MilStd2525CBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MilStd2525CBuilder {
    sidc: Option<String>,
}

impl MilStd2525CBuilder {
    pub fn sidc(mut self, value: impl Into<String>) -> Self {
        self.sidc = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MilStd2525C`].
    pub fn build(self) -> Result<MilStd2525C, BuildError> {
        Ok(MilStd2525C { sidc: self.sidc })
    }
}
