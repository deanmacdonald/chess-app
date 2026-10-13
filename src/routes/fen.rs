use axum::{extract::State, Json};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

pub async fn get_fen(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let s = state.read().await;
    Json(json!({ "fen": s.engine.get_fen() }))
}
