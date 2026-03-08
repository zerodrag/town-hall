use color_eyre::eyre::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init() -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
