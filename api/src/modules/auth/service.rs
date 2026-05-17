use std::sync::Arc;

use super::domain::{NewUser, TokenClaims, User, UserId};
use super::error::AuthError;
use super::ports::{PasswordHasher, TokenService, UserRepository};

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

    pub async fn register(&self, credentials: super::domain::Credentials) -> Result<User, AuthError>
    {
        let password_hash = self.hasher.hash(credentials.password.as_str()).await?;
        self.users
            .insert(NewUser {
                email: credentials.email.as_str().to_string(),
                password_hash,
            })
            .await
    }

    pub async fn login(&self, credentials: super::domain::Credentials) -> Result<String, AuthError>
    {
        let user_result = self.users.find_by_email(credentials.email.as_str()).await?;

        let hash_to_verify = match &user_result {
            Some(user) => user.password_hash.clone(),
            None => self.hasher.dummy_hash().await?,
        };

        if !self.hasher.verify(credentials.password.as_str(), &hash_to_verify).await? {
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
