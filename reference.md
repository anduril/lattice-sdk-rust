# Reference
## Entities
<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">publish_entity</a>(request: Entity) -> Result&lt;Entity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Publish an entity for ingest into the Entities API. Entities created with this method are "owned" by the originator: other sources, 
such as the UI, may not edit or delete these entities. The server validates entities at API call time and 
returns an error if the entity is invalid.

An entity ID must be provided when calling this endpoint. If the entity referenced by the entity ID does not exist
then it will be created. Otherwise the entity will be updated. An entity will only be updated if its
provenance.sourceUpdateTime is greater than the provenance.sourceUpdateTime of the existing entity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .publish_entity(
            &Entity {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">get_entity</a>(entity_id: String) -> Result&lt;Entity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .get_entity(&"entityId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — ID of the entity to return
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">override_entity</a>(entity_id: String, field_path: String, request: EntityOverride) -> Result&lt;Entity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Only fields marked with overridable can be overridden. Please refer to our documentation to see the comprehensive
list of fields that can be overridden. The entity in the request body should only have a value set on the field 
specified in the field path parameter. Field paths are rooted in the base entity object and must be represented 
using lower_snake_case. Do not include "entity" in the field path.

Note that overrides are applied in an eventually consistent manner. If multiple overrides are created 
concurrently for the same field path, the last writer wins.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .override_entity(
            &"entityId".to_string(),
            &"mil_view.disposition".to_string(),
            &EntityOverride {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — The unique ID of the entity to override
    
</dd>
</dl>

<dl>
<dd>

**field_path:** `String` — fieldPath to override
    
</dd>
</dl>

<dl>
<dd>

**entity:** `Option<Entity>` 

The entity containing the overridden fields. The service will extract the overridable fields from 
the object and ignore all other fields.
    
</dd>
</dl>

<dl>
<dd>

**provenance:** `Option<Provenance>` — Additional information about the source of the override.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">remove_entity_override</a>(entity_id: String, field_path: String) -> Result&lt;Entity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This operation clears the override value from the specified field path on the entity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .remove_entity_override(
            &"entityId".to_string(),
            &"mil_view.disposition".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — The unique ID of the entity to undo an override from.
    
</dd>
</dl>

<dl>
<dd>

**field_path:** `String` — The fieldPath to clear overrides from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">long_poll_entity_events</a>(request: EntityEventRequest) -> Result&lt;EntityEventResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This is a long polling API that will first return all pre-existing data and then return all new data as
it becomes available. If you want to start a new polling session then open a request with an empty
'sessionToken' in the request body. The server will return a new session token in the response.
If you want to retrieve the next batch of results from an existing polling session then send the session
token you received from the server in the request body. If no new data is available then the server will
hold the connection open for up to 5 minutes. After the 5 minute timeout period, the server will close the 
connection with no results and you may resume polling with the same session token. If your session falls behind 
more than 3x the total number of entities in the environment, the server will terminate your session. 
In this case you must start a new session by sending a request with an empty session token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .long_poll_entity_events(
            &EntityEventRequest {
                session_token: "sessionToken".to_string(),
                batch_size: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**session_token:** `String` — Long-poll session identifier. Leave empty to start a new polling session.
    
</dd>
</dl>

<dl>
<dd>

**batch_size:** `Option<i64>` — Maximum size of response batch. Defaults to 100. Must be between 1 and 2000 (inclusive).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">stream_entities</a>(request: EntityStreamRequest) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a server-sent events (SSE) connection that streams entity data in real-time.
This is a one-way connection from server to client that follows the SSE protocol with text/event-stream content type.

This endpoint enables clients to maintain a real-time view of the common operational picture (COP)
by first streaming all pre-existing entities that match filter criteria, then continuously delivering
updates as entities are created, modified, or deleted.

The server first sends events with type PREEXISTING for all live entities matching the filter that existed before the stream was open,
then streams CREATE events for newly created entities, UPDATE events when existing entities change, and DELETED events when entities are removed. The stream remains open
indefinitely unless preExistingOnly is set to true.

Heartbeat messages can be configured to maintain connection health and detect disconnects by setting the heartbeatIntervalMS
parameter. These heartbeats help keep the connection alive and allow clients to verify the server is still responsive.

Clients can optimize bandwidth usage by specifying which entity components they need populated using the componentsToInclude parameter.
This allows receiving only relevant data instead of complete entities.

The connection automatically recovers from temporary disconnections, resuming the stream where it left off. Unlike polling approaches,
this provides real-time updates with minimal latency and reduced server load.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .entities
        .stream_entities(
            &EntityStreamRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**heartbeat_interval_ms:** `Option<i64>` — at what interval to send heartbeat events, defaults to 30s.
    
</dd>
</dl>

<dl>
<dd>

**pre_existing_only:** `Option<bool>` — only stream pre-existing entities in the environment and then close the connection, defaults to false.
    
</dd>
</dl>

<dl>
<dd>

**components_to_include:** `Option<Vec<String>>` — list of components to include, leave empty to include all components.
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Statement>` 

Optional root of a Statement filter expression tree. If provided, only entities matching
the filter are streamed. Applied dynamically: an entity that begins matching is delivered
as a CREATE, and one that stops matching is delivered as a DELETE. Mirrors the filter on
the gRPC StreamEntityComponents endpoint.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tasks
<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">create_task</a>(request: TaskCreation) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new Task in the system with the specified parameters.

This method initiates a new task with a unique ID (either provided or auto-generated),
sets the initial task state to STATUS_CREATED, and establishes task ownership. The task
can be assigned to a specific agent through the Relations field.

Once created, a task enters the lifecycle workflow and can be tracked, updated, and managed
through other Tasks API endpoints.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .create_task(
            &TaskCreation {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `Option<String>` 

If non-empty, will set the requested Task ID, otherwise will generate a new random
GUID. Will reject if supplied Task ID does not match [A-Za-z0-9_-.]{5,36}.
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `Option<String>` — Human readable display name for this Task, should be short (<100 chars).
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Longer, free form human readable description of this Task.
    
</dd>
</dl>

<dl>
<dd>

**specification:** `Option<GoogleProtobufAny>` — The path for the Protobuf task definition, and the complete task data.
    
</dd>
</dl>

<dl>
<dd>

**author:** `Option<Principal>` 
    
</dd>
</dl>

<dl>
<dd>

**relations:** `Option<Relations>` 

Any relationships associated with this Task, such as a parent Task or an assignee
this Task is designated to for execution.
    
</dd>
</dl>

<dl>
<dd>

**is_executed_elsewhere:** `Option<bool>` 

If set, then the service will not trigger execution of this task on an agent. Useful
for when ingesting tasks from an external system that is triggering execution of tasks
on agents.
    
</dd>
</dl>

<dl>
<dd>

**initial_entities:** `Option<Vec<TaskEntity>>` 

Indicates an initial set of entities that can be used to execute an entity aware
task. For example, an entity Objective, an entity Keep In Zone, etc.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">get_task</a>(task_id: String) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a specific Task by its ID, with options to select a particular task version or view.

This method returns detailed information about a task including its current status,
specification, relations, and other metadata. The response includes the complete Task object
with all associated fields.

By default, the method returns the latest definition version of the task from the manager's
perspective.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client.tasks.get_task(&"taskId".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — ID of task to return
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">update_task_status</a>(task_id: String, request: TaskStatusUpdate) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the status of a Task as it progresses through its lifecycle.

This method allows agents or operators to report the current state of a task,
which could include changes to task status, and error information.

Each status update increments the task's status_version. When updating status,
clients must provide the current version to ensure consistency. The system rejects
updates with mismatched versions to prevent race conditions.

Terminal states (`STATUS_DONE_OK` and `STATUS_DONE_NOT_OK`) are permanent; once a task
reaches these states, no further updates are allowed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .update_task_status(
            &"taskId".to_string(),
            &TaskStatusUpdate {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — ID of task to update status of
    
</dd>
</dl>

<dl>
<dd>

**status_version:** `Option<i64>` 

The status version of the task to update. This version number increments to indicate the task's 
current stage in its status lifecycle. Specifically, whenever a task's status updates, the status 
version increments by one. Any status updates received with a lower status version number than what 
is known are considered stale and ignored.
    
</dd>
</dl>

<dl>
<dd>

**new_status:** `Option<TaskStatus>` — The new status of the task.
    
</dd>
</dl>

<dl>
<dd>

**author:** `Option<Principal>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">cancel_task</a>(task_id: String, request: TaskCancellation) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels a task by marking it for cancellation in the system.

This method initiates task cancellation based on the task's current state:
- If the task has not been sent to an agent, it cancels immediately and transitions the task
  to a terminal state (`STATUS_DONE_NOT_OK` with `ERROR_CODE_CANCELLED`).
- If the task has already been sent to an agent, the cancellation request is routed to the agent.
  The agent is then responsible for deciding whether cancellation is possible or not:
  - If the task can be cancelled, the agent must use `UpdateTaskStatus` and set the task status to `STATUS_DONE_NOT_OK`.
  - If the task cannot be cancelled, the agent must use `UpdateTaskStatus` to attach a `TaskError` to the task with the error code `ERROR_CODE_REJECTED`
    and a `message` explaining why the task cannot be cancelled.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .cancel_task(
            &"taskId".to_string(),
            &TaskCancellation {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — The ID of task to cancel
    
</dd>
</dl>

<dl>
<dd>

**author:** `Option<Principal>` — Who or what is requesting to cancel this task.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">query_tasks</a>(request: TaskQuery) -> Result&lt;TaskQueryResults, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Searches for Tasks that match specified filtering criteria and returns matching tasks in paginated form.

This method allows filtering tasks based on the following criteria:
- Parent-task relationships.
- Task status with inclusive or exclusive filtering.
- Update time ranges.

Lattice returns the results in pages. When more results are available than can be returned in a single
response, a `page_token` is provided. Use the `page_token` in subsequent requests to retrieve the next
set of results.

By default, this returns the latest task version for each matching task.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .query_tasks(
            &TaskQuery {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_token:** `Option<String>` — If set, returns results starting from the given pageToken.
    
</dd>
</dl>

<dl>
<dd>

**parent_task_id:** `Option<String>` 

If present matches tasks with this parent task ID.
This is mutually exclusive with all other query parameters, for example, either provide parent task ID, or
any of the remaining parameters, but not both.
    
</dd>
</dl>

<dl>
<dd>

**status_filter:** `Option<TaskQueryStatusFilter>` 
    
</dd>
</dl>

<dl>
<dd>

**update_time_range:** `Option<TaskQueryUpdateTimeRange>` — If provided, only provides tasks updated within the time range.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">stream_tasks</a>(request: TaskStreamRequest) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a server streaming connection that delivers task updates in real-time using Server-Sent Events (SSE).

The stream delivers all existing non-terminal tasks when first connected, followed by real-time
updates for task creation and status changes. Additionally, heartbeat messages are sent periodically to maintain the connection.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .stream_tasks(
            &TaskStreamRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**heartbeat_interval_ms:** `Option<i64>` — The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    
</dd>
</dl>

<dl>
<dd>

**rate_limit:** `Option<i64>` 

The time interval, in milliseconds, after an update for a given task before another one will be sent for the same task.
If set, value must be >= 250.
    
</dd>
</dl>

<dl>
<dd>

**exclude_preexisting_tasks:** `Option<bool>` 

Optional flag to only include tasks created or updated after the stream is initiated, and not any previous preexisting tasks.
If unset or false, the stream will include any new tasks and task updates, as well as all preexisting tasks.
    
</dd>
</dl>

<dl>
<dd>

**task_type:** `Option<TaskStreamRequestTaskType>` — Optional filter that only returns tasks with specific types. If not provided, all task types will be streamed.
    
</dd>
</dl>

<dl>
<dd>

**update_start_time:** `Option<Timestamp>` — If provided, returns tasks which have been updated since the given time.
    
</dd>
</dl>

<dl>
<dd>

**parent_task_id:** `Option<String>` 

A filter for tasks with a specific parent task ID.
Note: This filter is mutually exclusive with all other filter fields (`updateStartTime`, `assignee`, `statusFilter`, `taskType`).
Either provide `parentTaskId` or any combination of the other filters, but not both.
    
</dd>
</dl>

<dl>
<dd>

**assignee:** `Option<Principal>` — A filter for tasks assigned to a specific principal.
    
</dd>
</dl>

<dl>
<dd>

**status_filter:** `Option<TaskStreamRequestStatusFilter>` — A filter for task statuses (inclusive or exclusive).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">listen_as_agent</a>(request: AgentListener) -> Result&lt;AgentRequest, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a server streaming connection that delivers tasks to taskable agents for execution.

This method creates a persistent connection from Tasks API to an agent, allowing the server
to push tasks to the agent as they become available. The agent receives a stream of tasks that
match its selector criteria (entity IDs).

The stream delivers three types of requests:
- ExecuteRequest: Contains a new task for the agent to execute
- CancelRequest: Indicates a task should be canceled
- CompleteRequest: Indicates a task should be completed

This is the primary method for taskable agents to receive and process tasks in real-time.
Agents should maintain this connection and process incoming tasks according to their capabilities.

When an agent receives a task, it should update the task status using the UpdateStatus endpoint
to provide progress information back to Tasks API.

This is a long polling API that will block until a new task is ready for delivery. If no new task is
available then the server will hold on to your request for up to 5 minutes, after that 5 minute timeout
period you will be expected to reinitiate a new request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .listen_as_agent(
            &AgentListener {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_selector:** `Option<EntityIdsSelector>` — Selector criteria to determine which Agent Tasks the agent receives
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">stream_as_agent</a>(request: AgentStreamRequest) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a server streaming connection that delivers tasks to taskable agents for execution
using Server-Sent Events (SSE).

This method creates a connection from the Tasks API to an agent that streams relevant tasks to the listener agent. The agent receives a stream of tasks that match the entities specified by the tasks' selector criteria.

The stream delivers three types of requests:
- `ExecuteRequest`: Contains a new task for the agent to execute
- `CancelRequest`: Indicates a task should be canceled
- `CompleteRequest`: Indicates a task should be completed

Additionally, heartbeat messages are sent periodically to maintain the connection.

This is recommended method for taskable agents to receive and process tasks in real-time.
Agents should maintain connection to this stream and process incoming tasks according to their capabilities. 

When an agent receives a task, it should update the task status using the `UpdateStatus` endpoint
to provide progress information back to Tasks API.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .stream_as_agent(
            &AgentStreamRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_selector:** `Option<EntityIdsSelector>` — The selector criteria to determine which tasks the agent receives.
    
</dd>
</dl>

<dl>
<dd>

**heartbeat_interval_ms:** `Option<i64>` — The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">stream_manual_control_frames</a>(task_id: String, request: ManualControlStreamRequest) -> Result&lt;Stream&lt;Vec&lt;u8&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a server streaming connection that delivers manual control frames to agents
using server-sent events (SSE).

This endpoint streams manual control frames, for example, for joystick movements, for a specific task
to the executing agent. The agent should open this stream before reporting `STATUS_EXECUTING`
to ensure it is ready to receive control input when the operator begins sending frames.

Each frame includes epoch and sequence metadata for handling concurrent control sessions
and detecting stale or out-of-order frames. Heartbeat messages are sent periodically to
maintain the connection.

The stream terminates automatically when the task reaches a terminal state
(`STATUS_DONE_OK` or `STATUS_DONE_NOT_OK`).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .tasks
        .stream_manual_control_frames(
            &"taskId".to_string(),
            &ManualControlStreamRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — The ID of the manual control task to receive frames for.
    
</dd>
</dl>

<dl>
<dd>

**heartbeat_interval_ms:** `Option<i64>` — The time interval, in milliseconds, that determines the frequency at which to send heartbeat events. Defaults to 30000 (30 seconds).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Objects
<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">list_objects</a>(prefix: Option&lt;Option&lt;String&gt;&gt;, since_timestamp: Option&lt;Option&lt;String&gt;&gt;, page_token: Option&lt;Option&lt;String&gt;&gt;, all_objects_in_mesh: Option&lt;Option&lt;bool&gt;&gt;, max_page_size: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists objects in your environment. You can define a prefix to list a subset of your objects. If you do not set a prefix, Lattice returns all available objects. By default this endpoint will list local objects only.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .objects
        .list_objects(
            &ListObjectsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**prefix:** `Option<String>` — Filters the objects based on the specified prefix path. If no path is specified, all objects are returned.
    
</dd>
</dl>

<dl>
<dd>

**since_timestamp:** `Option<String>` — Sets the age for the oldest objects to query across the environment.
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<String>` — Base64 and URL-encoded cursor returned by the service to continue paging.
    
</dd>
</dl>

<dl>
<dd>

**all_objects_in_mesh:** `Option<bool>` — Lists objects across all environment nodes in a Lattice Mesh.
    
</dd>
</dl>

<dl>
<dd>

**max_page_size:** `Option<i64>` — Sets the maximum number of items that should be returned on a single page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">get_object</a>(object_path: String) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches an object from your environment using the objectPath path parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .objects
        .get_object(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to fetch.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">upload_object</a>(object_path: String) -> Result&lt;PathMetadata, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Uploads an object. The object must be 1 GiB or smaller.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client.objects.upload_object(&vec![], None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — Path of the Object that is to be uploaded.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">delete_object</a>(object_path: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an object from your environment given the objectPath path parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .objects
        .delete_object(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">get_object_metadata</a>(object_path: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns metadata for a specified object path. Use this to fetch metadata such as object size (size_bytes), its expiry time (expiry_time), or its latest update timestamp (last_updated_at).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .objects
        .get_object_metadata(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to query.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## oauth
<details><summary><code>client.oauth.<a href="/src/api/resources/oauth/client.rs">get_token</a>(request: GetTokenRequest) -> Result&lt;GetTokenResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets a new short-lived token using the specified client credentials
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use anduril::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = Lattice::new(config).expect("Failed to build client");
    client
        .oauth
        .get_token(
            &GetTokenRequest {
                grant_type: "client_credentials".to_string(),
                client_id: None,
                client_secret: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**grant_type:** `String` — The type of grant being requested
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<String>` — The client identifier
    
</dd>
</dl>

<dl>
<dd>

**client_secret:** `Option<String>` — The client secret
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

