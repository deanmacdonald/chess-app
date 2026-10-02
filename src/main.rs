use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod engine;
mod state;

use state::AppState;

type SharedState = Arc<Mutex<AppState>>;

// GET /status
async fn status_handler() -> &'static str {
    "OK"
}

// GET /health
async fn health_handler() -> &'static str {
    "OK"
}

// GET /configure
async fn configure_handler() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Chess backend is running"
    }))
}

#[tokio::main]
async fn main() {
    let app_state: SharedState = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/status", get(status_handler))
        .route("/health", get(health_handler))
        .route("/fen", get(get_fen))
        .route("/reset", post(reset_game))
        .route("/move", post(apply_move))
        .route("/load_fen", post(load_fen))
        .route("/configure", get(configure_handler))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind server to port 3000");

    println!("Chess backend running at http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("server error");
}

// GET /fen -> return JSON: { "fen": "..." }
async fn get_fen(State(state): State<SharedState>) -> Json<Value> {
    let fen = state.lock().expect("state mutex poisoned").get_fen();

    Json(json!({ "fen": fen }))
}

// POST /reset -> reset game and return the new FEN
async fn reset_game(State(state): State<SharedState>) -> Json<Value> {
    let mut app_state = state.lock().expect("state mutex poisoned");

    app_state.reset_game();

    Json(json!({
        "fen": app_state.get_fen()
    }))
}

#[derive(Deserialize)]
struct MoveReq {
    from: Option<String>,
    to: Option<String>,
}

// POST /move
async fn apply_move(
    State(state): State<SharedState>,
    Json(req): Json<MoveReq>,
) -> (StatusCode, Json<Value>) {
    let from = match req.from {
        Some(value) => value.trim().to_ascii_lowercase(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "missing required field: from" })),
            );
        }
    };

    let to = match req.to {
        Some(value) => value.trim().to_ascii_lowercase(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "missing required field: to" })),
            );
        }
    };

    if !is_square(&from) || !is_square(&to) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "from and to must be valid chess squares, for example e2 and e4"
            })),
        );
    }

    let mut app_state = state.lock().expect("state mutex poisoned");

    let move_result = app_state.apply_move_algebraic(&from, &to);

    (
        StatusCode::OK,
        Json(json!({
            "result": move_result,
            "from": from,
            "to": to,
            "fen": app_state.get_fen()
        })),
    )
}

#[derive(Deserialize)]
struct FenReq {
    fen: String,
}

// POST /load_fen -> load a FEN into backend state
async fn load_fen(
    State(state): State<SharedState>,
    Json(req): Json<FenReq>,
) -> (StatusCode, Json<Value>) {
    let fen = req.fen.trim();

    if fen.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "fen must not be empty"
            })),
        );
    }

    let mut app_state = state.lock().expect("state mutex poisoned");

    app_state.load_fen(fen);

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "fen": app_state.get_fen()
        })),
    )
}

fn is_square(square: &str) -> bool {
    let bytes = square.as_bytes();

    bytes.len() == 2 && matches!(bytes[0], b'a'..=b'h') && matches!(bytes[1], b'1'..=b'8')
}
