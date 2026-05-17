use axum::Router;
use axum::routing::{get, post};

use super::handler;
use crate::init::rate_limit;
use crate::state::AppState;

pub fn router(state: AppState) -> Router
{
    let login = rate_limit::auth_login(Router::new().route("/login", post(handler::login)));
    let register =
        rate_limit::auth_register(Router::new().route("/register", post(handler::register)));

    Router::new()
        .route("/me", get(handler::me))
        .merge(login)
        .merge(register)
        .with_state(state)
}
