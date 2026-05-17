use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::domain::{TokenClaims, UserId};
use super::super::error::AuthError;
use super::super::ports::TokenService;

const TTL: Duration = Duration::from_secs(60 * 60);

pub struct JwtTokenService
{
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    ttl: Duration,
}

impl JwtTokenService
{
    pub fn new(secret: &str) -> Self
    {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            validation: Validation::new(Algorithm::HS256),
            ttl: TTL,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Claims
{
    sub: String,
    exp: i64,
}

impl TokenService for JwtTokenService
{
    fn issue(&self, user_id: &UserId) -> Result<String, AuthError>
    {
        let exp = (SystemTime::now() + self.ttl)
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthError::Internal(Box::new(e)))?
            .as_secs() as i64;

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
        };

        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)
            .map_err(|e| AuthError::Internal(Box::new(e)))
    }

    fn verify(&self, token: &str) -> Result<TokenClaims, AuthError>
    {
        let data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|_| AuthError::InvalidCredentials)?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthError::Internal(Box::new(e)))?
            .as_secs() as i64;

        if data.claims.exp < current_time {
            return Err(AuthError::TokenExpired);
        }

        let user_id =
            Uuid::parse_str(&data.claims.sub).map_err(|_| AuthError::InvalidCredentials)?;

        Ok(TokenClaims {
            user_id,
            expires_at: data.claims.exp,
        })
    }
}
