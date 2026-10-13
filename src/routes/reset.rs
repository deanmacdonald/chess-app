use axum::{extract::State, Json};
use serde_json::{json, Value};

use super::configure::SharedState;

pub async fn reset_game(State(state): State<SharedState>) -> Json<Value> {
    let mut s = state.write().await;
    s.engine.reset();

    Json(json!({
        "result": "OK",
        "fen": s.engine.get_fen(),
        "turn": s.engine.get_turn(),
        "game_status": s.engine.get_status()
    }))
}
