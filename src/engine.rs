use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece, Square};
use std::str::FromStr;

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

    pub fn make_move_strict(&mut self, from: &str, to: &str) -> (String, i64, String) {
        let from_sq = Square::from_str(from).unwrap();
        let to_sq = Square::from_str(to).unwrap();

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

    pub fn best_move(&mut self, depth: u8) -> Option<(String, String)> {
        let mut chosen: Option<ChessMove> = None;
        let mut best_score = i32::MIN + 1;
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX;

        for mv in MoveGen::new_legal(&self.board) {
            let next = self.board.make_move_new(mv);
            let score = -search(&next, depth.saturating_sub(1), -beta, -alpha);

            if score > best_score {
                best_score = score;
                chosen = Some(mv);
                alpha = alpha.max(score);
            }
        }

        chosen.map(|mv| {
            (
                format!("{}", mv.get_source()),
                format!("{}", mv.get_dest()),
            )
        })
    }
}

fn search(board: &Board, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();

    if moves.is_empty() {
        return if board.checkers().0 != 0 {
            -10000 - depth as i32
        } else {
            0
        };
    }

    let mut best = i32::MIN + 1;

    for mv in moves {
        let next = board.make_move_new(mv);
        let score = -search(&next, depth - 1, -beta, -alpha);

        best = best.max(score);
        alpha = alpha.max(score);

        if alpha >= beta {
            break;
        }
    }

    best
}

fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for square in chess::ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let value = match piece {
                Piece::Pawn => 100,
                Piece::Knight => 320,
                Piece::Bishop => 330,
                Piece::Rook => 500,
                Piece::Queen => 900,
                Piece::King => 0,
            };

            if board.color_on(square) == Some(Color::White) {
                score += value;
            } else {
                score -= value;
            }
        }
    }

    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn save_fen_to_db(fen: &str) -> i64 {
    match try_save_fen_to_db(fen) {
        Ok(id) => id,
        Err(error) => {
            eprintln!("Failed to save chess position to SQLite: {error}");
            -1
        }
    }
}

fn try_save_fen_to_db(fen: &str) -> rusqlite::Result<i64> {
    let conn = Connection::open("chess.db")?;

    conn.busy_timeout(std::time::Duration::from_secs(2))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fen TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute("INSERT INTO games (fen) VALUES (?1)", params![fen])?;

    Ok(conn.last_insert_rowid())
}
