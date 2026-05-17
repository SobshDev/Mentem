use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

use super::domain::UserId;
use super::error::AuthError;
use crate::state::AppState;

pub struct AuthUser(pub UserId);

impl FromRequestParts<AppState> for AuthUser
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection>
    {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::InvalidCredentials)?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidCredentials)?;

        let claims = state.auth.verify_token(token)?;
        Ok(Self(claims.user_id))
    }
}
