use crate::config::Config;
use crate::db::SqliteDatabase;

#[derive(Clone)]
pub struct AppState {
    pub database: SqliteDatabase,
    pub config: Config,
}
