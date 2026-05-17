use std::env;
use std::error::Error;

// HS256 requires at least 32 bytes of secret per RFC 7518 §3.2.
const MIN_JWT_SECRET_LEN: usize = 32;

pub struct Config
{
    pub log_level: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

pub fn load() -> Result<Config, Box<dyn Error>>
{
    dotenvy::dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET")?;
    if jwt_secret.len() < MIN_JWT_SECRET_LEN {
        return Err(format!(
            "JWT_SECRET must be at least {MIN_JWT_SECRET_LEN} bytes, got {}",
            jwt_secret.len()
        )
        .into());
    }

    Ok(Config {
        log_level: env::var("LOG_LEVEL")?,
        port: env::var("PORT")?.parse()?,
        database_url: env::var("DATABASE_URL")?,
        jwt_secret,
    })
}
