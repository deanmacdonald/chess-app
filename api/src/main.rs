use axum::{Router, routing::post, Json};
use serde::Deserialize;
use tokio::net::TcpListener;

use engine::{
    fen::from_fen,
    search_best_move::search_best_move,
};

#[derive(Deserialize)]
struct MoveRequest {
    fen: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/best-move", post(best_move));

    println!("API running on http://localhost:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn best_move(Json(req): Json<MoveRequest>) -> Json<String> {
    // Parse FEN using your engine's real parser
    let mut board = from_fen(&req.fen).unwrap();

    // Call engine search with depth 4
    let result = search_best_move(&mut board, 4);

    // Convert engine output into a String
    let mv = match result {
        Some((best_move, _score, _info)) => best_move.to_string(),
        None => "none".to_string(),
    };

    Json(mv)
}
