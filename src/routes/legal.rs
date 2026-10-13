use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

pub async fn get_legal_moves(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let s = state.read().await;
    Json(json!({ "legal_moves": s.engine.get_legal_moves() }))
}

#[derive(Deserialize)]
pub struct LegalReq {
    pub from: String,
    pub to: String,
}

pub async fn is_legal_move(
    State(state): State<SharedState>,
    Json(req): Json<LegalReq>,
) -> Json<serde_json::Value> {
    let s = state.read().await;
    Json(json!({ "legal": s.engine.is_legal_move(&req.from, &req.to) }))
}
