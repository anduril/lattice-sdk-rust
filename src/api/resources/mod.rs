//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Entities**
//! - **Tasks**
//! - **Objects**
//! - **oauth**
//! - **Video**
//! - **Entity**
//! - **Object**
//! - **Task**

use crate::{ApiError, ClientConfig};

pub mod entities;
pub mod entity;
pub mod oauth;
pub mod object;
pub mod objects;
pub mod task;
pub mod tasks;
pub mod video;
pub struct Lattice {
    pub config: ClientConfig,
    pub entities: EntitiesClient,
    pub tasks: TasksClient,
    pub objects: ObjectsClient,
    pub oauth: OauthClient,
    pub video: VideoClient,
}

impl Lattice {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            entities: EntitiesClient::new(config.clone())?,
            tasks: TasksClient::new(config.clone())?,
            objects: ObjectsClient::new(config.clone())?,
            oauth: OauthClient::new(config.clone())?,
            video: VideoClient::new(config.clone())?,
        })
    }
}

pub use entities::EntitiesClient;
pub use entity::EntityClient;
pub use oauth::OauthClient;
pub use object::ObjectClient;
pub use objects::ObjectsClient;
pub use task::TaskClient;
pub use tasks::TasksClient;
pub use video::VideoClient;
