use axum::Router;

use super::rate_limit;
use crate::modules;

pub fn build() -> Router
{
    let router = Router::new()
        .nest("/health", modules::health::router())
        .nest("/auth", modules::auth::router());

    rate_limit::general(router)
}
