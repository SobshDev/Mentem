mod jwt;

use super::domain::UserId;
use super::error::AuthError;

pub use jwt::JwtTokenService;

pub struct TokenClaims
{
    pub user_id: UserId,
    pub expires_at: i64,
}

pub trait TokenService: Send + Sync
{
    fn issue(&self, user_id: &UserId) -> Result<String, AuthError>;
    fn verify(&self, token: &str) -> Result<TokenClaims, AuthError>;
}
