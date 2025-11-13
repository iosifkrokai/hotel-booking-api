pub mod health;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub redis_conn: redis::aio::MultiplexedConnection,
}
