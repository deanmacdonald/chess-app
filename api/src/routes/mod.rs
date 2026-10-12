use axum::{http::Method, routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

pub fn routes(pool: Pool<Postgres>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            "https://chess-app-psi-cyan.vercel.app"
                .parse()
                .expect("valid frontend origin"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    Router::new()
        .route("/", get(|| async { "Chess API is running" }))
        .layer(cors)
        .with_state(pool)
}
