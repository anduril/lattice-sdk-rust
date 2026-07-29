pub use crate::prelude::*;

/// A task represents a structured unit of work that can be assigned to an agent for execution.
///
/// Tasks are the fundamental building blocks of work assignment in the Lattice.
/// Each task has a unique identifier, a specification defining what needs to be done,
/// status information tracking its progress, and various metadata facilitating its lifecycle management.
///
/// Tasks can be related to each other, through parent-child relationships, assigned to
/// specific agents, and tracked through a well-defined state machine from creation to completion.
/// They support rich status reporting, including progress updates, error handling, and results.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Task {
    /// Version of this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<TaskVersion>,
    /// DEPRECATED: Human readable display name for this task, should be short (<100 chars).
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The path for the Protobuf task definition, and the complete task data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<GoogleProtobufAny>,
    /// Records who created this task. This field will not change after the task has been created.
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Principal>,
    /// Records who updated this task last.
    #[serde(rename = "lastUpdatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<Principal>,
    /// Records the time of last update.
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_update_time: Option<DateTime<FixedOffset>>,
    /// The status of this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// If the task has been scheduled to execute, what time it should execute at.
    #[serde(rename = "scheduledTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub scheduled_time: Option<DateTime<FixedOffset>>,
    /// Any related Tasks associated with this, typically includes an assignee for this task and/or a parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Relations>,
    /// Longer, free form human readable description of this task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If set, execution of this task is managed elsewhere, not by Task Manager.
    /// In other words, task manager will not attempt to update the assigned agent with execution instructions.
    #[serde(rename = "isExecutedElsewhere")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_executed_elsewhere: Option<bool>,
    /// Time of task creation.
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// If populated, designates this to be a replicated task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<Replication>,
    /// If populated, indicates an initial set of entities that can be used to execute an entity aware task
    /// For example, an entity Objective, an entity Keep In Zone, etc.
    /// These will not be updated during execution. If a taskable agent needs continuous updates on the entities from the
    /// COP, can call entity-manager, or use an AlternateId escape hatch.
    #[serde(rename = "initialEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_entities: Option<Vec<TaskEntity>>,
    /// The networked owner of this task. It is used to ensure that linear writes occur on the node responsible
    /// for replication of task data to other nodes running Task Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    /// Sets an optional try strategy for tasks. Use this option to control how Lattice attempts to retry delivery of tasks to assets with intermittent access or network connectivity to your environment.
    #[serde(rename = "retryStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    /// The current delivery state of a task.
    #[serde(rename = "deliveryState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_state: Option<DeliveryState>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        <TaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskBuilder {
    version: Option<TaskVersion>,
    display_name: Option<String>,
    specification: Option<GoogleProtobufAny>,
    created_by: Option<Principal>,
    last_updated_by: Option<Principal>,
    last_update_time: Option<DateTime<FixedOffset>>,
    status: Option<TaskStatus>,
    scheduled_time: Option<DateTime<FixedOffset>>,
    relations: Option<Relations>,
    description: Option<String>,
    is_executed_elsewhere: Option<bool>,
    create_time: Option<DateTime<FixedOffset>>,
    replication: Option<Replication>,
    initial_entities: Option<Vec<TaskEntity>>,
    owner: Option<Owner>,
    retry_strategy: Option<RetryStrategy>,
    delivery_state: Option<DeliveryState>,
}

impl TaskBuilder {
    pub fn version(mut self, value: TaskVersion) -> Self {
        self.version = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn specification(mut self, value: GoogleProtobufAny) -> Self {
        self.specification = Some(value);
        self
    }

    pub fn created_by(mut self, value: Principal) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: Principal) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn last_update_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_update_time = Some(value);
        self
    }

    pub fn status(mut self, value: TaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn scheduled_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_time = Some(value);
        self
    }

    pub fn relations(mut self, value: Relations) -> Self {
        self.relations = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_executed_elsewhere(mut self, value: bool) -> Self {
        self.is_executed_elsewhere = Some(value);
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn replication(mut self, value: Replication) -> Self {
        self.replication = Some(value);
        self
    }

    pub fn initial_entities(mut self, value: Vec<TaskEntity>) -> Self {
        self.initial_entities = Some(value);
        self
    }

    pub fn owner(mut self, value: Owner) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn retry_strategy(mut self, value: RetryStrategy) -> Self {
        self.retry_strategy = Some(value);
        self
    }

    pub fn delivery_state(mut self, value: DeliveryState) -> Self {
        self.delivery_state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Task`].
    pub fn build(self) -> Result<Task, BuildError> {
        Ok(Task {
            version: self.version,
            display_name: self.display_name,
            specification: self.specification,
            created_by: self.created_by,
            last_updated_by: self.last_updated_by,
            last_update_time: self.last_update_time,
            status: self.status,
            scheduled_time: self.scheduled_time,
            relations: self.relations,
            description: self.description,
            is_executed_elsewhere: self.is_executed_elsewhere,
            create_time: self.create_time,
            replication: self.replication,
            initial_entities: self.initial_entities,
            owner: self.owner,
            retry_strategy: self.retry_strategy,
            delivery_state: self.delivery_state,
        })
    }
}
