use std::env;
use std::error::Error;

pub struct Config {
    pub log_level: String,
    pub port: u16,
}

pub fn load() -> Result<Config, Box<dyn Error>> {
    dotenvy::dotenv().ok();

    Ok(Config {
        log_level: env::var("LOG_LEVEL")?,
        port: env::var("PORT")?.parse()?,
    })
}
