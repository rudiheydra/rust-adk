pub mod agent;
pub mod error;
pub mod model;
pub mod tool;
pub mod types;

pub use agent::Agent;
pub use error::AgentError;
pub use model::Model;
pub use tool::{Tool, ToolResult};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::agent::Agent;
    pub use crate::error::AgentError;
    pub use crate::model::Model;
    pub use crate::tool::{Tool, ToolResult};
    pub use crate::types::{Context, RunContext};
}
