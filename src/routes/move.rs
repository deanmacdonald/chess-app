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

    let valid_square = |square: &str| {
        let bytes = square.as_bytes();

        bytes.len() == 2
            && (b'a'..=b'h').contains(&bytes[0])
            && (b'1'..=b'8').contains(&bytes[1])
    };

    if !valid_square(&req.from) || !valid_square(&req.to) {
        return Json(json!({
            "result": "Invalid square",
            "message": "Square names must be between a1 and h8.",
            "fen": s.engine.get_fen(),
            "turn": s.engine.get_turn(),
            "game_status": s.engine.get_status()
        }));
    }

    if s.engine.get_status() != "Ongoing" {
        return Json(json!({
            "result": "Game over",
            "message": "The game has ended. Start a new game.",
            "fen": s.engine.get_fen(),
            "turn": s.engine.get_turn(),
            "game_status": s.engine.get_status()
        }));
    }

    if s.engine.get_turn() != "White" {
        return Json(json!({
            "result": "Computer turn",
            "message": "It is the computer's turn. Reset the game if needed.",
            "fen": s.engine.get_fen(),
            "turn": s.engine.get_turn(),
            "game_status": s.engine.get_status()
        }));
    }

    let (human_fen, human_id, result) =
        s.engine.make_move_strict(&req.from, &req.to);

    if result != "OK" {
        return Json(json!({
            "result": result,
            "message": "That move is not legal.",
            "fen": human_fen,
            "id": human_id,
            "turn": s.engine.get_turn(),
            "game_status": s.engine.get_status()
        }));
    }

    let mut final_id = human_id;
    let mut computer_move: Option<String> = None;

    if s.engine.get_status() == "Ongoing"
        && s.engine.get_turn() == "Black"
    {
        if let Some((from, to)) = s.engine.best_move(3) {
            let (_, computer_id, computer_result) =
                s.engine.make_move_strict(&from, &to);

            if computer_result == "OK" {
                final_id = computer_id;
                computer_move = Some(format!("{}{}", from, to));
            } else {
                return Json(json!({
                    "result": "Computer move failed",
                    "message": "Your move was saved, but the computer could not reply.",
                    "fen": s.engine.get_fen(),
                    "id": human_id,
                    "turn": s.engine.get_turn(),
                    "game_status": s.engine.get_status()
                }));
            }
        }
    }

    Json(json!({
        "result": "OK",
        "fen": s.engine.get_fen(),
        "id": final_id,
        "turn": s.engine.get_turn(),
        "game_status": s.engine.get_status(),
        "computer_move": computer_move
    }))
}
