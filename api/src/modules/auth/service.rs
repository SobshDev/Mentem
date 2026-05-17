use std::sync::Arc;

use super::domain::{NewUser, User, UserId};
use super::error::AuthError;
use super::hasher::PasswordHasher;
use super::repository::UserRepository;
use super::token::{TokenClaims, TokenService};

#[derive(Clone)]
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
        let user_result = self.users.find_by_email(email).await?;

        let hash_to_verify = match &user_result {
            Some(user) => user.password_hash.clone(),
            None => self.hasher.dummy_hash().await?,
        };

        if !self.hasher.verify(password, &hash_to_verify).await? {
            return Err(AuthError::InvalidCredentials);
        }

        let user = user_result.ok_or(AuthError::InvalidCredentials)?;
        self.tokens.issue(&user.id)
    }

    pub async fn user(&self, id: &UserId) -> Result<User, AuthError>
    {
        self.users
            .find_by_id(id)
            .await?
            .ok_or(AuthError::UserNotFound)
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, AuthError>
    {
        self.tokens.verify(token)
    }
}
