use crate::board::Board;
use crate::pieces::{Color, Piece};

pub struct Phase;

impl Phase {
    pub fn game_phase(board: &Board) -> i32 {
        // Piece weights for phase calculation
        // (Stockfish-inspired)
        let phase_values = [
            (Piece::Knight, 1),
            (Piece::Bishop, 1),
            (Piece::Rook,   2),
            (Piece::Queen,  4),
        ];

        let mut phase = 0;
        let mut total = 0;

        for (piece, val) in phase_values {
            let count_white = board.pieces[board.index(Color::White, piece)].count_ones() as i32;
            let count_black = board.pieces[board.index(Color::Black, piece)].count_ones() as i32;

            phase += (count_white + count_black) * val;
            total += 2 * val; // both sides
        }

        // Normalize to 0..24
        let phase_scaled = (phase * 24) / total.max(1);

        phase_scaled.clamp(0, 24)
    }
}
