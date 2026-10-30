use axum::Json;
use chess::Board;
use serde::{Deserialize, Serialize};

use crate::engine::ChessEngine;

#[derive(Deserialize)]
pub struct BestMoveRequest {
    pub fen: String,
    pub depth: Option<u8>,
}

#[derive(Serialize)]
pub struct BestMoveResponse {
    pub from: Option<String>,
    pub to: Option<String>,
    pub fen: String,
    pub status: String,
}

pub async fn best_move(
    Json(request): Json<BestMoveRequest>,
) -> Json<BestMoveResponse> {
    if request.fen.parse::<Board>().is_err() {
        return Json(BestMoveResponse {
            from: None,
            to: None,
            fen: request.fen,
            status: "Invalid FEN".to_string(),
        });
    }

    let mut engine = ChessEngine::new();
    engine.load_fen(&request.fen);

    let depth = request.depth.unwrap_or(3).clamp(1, 5);

    let Some((from, to)) = engine.best_move(depth) else {
        return Json(BestMoveResponse {
            from: None,
            to: None,
            fen: engine.get_fen(),
            status: engine.get_status(),
        });
    };

    let (fen, _, _) = engine.make_move_strict(&from, &to);

    Json(BestMoveResponse {
        from: Some(from),
        to: Some(to),
        fen,
        status: engine.get_status(),
    })
}
