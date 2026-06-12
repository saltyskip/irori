use std::sync::Arc;

use crate::core::config::Config;
use crate::core::db::DbPool;
use crate::core::storage::StorageBackend;

/// Shared application state available to all handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub config: Config,
    pub storage: Arc<Box<dyn StorageBackend>>,
}
