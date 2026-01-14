use crate::error::ApiError;
use api_types::resources::{CreateResource, Resource, UpdateResource};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};

fn row_to_resource(row: SqliteRow) -> Resource {
    Resource {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn db_err(error: sqlx::Error) -> ApiError {
    ApiError::Database(error.to_string())
}

#[derive(Clone)]
pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(database_url: &str) -> Result<Self, ApiError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(db_err)?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), ApiError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS resources (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    pub async fn list_resources(&self) -> Result<Vec<Resource>, ApiError> {
        let rows = sqlx::query("SELECT id, name, description, created_at, updated_at FROM resources ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_resource).collect())
    }

    pub async fn get_resource(&self, id: &str) -> Result<Option<Resource>, ApiError> {
        sqlx::query("SELECT id, name, description, created_at, updated_at FROM resources WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)
            .map(|row| row.map(row_to_resource))
    }

    pub async fn create_resource(&self, input: CreateResource) -> Result<Resource, ApiError> {
        let id = generate_id();
        let now = now_secs();
        sqlx::query("INSERT INTO resources (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&id)
            .bind(&input.name)
            .bind(&input.description)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        self.get_resource(&id).await?.ok_or_else(|| ApiError::Database("Insert failed".to_string()))
    }

    pub async fn update_resource(&self, id: &str, input: UpdateResource) -> Result<Option<Resource>, ApiError> {
        let result = sqlx::query("UPDATE resources SET name = COALESCE(?, name), description = COALESCE(?, description), updated_at = ? WHERE id = ?")
            .bind(&input.name)
            .bind(&input.description)
            .bind(now_secs())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        if result.rows_affected() == 0 { return Ok(None); }
        self.get_resource(id).await
    }

    pub async fn delete_resource(&self, id: &str) -> Result<bool, ApiError> {
        let result = sqlx::query("DELETE FROM resources WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(result.rows_affected() > 0)
    }
}

fn generate_id() -> String {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

fn now_secs() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string()
}
