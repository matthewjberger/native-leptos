use crate::error::ApiError;
use crate::state::AppState;
use api_types::resources::{CreateResource, Resource, UpdateResource};
use api_types::responses::{ApiListResponse, ApiResponse, HealthResponse};
use axum::extract::{Path, State};
use axum::Json;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn list_resources(
    State(state): State<AppState>,
) -> Result<Json<ApiListResponse<Resource>>, ApiError> {
    let resources = state.database.list_resources().await?;
    let total = resources.len();
    Ok(Json(ApiListResponse {
        data: resources,
        total,
    }))
}

pub async fn get_resource(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Resource>>, ApiError> {
    let resource = state
        .database
        .get_resource(&id)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(ApiResponse { data: resource }))
}

pub async fn create_resource(
    State(state): State<AppState>,
    Json(input): Json<CreateResource>,
) -> Result<Json<ApiResponse<Resource>>, ApiError> {
    let resource = state.database.create_resource(input).await?;
    Ok(Json(ApiResponse { data: resource }))
}

pub async fn update_resource(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<UpdateResource>,
) -> Result<Json<ApiResponse<Resource>>, ApiError> {
    let resource = state
        .database
        .update_resource(&id, input)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(ApiResponse { data: resource }))
}

pub async fn delete_resource(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<()>, ApiError> {
    let deleted = state.database.delete_resource(&id).await?;
    if !deleted {
        return Err(ApiError::NotFound);
    }
    Ok(Json(()))
}
