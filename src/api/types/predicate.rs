pub use crate::prelude::*;

/// The Predicate fully encodes the information required to make an evaluation of an entity field
/// against a given static value, resulting in a boolean TRUE/FALSE result. The structure of a
/// predicate will always follow: "{entity-value} {comparator} {fixed-value}" where the entity value
/// is determined by the field path.
///
/// For example, a predicate would read as: "{entity.location.velocity_enu} {LESS_THAN} {500kph}"
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Predicate {
    /// The field_path determines which field on an entity is being referenced in this predicate. For
    /// example: correlated.primary_entity_id would be primary_entity_id in correlated component.
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    /// The value determines the fixed value against which the entity field is to be compared.
    /// In the case of COMPARATOR_MATCH_ALL, the value contents do not matter as long as the Value is a supported
    /// type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// The comparator determines the manner in which the entity field and static value are compared.
    /// Comparators may only be applied to certain values. For example, the WITHIN comparator cannot
    /// be used for a boolean value comparison.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<PredicateComparator>,
}

impl Predicate {
    pub fn builder() -> PredicateBuilder {
        <PredicateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PredicateBuilder {
    field_path: Option<String>,
    value: Option<Value>,
    comparator: Option<PredicateComparator>,
}

impl PredicateBuilder {
    pub fn field_path(mut self, value: impl Into<String>) -> Self {
        self.field_path = Some(value.into());
        self
    }

    pub fn value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn comparator(mut self, value: PredicateComparator) -> Self {
        self.comparator = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Predicate`].
    pub fn build(self) -> Result<Predicate, BuildError> {
        Ok(Predicate {
            field_path: self.field_path,
            value: self.value,
            comparator: self.comparator,
        })
    }
}
