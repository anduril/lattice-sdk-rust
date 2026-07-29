pub use crate::prelude::*;

/// The ListOperation represents an operation against a proto list. If the list is of primitive proto
/// type (e.g. int32), paths in all child predicates should be left empty. If the list is of message
/// proto type (e.g. Sensor), paths in all child predicates should be relative to the list path.
///
/// For example, the criteria "take an action if an entity has any sensor with sensor_id='sensor' and
/// OperationalState=STATE_OFF" would be modeled as:
/// Predicate1: { path: "sensor_id", comparator: EQUAL_TO, value: "sensor" }
/// Predicate2: { path: "operational_state", comparator: EQUAL_TO, value: STATE_OFF }
///
/// Statement2: { AndOperation: PredicateSet: { <Predicate1>, <Predicate2> } }
/// ListOperation: { list_path: "sensors.sensors", list_comparator: ANY, statement: <Statement2> }
/// Statement1: { ListOperation: <ListOperation> }
///
/// Note that in the above, the child predicates of the list operation have paths relative to the
/// list_path because the list is comprised of message not primitive types.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOperation {
    /// The list_path specifies the repeated field on an entity to which this operation applies.
    #[serde(rename = "listPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_path: Option<String>,
    /// The list_comparator specifies how to compose the boolean results from the child statement
    /// for each member of the specified list.
    #[serde(rename = "listComparator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_comparator: Option<ListOperationListComparator>,
    /// The statement is a new expression tree conceptually rooted at type of the list. It determines
    /// how each member of the list is evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<Box<Statement>>,
}

impl ListOperation {
    pub fn builder() -> ListOperationBuilder {
        <ListOperationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOperationBuilder {
    list_path: Option<String>,
    list_comparator: Option<ListOperationListComparator>,
    statement: Option<Box<Statement>>,
}

impl ListOperationBuilder {
    pub fn list_path(mut self, value: impl Into<String>) -> Self {
        self.list_path = Some(value.into());
        self
    }

    pub fn list_comparator(mut self, value: ListOperationListComparator) -> Self {
        self.list_comparator = Some(value);
        self
    }

    pub fn statement(mut self, value: Box<Statement>) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOperation`].
    pub fn build(self) -> Result<ListOperation, BuildError> {
        Ok(ListOperation {
            list_path: self.list_path,
            list_comparator: self.list_comparator,
            statement: self.statement,
        })
    }
}
