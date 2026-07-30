pub use crate::prelude::*;

/// A Statement is the building block of the entity filter. The outermost statement is conceptually
/// the root node of an "expression tree" which allows for the construction of complete boolean
/// logic statements. Statements are formed by grouping sets of children statement(s) or predicate(s)
/// according to the boolean operation which is to be applied.
///
/// For example, the criteria "take an action if an entity is hostile and an air vehicle" can be
/// represented as: Statement1: { AndOperation: { Predicate1, Predicate2 } }. Where Statement1
/// is the root of the expression tree, with an AND operation that is applied to children
/// predicates. The predicates themselves encode "entity is hostile" and "entity is air vehicle."
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Statement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Box<AndOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Box<OrOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<NotOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Box<ListOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
}

impl Statement {
    pub fn builder() -> StatementBuilder {
        <StatementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementBuilder {
    and: Option<Box<AndOperation>>,
    or: Option<Box<OrOperation>>,
    not: Option<Box<NotOperation>>,
    list: Option<Box<ListOperation>>,
    predicate: Option<Predicate>,
}

impl StatementBuilder {
    pub fn and(mut self, value: Box<AndOperation>) -> Self {
        self.and = Some(value);
        self
    }

    pub fn or(mut self, value: Box<OrOperation>) -> Self {
        self.or = Some(value);
        self
    }

    pub fn not(mut self, value: Box<NotOperation>) -> Self {
        self.not = Some(value);
        self
    }

    pub fn list(mut self, value: Box<ListOperation>) -> Self {
        self.list = Some(value);
        self
    }

    pub fn predicate(mut self, value: Predicate) -> Self {
        self.predicate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Statement`].
    pub fn build(self) -> Result<Statement, BuildError> {
        Ok(Statement {
            and: self.and,
            or: self.or,
            not: self.not,
            list: self.list,
            predicate: self.predicate,
        })
    }
}
