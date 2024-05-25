use tokio::sync::OnceCell;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn initialize_database_connection() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("set the \"DATABASE_URL\" environment variable");
    let max_conn = match std::env::var("MAX_DB_CONNECTION") {
        Ok(mc) => mc.parse::<u32>().unwrap(),
        Err(_) => 4,
    };
    PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(url.as_str())
        .await.expect("unable to connect to database")
}

pub async fn pool() -> &'static Pool<Postgres> {
    POOL.get_or_init(initialize_database_connection).await
}