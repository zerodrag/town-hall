use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    town_hall_backend::start_server().await
}
