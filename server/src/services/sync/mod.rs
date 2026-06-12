pub mod models;

use async_trait::async_trait;
use uuid::Uuid;

pub use models::{
    Change, ChangeSet, ChangeType, ClientState, ConflictInfo, FileHandle, LocalChange, SyncAck,
};

/// Pluggable sync protocol implementation
#[async_trait]
pub trait SyncProtocol: Send + Sync {
    /// Identify the client
    async fn identify(&self, auth: &str, client_state: ClientState) -> crate::Result<ClientState>;

    /// Get changes since last sync
    async fn fetch_changes(
        &self,
        user_id: Uuid,
        cursor: Option<&str>,
        limit: i32,
    ) -> crate::Result<ChangeSet>;

    /// Push local changes to server
    async fn push_changes(
        &self,
        user_id: Uuid,
        changes: Vec<LocalChange>,
    ) -> crate::Result<SyncAck>;

    /// Upload file
    async fn upload_file(
        &self,
        user_id: Uuid,
        name: String,
        data: Vec<u8>,
    ) -> crate::Result<FileHandle>;

    /// Download file
    async fn download_file(
        &self,
        user_id: Uuid,
        file_id: Uuid,
    ) -> crate::Result<Vec<u8>>;
}
