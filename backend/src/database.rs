use std::{env, str::FromStr};

use anyhow::Result;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let db_options = PgConnectOptions::from_str(env::var("DATABASE_URL")?.as_str())?;
        let pool = PgPoolOptions::new().connect_with(db_options).await?;

        // Run migrations under `migrations/`
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}
