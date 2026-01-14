use crate::error::ApiError;
use crate::state::AppState;
use api_types::resources::{CreateResource, Resource, UpdateResource};
use api_types::responses::{ApiListResponse, ApiResponse, HealthResponse};
use axum::extract::{Path, State};
use axum::Json;

type ListResult = Result<Json<ApiListResponse<Resource>>, ApiError>;
type ItemResult = Result<Json<ApiResponse<Resource>>, ApiError>;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into(), version: env!("CARGO_PKG_VERSION").into() })
}

pub async fn list_resources(State(state): State<AppState>) -> ListResult {
    let data = state.database.list_resources().await?;
    Ok(Json(ApiListResponse { total: data.len(), data }))
}

pub async fn get_resource(State(state): State<AppState>, Path(id): Path<String>) -> ItemResult {
    Ok(Json(ApiResponse { data: state.database.get_resource(&id).await?.ok_or(ApiError::NotFound)? }))
}

pub async fn create_resource(State(state): State<AppState>, Json(input): Json<CreateResource>) -> ItemResult {
    Ok(Json(ApiResponse { data: state.database.create_resource(input).await? }))
}

pub async fn update_resource(State(state): State<AppState>, Path(id): Path<String>, Json(input): Json<UpdateResource>) -> ItemResult {
    Ok(Json(ApiResponse { data: state.database.update_resource(&id, input).await?.ok_or(ApiError::NotFound)? }))
}

pub async fn delete_resource(State(state): State<AppState>, Path(id): Path<String>) -> Result<(), ApiError> {
    if !state.database.delete_resource(&id).await? { return Err(ApiError::NotFound); }
    Ok(())
}
