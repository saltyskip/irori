use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifies the syncing client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub client_id: String,
    pub version: String,
    pub last_sync: Option<String>, // Cursor from previous sync
}

/// A change that occurred on the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub change_type: ChangeType,
    pub resource_id: Uuid,
    pub timestamp: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
}

/// Changes since last sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    pub cursor: String,           // Use for next sync
    pub changes: Vec<Change>,
    pub has_more: bool,
    pub checkpoint: Option<String>,
}

/// Local changes from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChange {
    pub id: String,
    pub change_type: ChangeType,
    pub resource_id: Uuid,
    pub data: Option<serde_json::Value>,
}

/// Acknowledgment of synced changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAck {
    pub received_count: usize,
    pub cursor: String,
    pub conflicts: Vec<ConflictInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub resource_id: Uuid,
    pub client_version: String,
    pub server_version: String,
}

/// File upload/download handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHandle {
    pub id: Uuid,
    pub path: String,
    pub size: u64,
    pub checksum: String,
}
