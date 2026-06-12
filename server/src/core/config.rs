use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// PostgreSQL connection URL
    pub database_url: String,

    /// Storage backend type: "local", "nfs", "s3"
    pub storage_backend: String,

    /// Path or endpoint for storage
    pub storage_path: String,

    /// NAS mount path (if using NFS)
    pub nas_mount_path: Option<String>,

    /// Sentry DSN for error tracking
    pub sentry_dsn: String,

    /// Environment: development, staging, production
    pub environment: String,

    /// Base URL for API responses
    pub api_base_url: String,

    /// JWT secret for token signing
    pub jwt_secret: String,

    /// Max file size in bytes (default: 5GB)
    pub max_file_size: u64,

    /// Sync protocol: "hearth", "immich"
    pub sync_protocol: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/hearth".to_string()),
            storage_backend: std::env::var("STORAGE_BACKEND")
                .unwrap_or_else(|_| "local".to_string()),
            storage_path: std::env::var("STORAGE_PATH")
                .unwrap_or_else(|_| "/tmp/hearth-storage".to_string()),
            nas_mount_path: std::env::var("NAS_MOUNT_PATH").ok(),
            sentry_dsn: std::env::var("SENTRY_DSN").unwrap_or_default(),
            environment: std::env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_string()),
            max_file_size: std::env::var("MAX_FILE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5 * 1024 * 1024 * 1024), // 5GB default
            sync_protocol: std::env::var("SYNC_PROTOCOL")
                .unwrap_or_else(|_| "hearth".to_string()),
        }
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
