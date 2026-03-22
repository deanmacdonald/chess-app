use crate::board::Board;
use crate::pieces::{Color, Piece};
use crate::bitboard::{bb, popcount};

pub struct KingEval;

impl KingEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::pawn_shield(board);
        score += Self::open_files_near_king(board);
        score += Self::tropism(board);

        score
    }

    /// Pawn shield around the king
    fn pawn_shield(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let king_sq = board.king_square(color);
            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            let shield = bb::KING_SHIELD[color as usize][king_sq];
            let count = popcount(pawns & shield);

            // Reward good pawn cover
            let val = count as i32 * 15;

            score += if color == Color::White { val } else { -val };
        }

        score
    }

    /// Penalty for open/semi-open files near the king
    fn open_files_near_king(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let king_sq = board.king_square(color);
            let file = king_sq % 8;

            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            for f in file.saturating_sub(1)..=file.saturating_add(1) {
                if f > 7 { continue; }

                let mask = bb::FILE[f];
                let friendly = pawns & mask;

                if friendly == 0 {
                    // Semi-open or open file near king
                    let penalty = 20;
                    score += if color == Color::White { -penalty } else { penalty };
                }
            }
        }

        score
    }

    /// Tropism: enemy piece proximity to king
    fn tropism(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let enemy = color.opposite();
            let king_sq = board.king_square(color);

            for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                let bb = board.pieces[board.index(enemy, piece)];

                let mut b = bb;
                while b != 0 {
                    let sq = b.trailing_zeros() as usize;
                    b &= b - 1;

                    let dist = Self::manhattan(king_sq, sq);

                    let weight = match piece {
                        Piece::Queen => 12,
                        Piece::Rook => 8,
                        Piece::Bishop => 6,
                        Piece::Knight => 6,
                        _ => 0,
                    };

                    let val = weight * (7 - dist as i32);

                    score += if color == Color::White { -val } else { val };
                }
            }
        }

        score
    }

    #[inline]
    fn manhattan(a: usize, b: usize) -> i32 {
        let (ax, ay) = (a % 8, a / 8);
        let (bx, by) = (b % 8, b / 8);
        (ax as i32 - bx as i32).abs() + (ay as i32 - by as i32).abs() 
    }
}
