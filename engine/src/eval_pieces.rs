use crate::board::Board;
use crate::pieces::{Color, Piece};
use crate::bitboard::{bb, popcount};

pub struct PieceEval;

impl PieceEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::bishop_pair(board);
        score += Self::rook_open_files(board);
        score += Self::rook_on_7th(board);
        score += Self::rook_behind_passers(board);

        score
    }

    /// Bishop pair bonus
    fn bishop_pair(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let bishops = board.pieces[board.index(color, Piece::Bishop)];
            let count = popcount(bishops);

            if count >= 2 {
                let bonus = 40;
                score += if color == Color::White { bonus } else { -bonus };
            }
        }

        score
    }

    /// Rook on open or semi-open file
    fn rook_open_files(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let rooks = board.pieces[board.index(color, Piece::Rook)];
            let friendly_pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy_pawns = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = rooks;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let file = sq % 8;
                let mask = bb::FILE[file];

                let friendly = friendly_pawns & mask;
                let enemy = enemy_pawns & mask;

                let bonus = if friendly == 0 && enemy == 0 {
                    20 // fully open file
                } else if friendly == 0 {
                    10 // semi-open
                } else {
                    0
                };

                score += if color == Color::White { bonus } else { -bonus };
            }
        }

        score
    }

    /// Rook on opponent's 7th rank
    fn rook_on_7th(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let rooks = board.pieces[board.index(color, Piece::Rook)];

            let target_rank = if color == Color::White { 6 } else { 1 };
            let mask = bb::RANK[target_rank];

            let count = popcount(rooks & mask);

            if count > 0 {
                let bonus = 20 * count as i32;
                score += if color == Color::White { bonus } else { -bonus };
            }
        }

        score
    }

    /// Rook behind passed pawns
    fn rook_behind_passers(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let rooks = board.pieces[board.index(color, Piece::Rook)];
            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            let mut p = pawns;
            while p != 0 {
                let sq = p.trailing_zeros() as usize;
                p &= p - 1;

                let file = sq % 8;
                let mask = bb::FILE[file];

                // Rook behind pawn means rook on same file, behind it
                let behind_mask = if color == Color::White {
                    mask & bb::BELOW[sq]
                } else {
                    mask & bb::ABOVE[sq]
                };

                if rooks & behind_mask != 0 {
                    let bonus = 15;
                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }
}
