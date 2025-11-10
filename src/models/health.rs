use serde::Serialize;

#[derive(Serialize)]
pub struct HealthLiveResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct HealthServiceResponse {
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct HealthReadyResponse {
    pub status: String,
    pub services: Vec<HealthServiceResponse>,
}
