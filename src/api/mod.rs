//! API client and types for the The Entities API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    EntitiesClient, EntityClient, Lattice, OauthClient, ObjectClient, ObjectsClient, TaskClient,
    TasksClient,
};
pub use types::*;
