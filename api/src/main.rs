use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use engine::{
    fen::from_fen,
    game::Game,
    pieces::Color,
    search_best_move::search_best_move,
};

#[derive(Deserialize)]
struct BestMoveRequest {
    fen: String,
}

#[derive(Deserialize)]
struct MoveRequest {
    from: Coord,
    to: Coord,
    fen: String,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
struct Coord {
    r: usize,
    c: usize,
}

#[derive(Serialize)]
struct MoveResponse {
    legal: bool,
    fen: String,
    captured: Option<String>,
    turn: String,
    game_over: bool,
    reason: Option<String>,
}

async fn best_move(Json(req): Json<BestMoveRequest>) -> Json<String> {
    let mut board = from_fen(&req.fen).unwrap();
    let result = search_best_move(&mut board, 4);
    let mv = match result {
        Some((best_move, _score, _info)) => best_move.to_string(),
        None => "none".to_string(),
    };
    Json(mv)
}

async fn apply_move(Json(req): Json<MoveRequest>) -> Json<MoveResponse> {
    let mut game = match Game::from_fen(&req.fen) {
        Ok(g) => g,
        Err(e) => {
            return Json(MoveResponse {
                legal: false,
                fen: req.fen,
                captured: None,
                turn: "white".into(),
                game_over: false,
                reason: Some(format!("Invalid FEN: {}", e)),
            })
        }
    };

    let from = (req.from.r * 8 + req.from.c) as u8;
    let to   = (req.to.r   * 8 + req.to.c)   as u8;

    if game.make_move(from, to) {
        let new_fen = game.to_fen();
        let turn = match game.current_turn() {
            Color::White => "white",
            Color::Black => "black",
        };

        Json(MoveResponse {
            legal: true,
            fen: new_fen,
            captured: None,
            turn: turn.into(),
            game_over: game.is_game_over(),
            reason: None,
        })
    } else {
        Json(MoveResponse {
            legal: false,
            fen: req.fen,
            captured: None,
            turn: "white".into(),
            game_over: false,
            reason: Some("Illegal move".into()),
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/best-move", post(best_move))
        .route("/move", post(apply_move));

    println!("API running on http://0.0.0.0:8000");

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
