pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityEventResponse {
    /// Long-poll session identifier. Use this token to resume polling on subsequent requests.
    #[serde(rename = "sessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    #[serde(rename = "entityEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_events: Option<Vec<EntityEvent>>,
}

impl EntityEventResponse {
    pub fn builder() -> EntityEventResponseBuilder {
        <EntityEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntityEventResponseBuilder {
    session_token: Option<String>,
    entity_events: Option<Vec<EntityEvent>>,
}

impl EntityEventResponseBuilder {
    pub fn session_token(mut self, value: impl Into<String>) -> Self {
        self.session_token = Some(value.into());
        self
    }

    pub fn entity_events(mut self, value: Vec<EntityEvent>) -> Self {
        self.entity_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntityEventResponse`].
    pub fn build(self) -> Result<EntityEventResponse, BuildError> {
        Ok(EntityEventResponse {
            session_token: self.session_token,
            entity_events: self.entity_events,
        })
    }
}
