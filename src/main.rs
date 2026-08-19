use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
    use serde_json::json;

mod state;
mod engine;   // <-- REQUIRED so Rust sees engine.rs

use state::AppState;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/fen", get(get_fen))
        .route("/reset", post(reset_game))
        .route("/move", post(apply_move))
        .route("/load_fen", post(load_fen))
        .with_state(app_state);

    println!("API running on http://0.0.0.0:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// GET /fen → return JSON { fen: "..." }
async fn get_fen(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<serde_json::Value> {
    let fen = state.lock().unwrap().get_fen();
    Json(json!({ "fen": fen }))
}

// POST /reset → reset game and return new FEN
async fn reset_game(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<serde_json::Value> {
    let mut s = state.lock().unwrap();
    s.reset_game();
    Json(json!({ "fen": s.get_fen() }))
}

#[derive(Deserialize)]
struct MoveReq {
    from: String,
    to: String,
}

// POST /move → apply move and return new FEN
async fn apply_move(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(req): Json<MoveReq>,
) -> Json<serde_json::Value> {
    let new_fen =
        state.lock().unwrap().apply_move_algebraic(&req.from, &req.to);
    Json(json!({ "fen": new_fen }))
}

#[derive(Deserialize)]
struct FenReq {
    fen: String,
}

// POST /load_fen → load FEN into backend
async fn load_fen(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(req): Json<FenReq>,
) -> Json<&'static str> {
    state.lock().unwrap().load_fen(&req.fen);
    Json("ok")
}
