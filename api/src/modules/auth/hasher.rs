use super::error::AuthError;

pub trait PasswordHasher: Send + Sync
{
    fn hash(&self, password: &str) -> Result<String, AuthError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>;
}
