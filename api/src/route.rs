use axum::Router;

use crate::modules;

pub fn router() -> Router {
    Router::new().nest("/health", modules::health::router())
}
