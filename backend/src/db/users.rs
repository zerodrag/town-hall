use color_eyre::eyre::Result;
use sqlx::PgPool;

use crate::error::BackendError;

pub async fn get_user_count(pool: &PgPool) -> Result<i64, BackendError> {
    let result = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    match result {
        None => Ok(-1),
        Some(count) => Ok(count),
    }
}
