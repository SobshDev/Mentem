mod handler;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(handler::health))
}
