use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
pub struct UploadRequest {
    pub name: String,
    pub resource_type: String,
    pub mime_type: Option<String>,
    pub data: Vec<u8>,
}

#[async_trait]
pub trait ResourceService: Send + Sync {
    async fn upload(
        &self,
        user_id: Uuid,
        request: UploadRequest,
    ) -> crate::Result<Resource>;

    async fn get(&self, user_id: Uuid, resource_id: Uuid) -> crate::Result<Option<Resource>>;

    async fn list(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> crate::Result<Vec<Resource>>;

    async fn delete(&self, user_id: Uuid, resource_id: Uuid) -> crate::Result<()>;

    async fn get_bytes(
        &self,
        user_id: Uuid,
        resource_id: Uuid,
    ) -> crate::Result<Vec<u8>>;
}
