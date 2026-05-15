mod config;
mod modules;
mod route;

use axum::Router;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main()
{
    modules::health::init();
    let cfg = config::load().expect("failed to load config");
    init_tracing(&cfg.log_level);
    serve(route::router(), cfg.port).await;
}

fn init_tracing(log_level: &str)
{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}

async fn serve(app: Router, port: u16)
{
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}
