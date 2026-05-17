use std::error::Error;
use std::fmt;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum AuthError
{
    UserNotFound,
    InvalidCredentials,
    InvalidEmail,
    PasswordTooShort,
    TokenExpired,
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
            Self::InvalidEmail => write!(f, "invalid email format"),
            Self::PasswordTooShort => write!(f, "password must be at least 8 characters"),
            Self::TokenExpired => write!(f, "token has expired"),
            Self::EmailAlreadyExists => write!(f, "email already exists"),
            Self::Internal(e) => write!(f, "internal error: {e}"),
        }
    }
}

impl Error for AuthError {}

#[derive(Serialize)]
struct ErrorBody
{
    error: &'static str,
}

impl IntoResponse for AuthError
{
    fn into_response(self) -> Response
    {
        let (status, error) = match &self {
            Self::UserNotFound => (StatusCode::NOT_FOUND, "user_not_found"),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
            Self::InvalidEmail => (StatusCode::BAD_REQUEST, "invalid_email"),
            Self::PasswordTooShort => (StatusCode::BAD_REQUEST, "password_too_short"),
            Self::TokenExpired => (StatusCode::UNAUTHORIZED, "token_expired"),
            Self::EmailAlreadyExists => (StatusCode::CONFLICT, "email_already_exists"),
            Self::Internal(e) => {
                tracing::error!(error = %e, "internal auth error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
            }
        };
        (status, Json(ErrorBody { error })).into_response()
    }
}
