use async_trait::async_trait;

use super::domain::{NewUser, User, UserId};
use super::error::AuthError;

#[async_trait]
pub trait UserRepository: Send + Sync
{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AuthError>;
    async fn insert(&self, user: NewUser) -> Result<User, AuthError>;
}

#[async_trait]
pub trait PasswordHasher: Send + Sync
{
    async fn hash(&self, password: &str) -> Result<String, AuthError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>;
    async fn dummy_hash(&self) -> Result<String, AuthError>;
}

pub trait TokenService: Send + Sync
{
    fn issue(&self, user_id: &UserId) -> Result<String, AuthError>;
    fn verify(&self, token: &str) -> Result<super::domain::TokenClaims, AuthError>;
}
