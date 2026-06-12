use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Owner,
    Editor,
    Viewer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub collection_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
    pub role: Role,
}
