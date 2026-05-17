use std::error::Error;
use std::fmt;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum AiError
{
    ProviderUnavailable,
    ProviderRateLimited,
    QuotaExceeded,
    InvalidRequest(String),
    ConversationNotFound,
    Unauthorized,
    Internal(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for AiError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Self::ProviderUnavailable => write!(f, "ai provider unavailable"),
            Self::ProviderRateLimited => write!(f, "ai provider rate limited"),
            Self::QuotaExceeded => write!(f, "ai quota exceeded"),
            Self::InvalidRequest(msg) => write!(f, "invalid ai request: {msg}"),
            Self::ConversationNotFound => write!(f, "conversation not found"),
            Self::Unauthorized => write!(f, "unauthorized"),
            Self::Internal(e) => write!(f, "internal error: {e}"),
        }
    }
}

impl Error for AiError {}

#[derive(Serialize)]
struct ErrorBody
{
    error: &'static str,
}

impl IntoResponse for AiError
{
    fn into_response(self) -> Response
    {
        let (status, error) = match &self {
            Self::ProviderUnavailable => (StatusCode::SERVICE_UNAVAILABLE, "provider_unavailable"),
            Self::ProviderRateLimited => (StatusCode::TOO_MANY_REQUESTS, "provider_rate_limited"),
            Self::QuotaExceeded => (StatusCode::TOO_MANY_REQUESTS, "quota_exceeded"),
            Self::InvalidRequest(_) => (StatusCode::BAD_REQUEST, "invalid_request"),
            Self::ConversationNotFound => (StatusCode::NOT_FOUND, "conversation_not_found"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            Self::Internal(e) => {
                tracing::error!(error = %e, "internal ai error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
            }
        };
        (status, Json(ErrorBody { error })).into_response()
    }
}
