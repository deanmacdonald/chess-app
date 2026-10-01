pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine
    }

    pub fn best_move(&mut self, _fen: &str) -> String {
        // TODO: replace with real search
        "e2e4".to_string()
    }

    pub fn evaluate(&mut self, _fen: &str) -> String {
        // TODO: replace with real evaluation
        "+0.15".to_string()
    }

    pub fn fen_to_pgn(&mut self, _fen: &str) -> String {
        // TODO: replace with real PGN conversion
        "[Event \"Custom\"]\n1. e4 e5 2. Nf3 Nc6".to_string()
    }
}
