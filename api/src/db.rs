use crate::error::ApiError;
use api_types::resources::{CreateResource, Resource, UpdateResource};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};

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
            .map_err(|error| ApiError::Database(error.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), ApiError> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS resources (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )"#,
        )
        .execute(&self.pool)
        .await
        .map_err(|error| ApiError::Database(error.to_string()))?;

        Ok(())
    }

    pub async fn list_resources(&self) -> Result<Vec<Resource>, ApiError> {
        let rows = sqlx::query(
            r#"SELECT id, name, description, created_at, updated_at FROM resources ORDER BY created_at DESC"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| ApiError::Database(error.to_string()))?;

        let resources = rows
            .into_iter()
            .map(|row| Resource {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(resources)
    }

    pub async fn get_resource(&self, id: &str) -> Result<Option<Resource>, ApiError> {
        let row = sqlx::query(
            r#"SELECT id, name, description, created_at, updated_at FROM resources WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| ApiError::Database(error.to_string()))?;

        Ok(row.map(|row| Resource {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    pub async fn create_resource(&self, input: CreateResource) -> Result<Resource, ApiError> {
        let id = generate_id();
        let now = timestamp_string();

        sqlx::query(
            r#"INSERT INTO resources (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|error| ApiError::Database(error.to_string()))?;

        self.get_resource(&id)
            .await?
            .ok_or_else(|| ApiError::Database("Failed to retrieve created resource".to_string()))
    }

    pub async fn update_resource(
        &self,
        id: &str,
        input: UpdateResource,
    ) -> Result<Option<Resource>, ApiError> {
        let now = timestamp_string();

        let result = sqlx::query(
            r#"UPDATE resources SET name = COALESCE(?, name), description = COALESCE(?, description), updated_at = ? WHERE id = ?"#,
        )
        .bind(&input.name)
        .bind(&input.description)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|error| ApiError::Database(error.to_string()))?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.get_resource(id).await
    }

    pub async fn delete_resource(&self, id: &str) -> Result<bool, ApiError> {
        let result = sqlx::query(r#"DELETE FROM resources WHERE id = ?"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|error| ApiError::Database(error.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

fn generate_id() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

fn timestamp_string() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_secs().to_string()
}
