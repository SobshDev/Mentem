mod hasher;
mod repository;
mod token;

use std::sync::Arc;

use sqlx::PgPool;

use super::service::AuthService;

pub use hasher::Argon2Hasher;
pub use repository::PgUserRepository;
pub use token::JwtTokenService;

pub fn build(pool: PgPool, jwt_secret: &str) -> AuthService
{
    AuthService::new(
        Arc::new(PgUserRepository::new(pool)),
        Arc::new(Argon2Hasher::new()),
        Arc::new(JwtTokenService::new(jwt_secret)),
    )
}
