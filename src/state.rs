use crate::engine::ChessEngine;

pub struct AppState {
    engine: ChessEngine,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            engine: ChessEngine::new(),
        }
    }

    pub fn get_fen(&self) -> String {
        self.engine.get_fen()
    }

    pub fn reset_game(&mut self) {
        self.engine.reset();
    }

    pub fn apply_move_algebraic(&mut self, from: &str, to: &str) -> String {
        self.engine.make_move(from, to)
    }

    pub fn load_fen(&mut self, fen: &str) {
        self.engine.load_fen(fen);
    }
}
