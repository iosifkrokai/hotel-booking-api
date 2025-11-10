use crate::{enums, models};
use axum::{
    extract::{Json, State},
    http, response,
};

#[utoipa::path(
    get,
    path = "/health/live",
    responses(
        (status = http::StatusCode::OK, description = "API is running", body = models::health::HealthLiveResponse)
    ),
    tag = "health"
)]
pub async fn live() -> impl response::IntoResponse {
    tracing::info!("GET /health/live endpoint called");

    (
        http::StatusCode::OK,
        Json(models::health::HealthLiveResponse {
            status: "ok".to_string(),
        }),
    )
}

#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = http::StatusCode::OK, description = "All services are ready", body = models::health::HealthReadyResponse),
        (status = http::StatusCode::SERVICE_UNAVAILABLE, description = "One or more services are not ready", body = models::health::HealthReadyResponse)
    ),
    tag = "health"
)]
pub async fn ready(state: State<models::app_state::AppState>) -> impl response::IntoResponse {
    tracing::info!("GET /health/ready endpoint called");

    let postgres_status = match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => enums::HealthStatus::Ok,
        Err(_) => enums::HealthStatus::Unavailable,
    };

    let redis_status = match redis::cmd("PING")
        .exec_async(&mut state.redis_conn.clone())
        .await
    {
        Ok(_) => enums::HealthStatus::Ok,
        Err(_) => enums::HealthStatus::Unavailable,
    };

    let services = vec![
        models::health::HealthServiceResponse {
            name: enums::ServiceName::Postgres.to_string(),
            status: postgres_status.to_string(),
        },
        models::health::HealthServiceResponse {
            name: enums::ServiceName::Redis.to_string(),
            status: redis_status.to_string(),
        },
    ];

    let status_code: http::StatusCode;
    let status: String;

    if services
        .iter()
        .all(|s| s.status == enums::HealthStatus::Ok.to_string())
    {
        status_code = http::StatusCode::OK;
        status = enums::HealthStatus::Ok.to_string();
    } else {
        status_code = http::StatusCode::SERVICE_UNAVAILABLE;
        status = enums::HealthStatus::Unavailable.to_string();
    };

    (
        status_code,
        Json(models::health::HealthReadyResponse { status, services }),
    )
}
