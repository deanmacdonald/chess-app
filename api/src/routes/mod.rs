use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

pub fn routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "Chess API is running" }))
        .with_state(pool)
}
