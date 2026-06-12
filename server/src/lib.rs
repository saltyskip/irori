// Core application modules
pub mod app;
pub mod error;

// Infrastructure layer
pub mod core {
    pub mod config;
    pub mod db;
    pub mod storage;
}

// Service layer (business logic, transport-agnostic)
pub mod services {
    pub mod resources;
    pub mod collections;
    pub mod sharing;
    pub mod sync;
    pub mod users;
}

// Transport layers
pub mod api;
pub mod mcp;

// Utilities
pub mod util {
    pub mod id;
}

// Re-exports
pub use app::AppState;
pub use error::{Error, Result};
