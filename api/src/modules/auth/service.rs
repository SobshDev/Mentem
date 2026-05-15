use std::sync::Arc;

use super::domain::{User, UserId};
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

    pub async fn register(&self, _email: &str, _password: &str) -> Result<User, AuthError>
    {
        todo!()
    }

    pub async fn login(&self, _email: &str, _password: &str) -> Result<String, AuthError>
    {
        todo!()
    }

    pub async fn user(&self, _id: &UserId) -> Result<User, AuthError>
    {
        todo!()
    }
}
