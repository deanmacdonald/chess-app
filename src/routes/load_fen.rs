use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Deserialize)]
pub struct FenReq {
    pub fen: String,
}

pub async fn load_fen(
    State(state): State<SharedState>,
    Json(req): Json<FenReq>,
) -> Json<serde_json::Value> {
    let mut s = state.write().await;
    s.engine.load_fen(&req.fen);

    Json(json!({
        "status": "ok",
        "fen": s.engine.get_fen(),
        "turn": s.engine.get_turn(),
        "game_status": s.engine.get_status()
    }))
}
