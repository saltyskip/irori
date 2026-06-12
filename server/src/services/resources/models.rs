use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub resource_type: String, // "photo", "video", "document", etc.
    pub size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub storage_path: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    pub name: String,
    pub resource_type: String,
    pub mime_type: Option<String>,
    pub data: Vec<u8>,
}
