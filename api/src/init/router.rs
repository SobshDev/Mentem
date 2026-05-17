use axum::Router;

use super::rate_limit;
use crate::modules;
use crate::modules::auth::http_routes;
use crate::state::AppState;

pub fn build(state: AppState) -> Router
{
    let router = Router::new()
        .nest("/health", modules::health::router())
        .nest("/auth", http_routes::router(state));

    rate_limit::general(router)
}
