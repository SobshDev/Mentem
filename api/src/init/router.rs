use axum::Router;

use crate::modules;

pub fn build() -> Router
{
    Router::new()
        .nest("/health", modules::health::router())
        .nest("/auth", modules::auth::router())
}
