use crate::board::Board;
use crate::pieces::{Color, Piece};
use crate::bitboard::{bb, popcount};

pub struct PawnEval {
    pub score: i32,
}

impl PawnEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::isolated(board);
        score += Self::doubled(board);
        score += Self::passed(board);
        score += Self::pawn_shield(board);

        score
    }

    fn isolated(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let file = sq % 8;

                let left = if file > 0 { file - 1 } else { 8 };
                let right = if file < 7 { file + 1 } else { 8 };

                let mut isolated = true;

                for f in [left, right] {
                    if f < 8 {
                        let mask = bb::FILE[f];
                        if pawns & mask != 0 {
                            isolated = false;
                        }
                    }
                }

                if isolated {
                    score += if color == Color::White { -15 } else { 15 };
                }
            }
        }

        score
    }

    fn doubled(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            for file in 0..8 {
                let mask = bb::FILE[file];
                let count = popcount(pawns & mask);

                if count > 1 {
                    score += if color == Color::White {
                        -10 * (count as i32 - 1)
                    } else {
                        10 * (count as i32 - 1)
                    };
                }
            }
        }

        score
    }

    fn passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy_pawns = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let _file = sq % 8;

                let mask = bb::PASSED[color as usize][sq];

                if enemy_pawns & mask == 0 {
                    let rank = sq / 8;
                    let bonus = if color == Color::White {
                        rank as i32 * 10
                    } else {
                        (7 - rank as i32) * 10
                    };

                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }

    fn pawn_shield(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let king_sq = board.king_square(color);
            let pawns = board.pieces[board.index(color, Piece::Pawn)];

            let shield = bb::KING_SHIELD[color as usize][king_sq];

            let count = popcount(pawns & shield);

            let val = count as i32 * 12;

            score += if color == Color::White { val } else { -val };
        }

        score
    }
}
