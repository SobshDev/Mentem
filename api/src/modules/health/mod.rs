mod handler;

use axum::{Router, routing::get};

pub use handler::init;

pub fn router() -> Router
{
    Router::new()
        .route("/", get(handler::health))
        .route("/uptime", get(handler::uptime))
}
