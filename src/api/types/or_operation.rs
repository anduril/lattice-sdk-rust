pub use crate::prelude::*;

/// The OrOperation represents the boolean OR operation, which is to be applied to the list of
/// children statement(s) or predicate(s).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrOperation {
    #[serde(rename = "predicateSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_set: Option<PredicateSet>,
    #[serde(rename = "statementSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_set: Option<Box<StatementSet>>,
}

impl OrOperation {
    pub fn builder() -> OrOperationBuilder {
        <OrOperationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrOperationBuilder {
    predicate_set: Option<PredicateSet>,
    statement_set: Option<Box<StatementSet>>,
}

impl OrOperationBuilder {
    pub fn predicate_set(mut self, value: PredicateSet) -> Self {
        self.predicate_set = Some(value);
        self
    }

    pub fn statement_set(mut self, value: Box<StatementSet>) -> Self {
        self.statement_set = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrOperation`].
    pub fn build(self) -> Result<OrOperation, BuildError> {
        Ok(OrOperation {
            predicate_set: self.predicate_set,
            statement_set: self.statement_set,
        })
    }
}
