use crate::board::Board;
use crate::legal::generate_legal_moves;
use crate::unmake_move::{make_move_with_state, unmake_move};
use crate::search::SearchInfo;
use crate::moves::Move;

/// Simple root search that picks a legal move and assigns a dummy score.
/// You can wire this back into full alpha-beta later.
pub fn search_best_move(board: &mut Board, _depth: i32) -> Option<(Move, i32, SearchInfo)> {
    let moves = generate_legal_moves(board);
    if moves.is_empty() {
        return None;
    }

    let mut best_move = moves.moves[0];
    let mut best_score = 0;
    let info = SearchInfo::default();

    for m in moves.moves {
        let state = make_move_with_state(board, m);

        // Placeholder: no deep search yet.
        let score = 0;

        unmake_move(board, m, state);

        if score > best_score {
            best_score = score;
            best_move = m;
        }
    }

    Some((best_move, best_score, info))
}
