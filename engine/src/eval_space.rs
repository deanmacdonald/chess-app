use crate::board::Board;
use crate::pieces::Color;
use crate::bitboard::{bb, popcount};

pub struct SpaceEval;

impl SpaceEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        score += Self::center_control(board);
        score += Self::space_in_enemy_half(board);
        score += Self::cramped_penalty(board);

        score
    }

    /// Reward controlling central squares (d4, d5, e4, e5)
    fn center_control(board: &Board) -> i32 {
        let mut score = 0;

        let center = bb::CENTER;

        for color in [Color::White, Color::Black] {
            let attacks = board.attacks_for(color);
            let count = popcount(attacks & center);

            let bonus = count as i32 * 4;

            score += if color == Color::White { bonus } else { -bonus };
        }

        score
    }

    /// Reward controlling squares in the enemy half of the board
    fn space_in_enemy_half(board: &Board) -> i32 {
        let mut score = 0;

        let white_half = bb::RANK[4] | bb::RANK[5] | bb::RANK[6] | bb::RANK[7];
        let black_half = bb::RANK[0] | bb::RANK[1] | bb::RANK[2] | bb::RANK[3];

        for color in [Color::White, Color::Black] {
            let attacks = board.attacks_for(color);

            let mask = if color == Color::White { black_half } else { white_half };

            let count = popcount(attacks & mask);

            let bonus = count as i32 * 2;

            score += if color == Color::White { bonus } else { -bonus };
        }

        score
    }

    /// Penalize cramped positions (few legal moves)
    fn cramped_penalty(board: &Board) -> i32 {
        let mut score = 0;

        for color in [Color::White, Color::Black] {
            let mut clone = board.clone();
            clone.side_to_move = color;

            let moves = clone.generate_legal_moves().moves.len() as i32;

            if moves < 8 {
                let penalty = (8 - moves) * 3;
                score += if color == Color::White { -penalty } else { penalty };
            }
        }

        score
    }
}
