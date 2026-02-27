//! Rust wrapper for the IBM MQ administrative REST API.

pub mod auth;
mod commands;
pub mod ensure;
pub mod error;
pub mod mapping;
pub mod mapping_data;
pub mod mapping_merge;
pub mod session;
pub mod sync_ops;
pub mod transport;

pub use auth::Credentials;
pub use ensure::{EnsureAction, EnsureResult};
pub use error::{MappingError, MappingIssue, MqRestError, Result};
pub use mapping::{map_request_attributes, map_response_attributes, map_response_list};
pub use mapping_merge::MappingOverrideMode;
pub use session::{MqRestSession, MqRestSessionBuilder};
pub use sync_ops::{SyncConfig, SyncOperation, SyncResult};
pub use transport::{MqRestTransport, ReqwestTransport, TransportResponse};
