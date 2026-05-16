use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher as _, PasswordVerifier};

use crate::modules::auth::{AuthError, PasswordHasher};

pub struct Argon2Hasher
{
    argon2: Argon2<'static>,
}

impl Argon2Hasher
{
    pub fn new() -> Self
    {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHasher for Argon2Hasher
{
    fn hash(&self, password: &str) -> Result<String, AuthError>
    {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::Internal(e.to_string().into()))?;
        Ok(hash.to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>
    {
        let parsed =
            PasswordHash::new(hash).map_err(|e| AuthError::Internal(e.to_string().into()))?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed)
            .is_ok())
    }
}
