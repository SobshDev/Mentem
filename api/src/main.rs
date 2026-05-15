mod config;

use axum::{Router, routing::get};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cfg = config::load().expect("failed to load config");
    init_tracing(&cfg.log_level);
    serve(router(), cfg.port).await;
}

fn init_tracing(log_level: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}

fn router() -> Router {
    Router::new().route("/", get(|| async { "Hello World!" }))
}

async fn serve(app: Router, port: u16) {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}
