use crate::{
    board::Board,
    fen::{from_fen, to_fen},
    legal::is_legal_move,
    make_move::make_move,
    movegen::generate_moves,
    pieces::Color,
};

pub struct Game {
    pub board: Board,
}

pub struct MoveRequest {
    pub from_r: usize,
    pub from_c: usize,
    pub to_r: usize,
    pub to_c: usize,
}

pub struct MoveResult {
    pub captured_piece: Option<char>,
}

impl Game {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        match from_fen(fen) {
            Ok(board) => Ok(Self { board }),
            Err(e) => Err(format!("FEN parse error: {:?}", e)),
        }
    }

    pub fn to_fen(&self) -> String {
        to_fen(&self.board)
    }

    pub fn current_turn(&self) -> Color {
        self.board.side_to_move
    }

    pub fn is_game_over(&self) -> bool {
        let moves = generate_moves(&self.board);
        moves.is_empty()
    }

    pub fn try_move(&mut self, req: MoveRequest) -> Result<MoveResult, String> {
        let from = (req.from_r, req.from_c);
        let to = (req.to_r, req.to_c);

        // Check legality
        if !is_legal_move(&self.board, from, to) {
            return Err("Illegal move".into());
        }

        // Apply move
        let captured = self.board.get_piece(to);
        make_move(&mut self.board, from, to);

        Ok(MoveResult {
            captured_piece: captured.map(|p| p.to_char()),
        })
    }
}

