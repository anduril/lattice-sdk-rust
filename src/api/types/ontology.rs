pub use crate::prelude::*;

/// Ontology of the entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Ontology {
    /// A string that describes the entity's high-level type with natural language.
    #[serde(rename = "platformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// A string that describes the entity's exact model or type.
    #[serde(rename = "specificType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_type: Option<String>,
    /// The template used when creating this entity. Specifies minimum required components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<OntologyTemplate>,
}

impl Ontology {
    pub fn builder() -> OntologyBuilder {
        <OntologyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OntologyBuilder {
    platform_type: Option<String>,
    specific_type: Option<String>,
    template: Option<OntologyTemplate>,
}

impl OntologyBuilder {
    pub fn platform_type(mut self, value: impl Into<String>) -> Self {
        self.platform_type = Some(value.into());
        self
    }

    pub fn specific_type(mut self, value: impl Into<String>) -> Self {
        self.specific_type = Some(value.into());
        self
    }

    pub fn template(mut self, value: OntologyTemplate) -> Self {
        self.template = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Ontology`].
    pub fn build(self) -> Result<Ontology, BuildError> {
        Ok(Ontology {
            platform_type: self.platform_type,
            specific_type: self.specific_type,
            template: self.template,
        })
    }
}
