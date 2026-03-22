use crate::board::Board;
use crate::pieces::{Color, Piece};
use crate::bitboard::bb;

pub struct PassedEval;

impl PassedEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::passed(board);
        score += Self::protected_passed(board);
        score += Self::connected_passed(board);
        score += Self::candidate_passed(board);
        score += Self::outside_passed(board);
        score += Self::blockaded(board);

        score
    }

    /// Basic passed pawn bonus (already partly implemented)
    fn passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let mask = bb::PASSED[color as usize][sq];

                if enemy & mask == 0 {
                    let rank = sq / 8;
                    let bonus = if color == Color::White {
                        rank as i32 * 15
                    } else {
                        (7 - rank as i32) * 15
                    };

                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }

    /// Protected passed pawns (supported by friendly pawn)
    fn protected_passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let mask = bb::PASSED[color as usize][sq];
                if enemy & mask != 0 {
                    continue;
                }

                // Protected by friendly pawn?
                let attacks = bb::PAWN_ATTACKS[color as usize][sq];
                if pawns & attacks != 0 {
                    let bonus = 25;
                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }

    /// Connected passed pawns (two passers next to each other)
    fn connected_passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let mask = bb::PASSED[color as usize][sq];
                if enemy & mask != 0 {
                    continue;
                }

                let file = sq % 8;

                for adj in [file.wrapping_sub(1), file + 1] {
                    if adj > 7 { continue; }

                    let adj_mask = bb::FILE[adj];
                    if pawns & adj_mask != 0 {
                        let bonus = 20;
                        score += if color == Color::White { bonus } else { -bonus };
                    }
                }
            }
        }

        score
    }

    /// Candidate passed pawns (no enemy pawn can stop them easily)
    fn candidate_passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy = board.pieces[board.index(color.opposite(), Piece::Pawn)];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let _file = sq % 8;
                let rank = sq / 8;

                // Enemy pawn in front but far away?
                let front_mask = bb::FRONT_SPANS[color as usize][sq];

                if enemy & front_mask == 0 {
                    let bonus = 10 + rank as i32 * 5;
                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }

    /// Outside passed pawns (far from enemy king)
    fn outside_passed(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy_king = board.king_square(color.opposite());

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let file = sq % 8;
                let king_file = enemy_king % 8;

                let dist = (file as i32 - king_file as i32).abs();

                if dist >= 3 {
                    let bonus = 20;
                    score += if color == Color::White { bonus } else { -bonus };
                }
            }
        }

        score
    }

    /// Blockaded passed pawns (enemy piece sits in front)
    fn blockaded(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let pawns = board.pieces[board.index(color, Piece::Pawn)];
            let enemy_pieces = board.occupied[color.opposite() as usize];

            let mut b = pawns;
            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                let front_sq = if color == Color::White {
                    sq + 8
                } else {
                    sq - 8
                };

                if front_sq < 64 && (enemy_pieces & (1u64 << front_sq)) != 0 {
                    let penalty = 20;
                    score += if color == Color::White { -penalty } else { penalty };
                }
            }
        }

        score
    }
}
