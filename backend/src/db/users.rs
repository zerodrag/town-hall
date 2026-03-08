use color_eyre::eyre::Result;
use sqlx::{Pool, Postgres};

use crate::error::DbError;

pub async fn get_user_count(pool: &Pool<Postgres>) -> Result<i64, DbError> {
    sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&*pool)
        .await?
        .ok_or_else(|| DbError::UnexpectedValue("user count query returned no result"))
}
