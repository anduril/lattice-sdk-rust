pub use crate::prelude::*;

/// The NotOperation represents the boolean NOT operation, which can only be applied to a single
/// child predicate or statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NotOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<Box<Statement>>,
}

impl NotOperation {
    pub fn builder() -> NotOperationBuilder {
        <NotOperationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotOperationBuilder {
    predicate: Option<Predicate>,
    statement: Option<Box<Statement>>,
}

impl NotOperationBuilder {
    pub fn predicate(mut self, value: Predicate) -> Self {
        self.predicate = Some(value);
        self
    }

    pub fn statement(mut self, value: Box<Statement>) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotOperation`].
    pub fn build(self) -> Result<NotOperation, BuildError> {
        Ok(NotOperation {
            predicate: self.predicate,
            statement: self.statement,
        })
    }
}
