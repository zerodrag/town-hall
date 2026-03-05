use std::error::Error;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in backend/.env")
                .as_str(),
        )
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let result = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?
        .ok_or("no result. something is set up wrong")?;

    println!("{result}");
    Ok(())
}
