use crate::{enums, models};
use axum::{
    extract::{Json, State},
    http, response,
};

pub async fn live() -> impl response::IntoResponse {
    tracing::info!("GET /health/live endpoint called");

    (
        http::StatusCode::OK,
        Json(models::health::HealthLiveResponse {
            status: "ok".to_string(),
        }),
    )
}

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

    (
        http::StatusCode::OK,
        Json(models::health::HealthReadyResponse {
            status: "ok".to_string(),
            services: vec![
                models::health::HealthServiceResponse {
                    name: enums::ServiceName::Postgres.to_string(),
                    status: postgres_status.to_string(),
                },
                models::health::HealthServiceResponse {
                    name: enums::ServiceName::Redis.to_string(),
                    status: redis_status.to_string(),
                },
            ],
        }),
    )
}
