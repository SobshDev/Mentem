mod domain;
mod error;
mod hasher;
mod repository;
mod service;
mod token;

use axum::Router;

pub use domain::{NewUser, User, UserId};
pub use error::AuthError;
pub use hasher::PasswordHasher;
pub use repository::UserRepository;
pub use service::AuthService;
pub use token::{TokenClaims, TokenService};

pub fn router() -> Router
{
    Router::new()
}
