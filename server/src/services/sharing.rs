use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Owner,
    Editor,
    Viewer,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
    pub role: Role,
}

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
