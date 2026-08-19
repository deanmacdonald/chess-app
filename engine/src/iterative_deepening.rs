use crate::board::Board;
use crate::search::alpha_beta;
use crate::search::SearchInfo;
use crate::transposition_table::TranspositionTable;
use crate::zobrist::Zobrist;
use crate::move_ordering::{KillerMoves, HistoryHeuristic};
use crate::moves::Move;

/// Iterative deepening search
pub fn iterative_deepening(
    board: &mut Board,
    max_depth: i32,
    tt: &mut TranspositionTable,
    zob: &Zobrist,
) -> (Option<Move>, i32, SearchInfo) {
    let mut best_move: Option<Move> = None;
    let mut best_score = 0;
    let mut info = SearchInfo::default();

    let mut killers = KillerMoves::new(128);
    let mut history = HistoryHeuristic::new();

    for depth in 1..=max_depth {
        let score = alpha_beta(
            board,
            depth,
            -1_000_000,
            1_000_000,
            &mut info,
            tt,
            zob,
            &mut killers,
            &mut history,
            0,
        );

        // Probe TT for best move at this depth
        let key = zob.hash(board);
        if let Some((_, Some(m))) = tt.probe(key, depth, -1_000_000, 1_000_000) {
            best_move = Some(m);
            best_score = score;
        }

        // Optional: print progress for debugging
        // println!("info depth {} score {} nodes {}", depth, score, info.nodes);
    }

    (best_move, best_score, info)
}
