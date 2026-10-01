use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SaveRequest {
    pub fen: String,
}

#[derive(Serialize)]
pub struct GameRow {
    pub id: i64,
    pub fen: String,
    pub created_at: String,
}
