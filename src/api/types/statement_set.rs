pub use crate::prelude::*;

/// The StatementSet represents a list of statements or "tree nodes," each of which follow the same
/// behavior as the Statement proto message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatementSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<Box<Statement>>>,
}

impl StatementSet {
    pub fn builder() -> StatementSetBuilder {
        <StatementSetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementSetBuilder {
    statements: Option<Vec<Box<Statement>>>,
}

impl StatementSetBuilder {
    pub fn statements(mut self, value: Vec<Box<Statement>>) -> Self {
        self.statements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementSet`].
    pub fn build(self) -> Result<StatementSet, BuildError> {
        Ok(StatementSet {
            statements: self.statements,
        })
    }
}
