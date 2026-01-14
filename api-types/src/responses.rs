use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiListResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}
