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

    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable is required")?;
    if jwt_secret.len() < MIN_JWT_SECRET_LEN {
        return Err(format!(
            "JWT_SECRET must be at least {MIN_JWT_SECRET_LEN} bytes, got {}",
            jwt_secret.len()
        )
        .into());
    }

    let log_level = env::var("LOG_LEVEL")
        .map_err(|_| "LOG_LEVEL environment variable is required")?;
    let port_str = env::var("PORT")
        .map_err(|_| "PORT environment variable is required")?;
    let port = port_str.parse()
        .map_err(|_| "PORT must be a valid u16")?;
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL environment variable is required")?;

    Ok(Config {
        log_level,
        port,
        database_url,
        jwt_secret,
    })
}
