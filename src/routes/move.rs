use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Deserialize)]
pub struct MoveReq {
    pub from: String,
    pub to: String,
}

pub async fn apply_move_strict(
    State(state): State<SharedState>,
    Json(req): Json<MoveReq>,
) -> Json<serde_json::Value> {
    let mut s = state.write().await;
    let (fen, id, result) = s.engine.make_move_strict(&req.from, &req.to);

    Json(json!({
        "result": result,
        "fen": fen,
        "id": id,
        "turn": s.engine.get_turn(),
        "game_status": s.engine.get_status()
    }))
}
