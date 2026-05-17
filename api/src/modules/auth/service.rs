use std::sync::Arc;

use super::domain::{NewUser, User, UserId};
use super::error::AuthError;
use super::hasher::PasswordHasher;
use super::repository::UserRepository;
use super::token::TokenService;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AuthService
{
    users: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
    tokens: Arc<dyn TokenService>,
}

impl AuthService
{
    pub fn new(
        users: Arc<dyn UserRepository>,
        hasher: Arc<dyn PasswordHasher>,
        tokens: Arc<dyn TokenService>,
    ) -> Self
    {
        Self {
            users,
            hasher,
            tokens,
        }
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<User, AuthError>
    {
        let password_hash = self.hasher.hash(password).await?;
        self.users
            .insert(NewUser {
                email: email.to_string(),
                password_hash,
            })
            .await
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, AuthError>
    {
        let user = self
            .users
            .find_by_email(email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        if !self.hasher.verify(password, &user.password_hash).await? {
            return Err(AuthError::InvalidCredentials);
        }

        self.tokens.issue(&user.id)
    }

    pub async fn user(&self, id: &UserId) -> Result<User, AuthError>
    {
        self.users
            .find_by_id(id)
            .await?
            .ok_or(AuthError::UserNotFound)
    }
}
