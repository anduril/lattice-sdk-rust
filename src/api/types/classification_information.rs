pub use crate::prelude::*;

/// Represents all of the necessary information required to generate a summarized
/// classification marking.
///
/// > example: A summarized classification marking of "TOPSECRET//NOFORN//FISA"
/// would be defined as: { "level": 5, "caveats": [ "NOFORN, "FISA" ] }
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClassificationInformation {
    /// Classification level to be applied to the information in question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<ClassificationInformationLevel>,
    /// Caveats that may further restrict how the information can be disseminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caveats: Option<Vec<String>>,
}

impl ClassificationInformation {
    pub fn builder() -> ClassificationInformationBuilder {
        <ClassificationInformationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationInformationBuilder {
    level: Option<ClassificationInformationLevel>,
    caveats: Option<Vec<String>>,
}

impl ClassificationInformationBuilder {
    pub fn level(mut self, value: ClassificationInformationLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn caveats(mut self, value: Vec<String>) -> Self {
        self.caveats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClassificationInformation`].
    pub fn build(self) -> Result<ClassificationInformation, BuildError> {
        Ok(ClassificationInformation {
            level: self.level,
            caveats: self.caveats,
        })
    }
}
