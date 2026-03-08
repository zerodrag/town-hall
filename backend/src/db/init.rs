use color_eyre::eyre::Result;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
