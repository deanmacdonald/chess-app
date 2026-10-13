use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

use crate::state::AppState;
use crate::webhook;

use super::{
    fen::get_fen,
    legal::{get_legal_moves, is_legal_move},
    load_fen::load_fen,
    r#move::apply_move_strict,
    reset::reset_game,
    status::server_status,
};

pub type SharedState = Arc<RwLock<AppState>>;

pub fn build_routes(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://127.0.0.1:5173"),
            HeaderValue::from_static("http://localhost:5173"),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/status", get(server_status))
        .route("/fen", get(get_fen))
        .route("/move", post(apply_move_strict))
        .route("/reset", post(reset_game))
        .route("/legal_moves", get(get_legal_moves))
        .route("/is_legal", post(is_legal_move))
        .route("/load_fen", post(load_fen))
        .nest("/webhook", webhook::router())
        .layer(cors)
        .with_state(state)
}
