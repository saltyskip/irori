pub mod models;

use async_trait::async_trait;
use uuid::Uuid;

pub use models::{AddMemberRequest, Member, Role};

#[async_trait]
pub trait SharingService: Send + Sync {
    async fn add_member(
        &self,
        collection_id: Uuid,
        request: AddMemberRequest,
    ) -> crate::Result<Member>;

    async fn remove_member(
        &self,
        collection_id: Uuid,
        user_id: Uuid,
    ) -> crate::Result<()>;

    async fn list_members(&self, collection_id: Uuid) -> crate::Result<Vec<Member>>;

    async fn update_role(
        &self,
        collection_id: Uuid,
        user_id: Uuid,
        role: Role,
    ) -> crate::Result<()>;

    async fn get_user_collections(&self, user_id: Uuid) -> crate::Result<Vec<Uuid>>;
}
