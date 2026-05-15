pub type UserId = String;

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
