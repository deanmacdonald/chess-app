use crate::board::Board;
use crate::moves::{Move, encode_move};
use crate::legal::{generate_legal_moves, king_in_check};
use crate::pieces::{Color, Piece};

pub struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::startpos(),
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let board = crate::fen::from_fen(fen)?;
        Ok(Self { board })
    }

    pub fn to_fen(&self) -> String {
        crate::fen::to_fen(&self.board)
    }

    pub fn current_turn(&self) -> Color {
        self.board.side_to_move
    }

    pub fn is_game_over(&self) -> bool {
        false
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        generate_legal_moves(&self.board).moves
    }

    pub fn make_move(&mut self, from: u8, to: u8) -> bool {
        let m = encode_move(from, to, 0, false, false, false);

        if !self.legal_moves().contains(&m) {
            return false;
        }

        crate::make_move::make_move(&mut self.board, m);
        true
    }

    pub fn in_check(&self) -> bool {
        king_in_check(&self.board, self.board.side_to_move)
    }

    pub fn piece_at(&self, sq: u8) -> Option<(Color, Piece)> {
        self.board.piece_at(sq)
    }
}
