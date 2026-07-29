pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskCreation {
    /// If non-empty, will set the requested Task ID, otherwise will generate a new random
    /// GUID. Will reject if supplied Task ID does not match [A-Za-z0-9_-.]{5,36}.
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Human readable display name for this Task, should be short (<100 chars).
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Longer, free form human readable description of this Task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The path for the Protobuf task definition, and the complete task data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Principal>,
    /// Any relationships associated with this Task, such as a parent Task or an assignee
    /// this Task is designated to for execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
    /// If set, then the service will not trigger execution of this task on an agent. Useful
    /// for when ingesting tasks from an external system that is triggering execution of tasks
    /// on agents.
    #[serde(rename = "isExecutedElsewhere")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_executed_elsewhere: Option<bool>,
    /// Indicates an initial set of entities that can be used to execute an entity aware
    /// task. For example, an entity Objective, an entity Keep In Zone, etc.
    #[serde(rename = "initialEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_entities: Option<Vec<TaskEntity>>,
}

impl TaskCreation {
    pub fn builder() -> TaskCreationBuilder {
        <TaskCreationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskCreationBuilder {
    task_id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    specification: Option<GoogleProtobufAny>,
    author: Option<Principal>,
    relations: Option<Relations>,
    is_executed_elsewhere: Option<bool>,
    initial_entities: Option<Vec<TaskEntity>>,
}

impl TaskCreationBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn specification(mut self, value: GoogleProtobufAny) -> Self {
        self.specification = Some(value);
        self
    }

    pub fn author(mut self, value: Principal) -> Self {
        self.author = Some(value);
        self
    }

    pub fn relations(mut self, value: Relations) -> Self {
        self.relations = Some(value);
        self
    }

    pub fn is_executed_elsewhere(mut self, value: bool) -> Self {
        self.is_executed_elsewhere = Some(value);
        self
    }

    pub fn initial_entities(mut self, value: Vec<TaskEntity>) -> Self {
        self.initial_entities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskCreation`].
    pub fn build(self) -> Result<TaskCreation, BuildError> {
        Ok(TaskCreation {
            task_id: self.task_id,
            display_name: self.display_name,
            description: self.description,
            specification: self.specification,
            author: self.author,
            relations: self.relations,
            is_executed_elsewhere: self.is_executed_elsewhere,
            initial_entities: self.initial_entities,
        })
    }
}
