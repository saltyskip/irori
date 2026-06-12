pub mod models;

use async_trait::async_trait;
use uuid::Uuid;

pub use models::{Collection, CreateCollectionRequest, UpdateCollectionRequest};

#[async_trait]
pub trait CollectionService: Send + Sync {
    async fn create(
        &self,
        user_id: Uuid,
        request: CreateCollectionRequest,
    ) -> crate::Result<Collection>;

    async fn get(&self, collection_id: Uuid) -> crate::Result<Option<Collection>>;

    async fn list(&self, user_id: Uuid) -> crate::Result<Vec<Collection>>;

    async fn update(
        &self,
        user_id: Uuid,
        collection_id: Uuid,
        request: UpdateCollectionRequest,
    ) -> crate::Result<Collection>;

    async fn delete(&self, user_id: Uuid, collection_id: Uuid) -> crate::Result<()>;

    async fn add_resource(
        &self,
        collection_id: Uuid,
        resource_id: Uuid,
    ) -> crate::Result<()>;

    async fn remove_resource(
        &self,
        collection_id: Uuid,
        resource_id: Uuid,
    ) -> crate::Result<()>;
}
