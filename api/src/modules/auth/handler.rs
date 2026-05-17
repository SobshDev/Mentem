use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::domain::User;
use super::error::AuthError;
use super::extractor::AuthUser;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest
{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest
{
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse
{
    pub id: Uuid,
    pub email: String,
}

impl From<User> for UserResponse
{
    fn from(user: User) -> Self
    {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Serialize)]
pub struct LoginResponse
{
    pub token: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AuthError>
{
    let credentials = super::domain::Credentials::new(req.email, req.password)?;
    let user = state.auth.register(credentials).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError>
{
    let credentials = super::domain::Credentials::new(req.email, req.password)?;
    let token = state.auth.login(credentials).await?;
    Ok(Json(LoginResponse { token }))
}

pub async fn me(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>, AuthError>
{
    let user = state.auth.user(&user_id).await?;
    Ok(Json(user.into()))
}
