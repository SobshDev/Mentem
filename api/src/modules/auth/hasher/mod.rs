mod argon2;

use async_trait::async_trait;

use super::error::AuthError;

pub use argon2::Argon2Hasher;

#[async_trait]
pub trait PasswordHasher: Send + Sync
{
    async fn hash(&self, password: &str) -> Result<String, AuthError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>;
    async fn dummy_hash(&self) -> Result<String, AuthError>;
}
