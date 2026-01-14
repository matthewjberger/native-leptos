use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod router;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    dotenvy::dotenv().ok();
    let config = config::Config::from_env();
    std::fs::create_dir_all("./data").ok();
    let database = db::SqliteDatabase::new(&config.database_url).await?;
    database.migrate().await?;
    let addr: std::net::SocketAddr = config.bind_address.parse()?;
    tracing::info!("Starting server on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router::create_router(state::AppState { database, config })).await?;
    Ok(())
}
