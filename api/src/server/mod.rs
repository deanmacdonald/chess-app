use axum::Router;
use sqlx::{Pool, Postgres};
use crate::routes::routes;
use crate::db::get_pool;

pub async fn run() {
    let pool: Pool<Postgres> = get_pool().await;

    let app = routes(pool);

    let addr = "0.0.0.0:8000";
    println!("API running on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
