use chess::{Board, ChessMove, Square};
use std::str::FromStr;

pub struct ChessEngine {
    board: Board,
}

impl ChessEngine {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::default();
    }

    pub fn get_fen(&self) -> String {
        self.board.to_string()
    }

    pub fn load_fen(&mut self, fen: &str) {
        if let Ok(b) = fen.parse::<Board>() {
            self.board = b;
        }
    }

    pub fn make_move(&mut self, from: &str, to: &str) -> String {
        let from_sq = Square::from_str(from).unwrap();
        let to_sq = Square::from_str(to).unwrap();

        let mv = ChessMove::new(from_sq, to_sq, None);

        if self.board.legal(mv) {
            self.board = self.board.make_move_new(mv);
        }

        self.get_fen()
    }
}
