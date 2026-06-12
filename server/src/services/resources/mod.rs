pub mod models;

use async_trait::async_trait;
use uuid::Uuid;

pub use models::{Resource, UploadRequest};

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
