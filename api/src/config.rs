use std::env;

#[derive(Clone)]
pub struct Config {
    pub bind_address: String,
    pub database_url: String,
    pub database_backend: DatabaseBackend,
    pub auth_enabled: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    Sqlite,
    Postgres,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_backend = match env::var("DATABASE_BACKEND").as_deref() {
            Ok("postgres") => DatabaseBackend::Postgres,
            _ => DatabaseBackend::Sqlite,
        };

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| match database_backend {
            DatabaseBackend::Sqlite => "sqlite:./data/app.db?mode=rwc".to_string(),
            DatabaseBackend::Postgres => "postgres://localhost/native_leptos".to_string(),
        });

        let bind_address =
            env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

        let auth_enabled = env::var("AUTH_ENABLED")
            .map(|value| value == "true" || value == "1")
            .unwrap_or(false);

        Ok(Self {
            bind_address,
            database_url,
            database_backend,
            auth_enabled,
        })
    }
}

#[derive(thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
}

impl std::fmt::Debug for ConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, formatter)
    }
}
