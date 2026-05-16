use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init(url: &str) -> PgPool
{
    let pool = connect(url).await.expect("failed to connect to database");
    migrate(&pool).await.expect("migrations failed");
    pool
}

async fn connect(url: &str) -> Result<PgPool, sqlx::Error>
{
    PgPoolOptions::new().max_connections(10).connect(url).await
}

async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError>
{
    sqlx::migrate!("./migrations").run(pool).await
}
