use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use super::UserRepository;
use crate::modules::auth::domain::{NewUser, User, UserId};
use crate::modules::auth::error::AuthError;

pub struct PgUserRepository
{
    pool: PgPool,
}

impl PgUserRepository
{
    pub fn new(pool: PgPool) -> Self
    {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository
{
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>
    {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AuthError>
    {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, password_hash FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)
    }

    async fn insert(&self, user: NewUser) -> Result<User, AuthError>
    {
        let id = Uuid::new_v4();
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (id, email, password_hash)
               VALUES ($1, $2, $3)
               RETURNING id, email, password_hash"#,
            id,
            user.email,
            user.password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.code().as_deref() == Some("23505") => {
                AuthError::EmailAlreadyExists
            }
            other => internal(other),
        })
    }
}

fn internal(e: sqlx::Error) -> AuthError
{
    AuthError::Internal(Box::new(e))
}
