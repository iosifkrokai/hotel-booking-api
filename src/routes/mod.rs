pub mod health;
use crate::models;
use axum::{Router, routing};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Hotel Booking API", version = "0.1.0"),
    paths(health::live, health::ready),
    tags(
        (name = "health", description = "Health check endpoints")
    )
)]
pub struct ApiDoc;

pub fn create_routers(state: models::app_state::AppState) -> Router {
    let openapi = ApiDoc::openapi();

    Router::new()
        .route("/health/live", routing::get(health::live))
        .route("/health/ready", routing::get(health::ready))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .with_state(state)
}
