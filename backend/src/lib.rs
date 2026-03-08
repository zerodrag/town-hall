mod db;
mod error;
mod handlers;
mod models;
mod router;

use color_eyre::eyre::Result;

pub async fn start_server() -> Result<()> {
    use axum::Router;
    use tokio::net::TcpListener;

    dotenvy::dotenv()?;

    let app = Router::new().nest("/api", router::api_router().await?);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn ts_bindgen() -> Result<String> {
    use specta_typescript::{BigIntExportBehavior, Typescript};

    let types = specta::collect();
    let bindings = Typescript::default()
        .bigint(BigIntExportBehavior::String)
        .export(&types)?;
    Ok(bindings)
}
