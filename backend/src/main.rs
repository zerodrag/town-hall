use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use color_eyre::eyre::Result;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;

    // Set up state
    let shared_state = Arc::new(AppState {
        pool: init_db().await?,
    });

    let app = Router::new()
        .route("/user-count", get(get_user_count))
        .route("/health", get(|| async { StatusCode::OK }))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn init_db() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

async fn get_user_count(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    let result = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(result.to_string())
}
