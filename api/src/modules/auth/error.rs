use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AuthError
{
    UserNotFound,
    InvalidCredentials,
    EmailAlreadyExists,
    Internal(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for AuthError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::UserNotFound => write!(f, "user not found"),
            Self::InvalidCredentials => write!(f, "invalid credentials"),
            Self::EmailAlreadyExists => write!(f, "email already exists"),
            Self::Internal(e) => write!(f, "internal error: {e}"),
        }
    }
}

impl Error for AuthError {}
