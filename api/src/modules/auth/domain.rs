use uuid::Uuid;

use super::error::AuthError;

pub type UserId = Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email
{
    pub fn new(value: String) -> Result<Self, AuthError>
    {
        if value.is_empty() || !value.contains('@') {
            return Err(AuthError::InvalidEmail);
        }
        Ok(Email(value))
    }

    pub fn as_str(&self) -> &str
    {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password
{
    pub fn new(value: String) -> Result<Self, AuthError>
    {
        if value.len() < 8 {
            return Err(AuthError::PasswordTooShort);
        }
        Ok(Password(value))
    }

    pub fn as_str(&self) -> &str
    {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Credentials
{
    pub email: Email,
    pub password: Password,
}

impl Credentials
{
    pub fn new(email: String, password: String) -> Result<Self, AuthError>
    {
        Ok(Credentials {
            email: Email::new(email)?,
            password: Password::new(password)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct User
{
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct NewUser
{
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct TokenClaims
{
    pub user_id: UserId,
    pub expires_at: i64,
}
