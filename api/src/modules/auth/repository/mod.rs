mod pg;

use async_trait::async_trait;

use super::domain::{NewUser, User, UserId};
use super::error::AuthError;

pub use pg::PgUserRepository;

#[async_trait]
pub trait UserRepository: Send + Sync
{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AuthError>;
    async fn insert(&self, user: NewUser) -> Result<User, AuthError>;
}
