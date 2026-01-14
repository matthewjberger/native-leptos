use serde::{Deserialize, Serialize};

pub type ResourceId = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateResource {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateResource {
    pub name: Option<String>,
    pub description: Option<String>,
}
