pub use crate::prelude::*;

/// The PredicateSet represents a list of predicates or "leaf nodes" in the expression tree, which
/// can be directly evaluated to a boolean TRUE/FALSE result.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PredicateSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicates: Option<Vec<Predicate>>,
}

impl PredicateSet {
    pub fn builder() -> PredicateSetBuilder {
        <PredicateSetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PredicateSetBuilder {
    predicates: Option<Vec<Predicate>>,
}

impl PredicateSetBuilder {
    pub fn predicates(mut self, value: Vec<Predicate>) -> Self {
        self.predicates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PredicateSet`].
    pub fn build(self) -> Result<PredicateSet, BuildError> {
        Ok(PredicateSet {
            predicates: self.predicates,
        })
    }
}
