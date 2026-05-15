use std::sync::LazyLock;
use std::time::Instant;

static START: LazyLock<Instant> = LazyLock::new(Instant::now);

pub fn init()
{
    LazyLock::force(&START);
}

pub async fn health() -> &'static str
{
    "ok"
}

pub async fn uptime() -> String
{
    let secs = START.elapsed().as_secs();
    format!("{secs}s")
}
