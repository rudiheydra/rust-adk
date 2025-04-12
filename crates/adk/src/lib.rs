//! Agent Kit - A Rust implementation of OpenAI agents kit
//!
//! This crate provides a Rust implementation of the OpenAI agents kit,
//! allowing you to create and manage AI agents using the OpenAI API.

pub mod agent;
pub mod error;
pub mod openai;
pub mod tool;
pub mod types;

pub use agent::Agent;
pub use error::AgentError;
pub use openai::Model;
pub use tool::{Tool, ToolResult};

/// Re-export common types for convenience
pub mod prelude {
    pub use super::types::*;
    pub use super::Agent;
    pub use super::AgentError;
}
