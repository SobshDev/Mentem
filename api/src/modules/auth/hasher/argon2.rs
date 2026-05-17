use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier, Version};
use async_trait::async_trait;
use tokio::task;

use super::PasswordHasher;
use crate::modules::auth::error::AuthError;

#[derive(Clone)]
pub struct Argon2Hasher
{
    argon2: Argon2<'static>,
}

impl Argon2Hasher
{
    pub fn new() -> Self
    {
        // OWASP-recommended Argon2id parameters: m=19 MiB, t=2, p=1.
        let params = Params::new(19_456, 2, 1, None).expect("valid argon2 params");
        Self {
            argon2: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
        }
    }
}

impl Default for Argon2Hasher
{
    fn default() -> Self
    {
        Self::new()
    }
}

#[async_trait]
impl PasswordHasher for Argon2Hasher
{
    async fn hash(&self, password: &str) -> Result<String, AuthError>
    {
        let argon2 = self.argon2.clone();
        let password = password.to_owned();
        task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            // argon2::password_hash::Error is not Send + Sync; stringify here.
            let hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| AuthError::Internal(e.to_string().into()))?;
            Ok(hash.to_string())
        })
        .await
        .map_err(|e| AuthError::Internal(Box::new(e)))?
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>
    {
        let argon2 = self.argon2.clone();
        let password = password.to_owned();
        let hash = hash.to_owned();
        task::spawn_blocking(move || {
            let parsed =
                PasswordHash::new(&hash).map_err(|e| AuthError::Internal(e.to_string().into()))?;
            Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
        })
        .await
        .map_err(|e| AuthError::Internal(Box::new(e)))?
    }
}
