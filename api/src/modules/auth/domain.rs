use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug, Clone)]
pub struct User
{
    pub id: UserId,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct NewUser
{
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct TokenClaims
{
    pub user_id: UserId,
    pub expires_at: i64,
}
