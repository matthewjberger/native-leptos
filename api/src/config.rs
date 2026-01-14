use std::env;

#[derive(Clone)]
pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub auth_enabled: bool,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./data/app.db?mode=rwc".to_string());
        let bind_address = env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let auth_enabled = env::var("AUTH_ENABLED")
            .map(|value| value == "true" || value == "1")
            .unwrap_or(false);
        Self { bind_address, database_url, auth_enabled }
    }
}
