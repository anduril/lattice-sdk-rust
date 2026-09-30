pub use crate::prelude::*;

/// The EnumType represents members of well-known anduril ontologies, such as "disposition." When
/// such a value is specified, the evaluation library expects the integer representation of the enum
/// value. For example, a disposition derived from ontology.v1 such as "DISPOSITION_HOSTILE" should be
/// represented with the integer value 2.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnumType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl EnumType {
    pub fn builder() -> EnumTypeBuilder {
        <EnumTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnumTypeBuilder {
    value: Option<i64>,
}

impl EnumTypeBuilder {
    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnumType`].
    pub fn build(self) -> Result<EnumType, BuildError> {
        Ok(EnumType { value: self.value })
    }
}
