use crate::engine::ChessEngine;

pub struct AppState {
    pub engine: ChessEngine,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine: ChessEngine::new(),
        }
    }

    pub async fn get_fen(&self) -> String {
        self.engine.get_fen()
    }

    pub async fn reset_game(&mut self) {
        self.engine.reset();
    }

    pub async fn load_fen(&mut self, fen: &str) {
        self.engine.load_fen(fen);
    }

    pub async fn get_turn(&self) -> String {
        self.engine.get_turn()
    }

    pub async fn get_status(&self) -> String {
        self.engine.get_status()
    }

    pub async fn get_legal_moves(&self) -> Vec<String> {
        self.engine.get_legal_moves()
    }

    pub async fn is_legal_move(&self, from: &str, to: &str) -> bool {
        self.engine.is_legal_move(from, to)
    }

    pub async fn apply_move_strict(&mut self, from: &str, to: &str) -> (String, i64, String) {
        self.engine.make_move_strict(from, to)
    }
}
