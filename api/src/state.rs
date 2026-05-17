use crate::modules::auth::AuthService;

#[derive(Clone)]
pub struct AppState
{
    pub auth: AuthService,
}
