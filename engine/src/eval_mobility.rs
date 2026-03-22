use crate::board::Board;
use crate::pieces::Color;
use crate::legal::generate_legal_moves;

pub struct MobilityEval;

impl MobilityEval {
    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;

        // White mobility
        let white_moves = generate_legal_moves(&mut board.clone()).moves.len() as i32;

        // Flip side to move to get black mobility
        let mut clone = board.clone();
        clone.side_to_move = Color::Black;
        let black_moves = generate_legal_moves(&mut clone).moves.len() as i32;

        // Weight mobility
        let mobility_score = (white_moves - black_moves) * 2;

        score += mobility_score;

        score
    }
}
