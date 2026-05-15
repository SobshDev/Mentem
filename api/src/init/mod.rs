mod database;
mod env;
mod router;

use axum::Router;
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

use crate::modules;

pub struct App
{
    pub router: Router,
    pub port: u16,
    #[allow(dead_code)]
    pub pool: PgPool,
}

pub async fn init() -> App
{
    modules::health::init();
    let cfg = env::load().expect("failed to load config");
    init_tracing(&cfg.log_level);

    let pool = database::init(&cfg.database_url).await;
    let router = router::build();

    App {
        router,
        port: cfg.port,
        pool,
    }
}

fn init_tracing(log_level: &str)
{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}
