use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthLiveResponse {
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct HealthServiceResponse {
    pub name: String,
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct HealthReadyResponse {
    pub status: String,
    pub services: Vec<HealthServiceResponse>,
}
