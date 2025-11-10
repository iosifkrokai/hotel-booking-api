pub mod health;
use crate::models;
use axum::{Router, routing};

pub fn create_routers(state: models::app_state::AppState) -> Router {
    Router::new()
        .route("/health/live", routing::get(health::live))
        .route("/health/ready", routing::get(health::ready))
        .with_state(state)
}
