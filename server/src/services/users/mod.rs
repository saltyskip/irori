pub mod models;

use async_trait::async_trait;
use uuid::Uuid;

pub use models::{LoginRequest, LoginResponse, RegisterRequest, User};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn register(&self, request: RegisterRequest) -> crate::Result<User>;

    async fn login(&self, request: LoginRequest) -> crate::Result<LoginResponse>;

    async fn get(&self, user_id: Uuid) -> crate::Result<Option<User>>;

    async fn get_by_email(&self, email: &str) -> crate::Result<Option<User>>;

    async fn update_profile(&self, user_id: Uuid, name: Option<String>) -> crate::Result<User>;

    async fn delete(&self, user_id: Uuid) -> crate::Result<()>;
}
