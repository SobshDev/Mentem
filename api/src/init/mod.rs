mod database;
mod env;
mod rate_limit;
mod router;

use axum::Router;
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;
use std::sync::Arc;

use crate::modules;
use crate::modules::auth::{Argon2Hasher, AuthService, JwtTokenService, PgUserRepository};

pub struct App
{
    pub router: Router,
    pub port: u16,
    #[allow(dead_code)]
    pub pool: PgPool,
    pub auth: AuthService,
}

pub async fn init() -> App
{
    modules::health::init();
    let cfg = env::load().expect("failed to load config");
    init_tracing(&cfg.log_level);

    let pool = database::init(&cfg.database_url).await;
    let router = router::build();
    let auth = init_auth(&pool, &cfg.jwt_secret);

    App {
        router,
        port: cfg.port,
        pool,
        auth,
    }
}

fn init_tracing(log_level: &str)
{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}

fn init_auth(pool: &PgPool, jwt_secret: &str) -> AuthService
{
    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());
    let token_service = Arc::new(JwtTokenService::new(jwt_secret));

    AuthService::new(user_repository, password_hasher, token_service)
}