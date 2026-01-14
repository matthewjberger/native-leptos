use crate::config::Config;
use crate::db::SqliteDatabase;
use crate::router::create_router;
use crate::state::AppState;
use std::net::SocketAddr;
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
    let config = Config::from_env()?;

    std::fs::create_dir_all("./data").ok();

    let database = SqliteDatabase::new(&config.database_url).await?;
    database.migrate().await?;

    let state = AppState::new(database, config.clone());
    let app = create_router(state);

    let addr: SocketAddr = config.bind_address.parse()?;
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
