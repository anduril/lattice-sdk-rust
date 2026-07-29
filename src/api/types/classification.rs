pub use crate::prelude::*;

/// A component that describes an entity's security classification levels.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Classification {
    /// The default classification information which should be assumed to apply to everything in
    /// the entity unless a specific field level classification is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<ClassificationInformation>,
    /// The set of individual field classification information which should always precedence
    /// over the default classification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldClassificationInformation>>,
}

impl Classification {
    pub fn builder() -> ClassificationBuilder {
        <ClassificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClassificationBuilder {
    default: Option<ClassificationInformation>,
    fields: Option<Vec<FieldClassificationInformation>>,
}

impl ClassificationBuilder {
    pub fn default(mut self, value: ClassificationInformation) -> Self {
        self.default = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<FieldClassificationInformation>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Classification`].
    pub fn build(self) -> Result<Classification, BuildError> {
        Ok(Classification {
            default: self.default,
            fields: self.fields,
        })
    }
}
