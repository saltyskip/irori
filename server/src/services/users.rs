use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn register(&self, request: RegisterRequest) -> crate::Result<User>;

    async fn login(&self, request: LoginRequest) -> crate::Result<LoginResponse>;

    async fn get(&self, user_id: Uuid) -> crate::Result<Option<User>>;

    async fn get_by_email(&self, email: &str) -> crate::Result<Option<User>>;

    async fn update_profile(&self, user_id: Uuid, name: Option<String>) -> crate::Result<User>;

    async fn delete(&self, user_id: Uuid) -> crate::Result<()>;
}
