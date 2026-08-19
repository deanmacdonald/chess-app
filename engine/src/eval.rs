use crate::board::Board;
use crate::pieces::{Color, Piece};

use crate::eval_pawns::PawnEval;
use crate::eval_king::KingEval;
use crate::eval_mobility::MobilityEval;
use crate::eval_pst::PST;
use crate::eval_pieces::PieceEval;
use crate::eval_passed::PassedEval;
use crate::eval_space::SpaceEval;
use crate::eval_phase::Phase;
use crate::eval_endgame::EndgameEval;

pub fn evaluate(board: &Board) -> i32 {
    let mg = evaluate_mg(board);
    let eg = evaluate_eg(board);

    let phase = Phase::game_phase(board).clamp(0, 24);

    let blended = (mg * phase + eg * (24 - phase)) / 24;

    EndgameEval::scale(board, blended)
}

fn evaluate_mg(board: &Board) -> i32 {
    let mut score = 0;

    // Material
    score += material(board);

    // Pawn structure
    score += PawnEval::evaluate(board);

    // King safety (full weight in MG)
    score += KingEval::evaluate(board);

    // Mobility
    score += MobilityEval::evaluate(board);

    // Piece-square tables
    score += PST::evaluate(board);

    // Bishop pair, rook logic, etc.
    score += PieceEval::evaluate(board);

    // Advanced passed pawns
    score += PassedEval::evaluate(board);

    // Space
    score += SpaceEval::evaluate(board);

    // Side to move bonus
    if board.side_to_move == Color::White {
        score += 10;
    } else {
        score -= 10;
    }

    score
}

fn evaluate_eg(board: &Board) -> i32 {
    let mut score = 0;

    // Material
    score += material(board);

    // Passed pawns matter more in EG
    score += PassedEval::evaluate(board) * 2;

    // King safety matters less
    score += KingEval::evaluate(board) / 4;

    // Mobility matters more
    score += MobilityEval::evaluate(board) * 2;

    // Space matters less
    score += SpaceEval::evaluate(board) / 2;

    // PSTs (you can later swap to EG king PST here)
    score += PST::evaluate(board);

    // Bishop pair, rook logic still relevant
    score += PieceEval::evaluate(board);

    // Side to move bonus
    if board.side_to_move == Color::White {
        score += 10;
    } else {
        score -= 10;
    }

    score
}

fn material(board: &Board) -> i32 {
    let mut score = 0;

    for color in [Color::White, Color::Black] {
        for piece in Piece::ALL {
            let idx = board.index(color, piece);
            let count = board.pieces[idx].count_ones() as i32;

            let val = match piece {
                Piece::Pawn   => 100,
                Piece::Knight => 320,
                Piece::Bishop => 330,
                Piece::Rook   => 500,
                Piece::Queen  => 900,
                Piece::King   => 0,
            };

            score += if color == Color::White {
                val * count
            } else {
                -val * count
            };
        }
    }

    score
}
