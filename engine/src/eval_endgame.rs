use crate::board::Board;
use crate::pieces::{Color, Piece};

pub struct EndgameEval;

impl EndgameEval {
    pub fn scale(board: &Board, eval: i32) -> i32 {
        // 1. Insufficient material
        if Self::insufficient_material(board) {
            return 0;
        }

        // 2. Opposite-colored bishops are drawish
        if Self::opposite_bishops(board) {
            return eval / 2;
        }

        // 3. Rook + pawn vs rook is drawish unless pawn is far advanced
        if Self::rook_pawn_vs_rook(board) {
            return eval / 2;
        }

        // 4. Minor piece endgames are drawish
        if Self::minor_piece_endgame(board) {
            return eval / 2;
        }

        eval
    }

    fn insufficient_material(board: &Board) -> bool {
        let white = board.material(Color::White);
        let black = board.material(Color::Black);

        // King vs King
        if white == 0 && black == 0 {
            return true;
        }

        // King + minor vs King
        if (white == 3 && black == 0) || (black == 3 && white == 0) {
            return true;
        }

        // King + bishop vs King + bishop (same color bishops)
        let wb = board.pieces[board.index(Color::White, Piece::Bishop)];
        let bb = board.pieces[board.index(Color::Black, Piece::Bishop)];

        if wb.count_ones() == 1 && bb.count_ones() == 1 {
            let w_sq = wb.trailing_zeros() as usize;
            let b_sq = bb.trailing_zeros() as usize;

            if (w_sq + b_sq) % 2 == 0 {
                return true;
            }
        }

        false
    }

    fn opposite_bishops(board: &Board) -> bool {
        let wb = board.pieces[board.index(Color::White, Piece::Bishop)];
        let bb = board.pieces[board.index(Color::Black, Piece::Bishop)];

        if wb.count_ones() == 1 && bb.count_ones() == 1 {
            let w_sq = wb.trailing_zeros() as usize;
            let b_sq = bb.trailing_zeros() as usize;

            // Opposite colors
            return (w_sq + b_sq) % 2 == 1;
        }

        false
    }

    fn rook_pawn_vs_rook(board: &Board) -> bool {
        let wr = board.pieces[board.index(Color::White, Piece::Rook)].count_ones();
        let br = board.pieces[board.index(Color::Black, Piece::Rook)].count_ones();

        let wp = board.pieces[board.index(Color::White, Piece::Pawn)].count_ones();
        let bp = board.pieces[board.index(Color::Black, Piece::Pawn)].count_ones();

        // R+P vs R
        if wr == 1 && br == 1 && wp == 1 && bp == 0 {
            return true;
        }
        if wr == 1 && br == 1 && bp == 1 && wp == 0 {
            return true;
        }

        false
    }

    fn minor_piece_endgame(board: &Board) -> bool {
        let total_pawns =
            board.pieces[board.index(Color::White, Piece::Pawn)].count_ones() +
            board.pieces[board.index(Color::Black, Piece::Pawn)].count_ones();

        let total_rooks =
            board.pieces[board.index(Color::White, Piece::Rook)].count_ones() +
            board.pieces[board.index(Color::Black, Piece::Rook)].count_ones();

        let total_queens =
            board.pieces[board.index(Color::White, Piece::Queen)].count_ones() +
            board.pieces[board.index(Color::Black, Piece::Queen)].count_ones();

        // No rooks, no queens, few pawns → drawish
        total_rooks == 0 && total_queens == 0 && total_pawns <= 2
    }
}
