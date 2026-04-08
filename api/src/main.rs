use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use engine::{
    fen::from_fen,
    search_best_move::search_best_move,
};

/* ---------------------------------------------------------
   DATA STRUCTURES
--------------------------------------------------------- */

#[derive(Serialize)]
struct GameState {
    fen: String,
    turn: String,
    captured_white: Vec<String>,
    captured_black: Vec<String>,
    white_time: u32,
    black_time: u32,
    game_over: bool,
}

#[derive(Deserialize)]
struct MoveReq {
    from: String,
    to: String,
}

#[derive(Deserialize)]
struct BestMoveReq {
    fen: String,
}

/* ---------------------------------------------------------
   ENDPOINTS
--------------------------------------------------------- */

// GET /state
async fn get_state() -> Json<GameState> {
    Json(GameState {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        turn: "w".to_string(),
        captured_white: vec![],
        captured_black: vec![],
        white_time: 300,
        black_time: 300,
        game_over: false,
    })
}

// POST /move
async fn make_move(Json(req): Json<MoveReq>) -> Json<GameState> {
    // TODO: integrate your engine move logic here

    Json(GameState {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        turn: "b".to_string(),
        captured_white: vec![],
        captured_black: vec![],
        white_time: 300,
        black_time: 300,
        game_over: false,
    })
}

// GET /legal-moves
async fn legal_moves() -> Json<Vec<String>> {
    // TODO: integrate engine movegen
    Json(vec![
        "e4".to_string(),
        "d4".to_string(),
        "c4".to_string(),
        "Nf3".to_string(),
    ])
}

// POST /best-move
async fn best_move(Json(req): Json<BestMoveReq>) -> Json<String> {
    let mut board = from_fen(&req.fen).unwrap();
    let result = search_best_move(&mut board, 4);

    let mv = match result {
        Some((best_move, _score, _info)) => best_move.to_string(),
        None => "none".to_string(),
    };

    Json(mv)
}

/* ---------------------------------------------------------
   MAIN SERVER
--------------------------------------------------------- */

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/state", get(get_state))
        .route("/move", post(make_move))
        .route("/legal-moves", get(legal_moves))
        .route("/best-move", post(best_move));

    println!("API running on http://0.0.0.0:8000");

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
