pub use crate::prelude::*;

/// Response containing tasks that match the query criteria.
///
/// This message returns a list of Task objects that satisfy the filter conditions
/// specified in the request. When there are more matching tasks than can be returned
/// in a single response, a page_token is provided to retrieve the next batch in
/// a subsequent request. An empty tasks list with no page_token indicates that
/// there are no more matching tasks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskQueryResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    /// Incomplete results can be detected by a non-empty nextPageToken field in the query results. In order to retrieve
    /// the next page, perform the exact same request as previously and append a pageToken field with the value of
    /// nextPageToken from the previous page. A new nextPageToken is provided on the following pages until all the
    /// results are retrieved.
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl TaskQueryResults {
    pub fn builder() -> TaskQueryResultsBuilder {
        <TaskQueryResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskQueryResultsBuilder {
    tasks: Option<Vec<Task>>,
    next_page_token: Option<String>,
}

impl TaskQueryResultsBuilder {
    pub fn tasks(mut self, value: Vec<Task>) -> Self {
        self.tasks = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaskQueryResults`].
    pub fn build(self) -> Result<TaskQueryResults, BuildError> {
        Ok(TaskQueryResults {
            tasks: self.tasks,
            next_page_token: self.next_page_token,
        })
    }
}
