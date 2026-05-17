use axum::Router;

use super::rate_limit;
use crate::modules;
use crate::state::AppState;

pub fn build(state: AppState) -> Router
{
    let router = Router::new()
        .nest("/health", modules::health::router())
        .nest("/auth", modules::auth::router(state));

    rate_limit::general(router)
}
