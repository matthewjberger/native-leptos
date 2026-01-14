use crate::config::Config;
use crate::db::SqliteDatabase;

#[derive(Clone)]
pub struct AppState {
    pub database: SqliteDatabase,
    pub config: Config,
}

impl AppState {
    pub fn new(database: SqliteDatabase, config: Config) -> Self {
        Self { database, config }
    }
}
