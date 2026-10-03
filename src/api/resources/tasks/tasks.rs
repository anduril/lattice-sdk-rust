use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct TasksClient {
    pub http_client: HttpClient,
}

impl TasksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a new Task in the system with the specified parameters.
    ///
    /// This method initiates a new task with a unique ID (either provided or auto-generated),
    /// sets the initial task state to STATUS_CREATED, and establishes task ownership. The task
    /// can be assigned to a specific agent through the Relations field.
    ///
    /// Once created, a task enters the lifecycle workflow and can be tracked, updated, and managed
    /// through other Tasks API endpoints.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .create_task(
    ///             &TaskCreation {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_task(
        &self,
        request: &TaskCreation,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/tasks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a specific Task by its ID, with options to select a particular task version or view.
    ///
    /// This method returns detailed information about a task including its current status,
    /// specification, relations, and other metadata. The response includes the complete Task object
    /// with all associated fields.
    ///
    /// By default, the method returns the latest definition version of the task from the manager's
    /// perspective.
    ///
    /// # Arguments
    ///
    /// * `task_id` - ID of task to return
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client.tasks.get_task(&"taskId".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_task(
        &self,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/tasks/{}", task_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the status of a Task as it progresses through its lifecycle.
    ///
    /// This method allows agents or operators to report the current state of a task,
    /// which could include changes to task status, and error information.
    ///
    /// Each status update increments the task's status_version. When updating status,
    /// clients must provide the current version to ensure consistency. The system rejects
    /// updates with mismatched versions to prevent race conditions.
    ///
    /// Terminal states (`STATUS_DONE_OK` and `STATUS_DONE_NOT_OK`) are permanent; once a task
    /// reaches these states, no further updates are allowed.
    ///
    /// # Arguments
    ///
    /// * `task_id` - ID of task to update status of
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .update_task_status(
    ///             &"taskId".to_string(),
    ///             &TaskStatusUpdate {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_task_status(
        &self,
        task_id: &str,
        request: &TaskStatusUpdate,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v1/tasks/{}/status", task_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels a task by marking it for cancellation in the system.
    ///
    /// This method initiates task cancellation based on the task's current state:
    /// - If the task has not been sent to an agent, it cancels immediately and transitions the task
    /// to a terminal state (`STATUS_DONE_NOT_OK` with `ERROR_CODE_CANCELLED`).
    /// - If the task has already been sent to an agent, the cancellation request is routed to the agent.
    /// The agent is then responsible for deciding whether cancellation is possible or not:
    /// - If the task can be cancelled, the agent must use `UpdateTaskStatus` and set the task status to `STATUS_DONE_NOT_OK`.
    /// - If the task cannot be cancelled, the agent must use `UpdateTaskStatus` to attach a `TaskError` to the task with the error code `ERROR_CODE_REJECTED`
    /// and a `message` explaining why the task cannot be cancelled.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of task to cancel
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .cancel_task(
    ///             &"taskId".to_string(),
    ///             &TaskCancellation {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_task(
        &self,
        task_id: &str,
        request: &TaskCancellation,
        options: Option<RequestOptions>,
    ) -> Result<Task, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("api/v1/tasks/{}/cancel", task_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Searches for Tasks that match specified filtering criteria and returns matching tasks in paginated form.
    ///
    /// This method allows filtering tasks based on the following criteria:
    /// - Parent-task relationships.
    /// - Task status with inclusive or exclusive filtering.
    /// - Update time ranges.
    ///
    /// Lattice returns the results in pages. When more results are available than can be returned in a single
    /// response, a `page_token` is provided. Use the `page_token` in subsequent requests to retrieve the next
    /// set of results.
    ///
    /// By default, this returns the latest task version for each matching task.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .query_tasks(
    ///             &TaskQuery {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query_tasks(
        &self,
        request: &TaskQuery,
        options: Option<RequestOptions>,
    ) -> Result<TaskQueryResults, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/tasks/query",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Establishes a server streaming connection that delivers task updates in real-time using Server-Sent Events (SSE).
    ///
    /// The stream delivers all existing non-terminal tasks when first connected, followed by real-time
    /// updates for task creation and status changes. Additionally, heartbeat messages are sent periodically to maintain the connection.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use StreamExt from the prelude to iterate)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .stream_tasks(
    ///             &TaskStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_tasks(
        &self,
        request: &TaskStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamTasksResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                "api/v1/tasks/stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// Establishes a server streaming connection that delivers tasks to taskable agents for execution.
    ///
    /// This method creates a persistent connection from Tasks API to an agent, allowing the server
    /// to push tasks to the agent as they become available. The agent receives a stream of tasks that
    /// match its selector criteria (entity IDs).
    ///
    /// The stream delivers three types of requests:
    /// - ExecuteRequest: Contains a new task for the agent to execute
    /// - CancelRequest: Indicates a task should be canceled
    /// - CompleteRequest: Indicates a task should be completed
    ///
    /// This is the primary method for taskable agents to receive and process tasks in real-time.
    /// Agents should maintain this connection and process incoming tasks according to their capabilities.
    ///
    /// When an agent receives a task, it should update the task status using the UpdateStatus endpoint
    /// to provide progress information back to Tasks API.
    ///
    /// This is a long polling API that will block until a new task is ready for delivery. If no new task is
    /// available then the server will hold on to your request for up to 5 minutes, after that 5 minute timeout
    /// period you will be expected to reinitiate a new request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .listen_as_agent(
    ///             &AgentListener {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn listen_as_agent(
        &self,
        request: &AgentListener,
        options: Option<RequestOptions>,
    ) -> Result<AgentRequest, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api/v1/agent/listen",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Establishes a server streaming connection that delivers tasks to taskable agents for execution
    /// using Server-Sent Events (SSE).
    ///
    /// This method creates a connection from the Tasks API to an agent that streams relevant tasks to the listener agent. The agent receives a stream of tasks that match the entities specified by the tasks' selector criteria.
    ///
    /// The stream delivers three types of requests:
    /// - `ExecuteRequest`: Contains a new task for the agent to execute
    /// - `CancelRequest`: Indicates a task should be canceled
    /// - `CompleteRequest`: Indicates a task should be completed
    ///
    /// Additionally, heartbeat messages are sent periodically to maintain the connection.
    ///
    /// This is recommended method for taskable agents to receive and process tasks in real-time.
    /// Agents should maintain connection to this stream and process incoming tasks according to their capabilities.
    ///
    /// When an agent receives a task, it should update the task status using the `UpdateStatus` endpoint
    /// to provide progress information back to Tasks API.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use StreamExt from the prelude to iterate)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .stream_as_agent(
    ///             &AgentStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_as_agent(
        &self,
        request: &AgentStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamAsAgentResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                "api/v1/agent/stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// Establishes a server streaming connection that delivers manual control frames to agents
    /// using server-sent events (SSE).
    ///
    /// This endpoint streams manual control frames, for example, for joystick movements, for a specific task
    /// to the executing agent. The agent should open this stream before reporting `STATUS_EXECUTING`
    /// to ensure it is ready to receive control input when the operator begins sending frames.
    ///
    /// Each frame includes epoch and sequence metadata for handling concurrent control sessions
    /// and detecting stale or out-of-order frames. Heartbeat messages are sent periodically to
    /// maintain the connection.
    ///
    /// The stream terminates automatically when the task reaches a terminal state
    /// (`STATUS_DONE_OK` or `STATUS_DONE_NOT_OK`).
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the manual control task to receive frames for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use StreamExt from the prelude to iterate)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anduril_lattice_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = Lattice::new(config).expect("Failed to build client");
    ///     client
    ///         .tasks
    ///         .stream_manual_control_frames(
    ///             &"taskId".to_string(),
    ///             &ManualControlStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_manual_control_frames(
        &self,
        task_id: &str,
        request: &ManualControlStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<StreamManualControlFramesResponse>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                &format!("api/v1/tasks/{}/manual-control/stream", task_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }
}
