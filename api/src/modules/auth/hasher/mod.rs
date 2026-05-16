mod argon2;

use super::error::AuthError;

pub use argon2::Argon2Hasher;

pub trait PasswordHasher: Send + Sync
{
    fn hash(&self, password: &str) -> Result<String, AuthError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>;
}
