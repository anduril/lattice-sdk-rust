pub use crate::prelude::*;

/// A field specific classification information definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FieldClassificationInformation {
    /// Proto field path which is the string representation of a field.
    /// > example: signal.bandwidth_hz would be bandwidth_hz in the signal component
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    /// The information which makes up the field level classification marking.
    #[serde(rename = "classificationInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_information: Option<ClassificationInformation>,
}

impl FieldClassificationInformation {
    pub fn builder() -> FieldClassificationInformationBuilder {
        <FieldClassificationInformationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FieldClassificationInformationBuilder {
    field_path: Option<String>,
    classification_information: Option<ClassificationInformation>,
}

impl FieldClassificationInformationBuilder {
    pub fn field_path(mut self, value: impl Into<String>) -> Self {
        self.field_path = Some(value.into());
        self
    }

    pub fn classification_information(mut self, value: ClassificationInformation) -> Self {
        self.classification_information = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FieldClassificationInformation`].
    pub fn build(self) -> Result<FieldClassificationInformation, BuildError> {
        Ok(FieldClassificationInformation {
            field_path: self.field_path,
            classification_information: self.classification_information,
        })
    }
}
