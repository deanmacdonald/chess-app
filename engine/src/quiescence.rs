use crate::board::Board;
use crate::eval::evaluate;
use crate::legal::generate_legal_moves;
use crate::moves::move_is_capture;
use crate::unmake_move::{make_move_with_state, unmake_move};
use crate::search::SearchInfo;

/// Quiescence search: only captures (and optionally checks later)
pub fn quiescence(
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    info: &mut SearchInfo,
) -> i32 {
    info.nodes += 1;

    // Stand-pat evaluation
    let stand_pat = evaluate(board);

    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let moves = generate_legal_moves(board);

    for m in moves.moves {
        if !move_is_capture(m) {
            continue;
        }

        let state = make_move_with_state(board, m);
        let score = -quiescence(board, -beta, -alpha, info);
        unmake_move(board, m, state);

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
