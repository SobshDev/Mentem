mod database;
mod env;
pub(crate) mod rate_limit;
mod router;

use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

use crate::modules;
use crate::modules::auth::{Argon2Hasher, AuthService, JwtTokenService, PgUserRepository};
use crate::state::AppState;

pub struct App
{
    pub router: Router,
    pub port: u16,
}

pub async fn init() -> App
{
    modules::health::init();
    let cfg = env::load().expect("failed to load config");
    init_tracing(&cfg.log_level);

    let pool = database::init(&cfg.database_url).await;
    let state = AppState {
        auth: build_auth(pool, &cfg.jwt_secret),
    };

    App {
        router: router::build(state),
        port: cfg.port,
    }
}

fn build_auth(pool: PgPool, jwt_secret: &str) -> AuthService
{
    AuthService::new(
        Arc::new(PgUserRepository::new(pool)),
        Arc::new(Argon2Hasher::new()),
        Arc::new(JwtTokenService::new(jwt_secret)),
    )
}

fn init_tracing(log_level: &str)
{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}
