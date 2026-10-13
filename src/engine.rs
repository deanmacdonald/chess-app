use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece, Square};
use std::str::FromStr;

// SQLite support
use rusqlite::{params, Connection};

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
        let fen = self.board.to_string();
        save_fen_to_db(&fen);
    }

    pub fn get_fen(&self) -> String {
        self.board.to_string()
    }

    pub fn load_fen(&mut self, fen: &str) {
        if let Ok(b) = fen.parse::<Board>() {
            self.board = b;
            let fen = self.board.to_string();
            save_fen_to_db(&fen);
        }
    }

    pub fn get_turn(&self) -> String {
        match self.board.side_to_move() {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        }
    }

    pub fn get_status(&self) -> String {
        match self.board.status() {
            BoardStatus::Ongoing => "Ongoing".to_string(),
            BoardStatus::Checkmate => "Checkmate".to_string(),
            BoardStatus::Stalemate => "Stalemate".to_string(),
        }
    }

    pub fn get_legal_moves(&self) -> Vec<String> {
        MoveGen::new_legal(&self.board)
            .map(|m| m.to_string())
            .collect()
    }

    pub fn is_legal_move(&self, from: &str, to: &str) -> bool {
        let from_sq = Square::from_str(from).unwrap();
        let to_sq = Square::from_str(to).unwrap();

        let mv = ChessMove::new(from_sq, to_sq, None);
        self.board.legal(mv)
    }

    // STRICT MOVE: legal chess only
    pub fn make_move_strict(&mut self, from: &str, to: &str) -> (String, i64, String) {
        let from_sq = Square::from_str(from).unwrap();
        let to_sq = Square::from_str(to).unwrap();

        // Auto-promote to queen if needed
        let promotion = if self.is_pawn_promotion(from_sq, to_sq) {
            Some(Piece::Queen)
        } else {
            None
        };

        let mv = ChessMove::new(from_sq, to_sq, promotion);

        if !self.board.legal(mv) {
            return (self.get_fen(), -1, "Illegal move".to_string());
        }

        self.board = self.board.make_move_new(mv);

        let fen = self.board.to_string();
        let id = save_fen_to_db(&fen);

        (fen, id, "OK".to_string())
    }

    fn is_pawn_promotion(&self, from: Square, to: Square) -> bool {
        if let Some(piece) = self.board.piece_on(from) {
            if piece == Piece::Pawn {
                let rank = to.get_rank().to_index();
                return rank == 0 || rank == 7;
            }
        }
        false
    }
}

// Save FEN and return row ID
fn save_fen_to_db(fen: &str) -> i64 {
    let conn = Connection::open("chess.db").unwrap();
    conn.execute("INSERT INTO games (fen) VALUES (?1)", params![fen])
        .unwrap();

    conn.last_insert_rowid()
}
