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

    let pool = db::init().await?;
    println!("> db init complete");

    let app = Router::new().nest("/api", router::api_router(pool).await?);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("> serving on {}/api", listener.local_addr().unwrap());
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
