use std::env;
use std::error::Error;

pub struct Config
{
    pub log_level: String,
    pub port: u16,
    pub database_url: String,
}

pub fn load() -> Result<Config, Box<dyn Error>>
{
    dotenvy::dotenv().ok();

    Ok(Config {
        log_level: env::var("LOG_LEVEL")?,
        port: env::var("PORT")?.parse()?,
        database_url: env::var("DATABASE_URL")?,
    })
}
