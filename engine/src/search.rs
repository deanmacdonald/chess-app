use crate::board::Board;
use crate::legal::generate_legal_moves;
use crate::unmake_move::{make_move_with_state, unmake_move};
use crate::pieces::piece_index;
use crate::moves::{Move, move_is_capture, move_to, move_from};
use crate::zobrist::Zobrist;
use crate::transposition_table::{TranspositionTable, Bound};
use crate::move_ordering::{KillerMoves, HistoryHeuristic, mvv_lva_score};
use crate::quiescence::quiescence;
use crate::nullmove::{nullmove_allowed, make_null_move, unmake_null_move};

#[derive(Default, Clone)]
pub struct SearchInfo {
    pub nodes: u64,
}

pub fn alpha_beta(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    info: &mut SearchInfo,
    tt: &mut TranspositionTable,
    zob: &Zobrist,
    killers: &mut KillerMoves,
    history: &mut HistoryHeuristic,
    ply: usize,
) -> i32 {
    info.nodes += 1;

    let key = zob.hash(board);

    if let Some((tt_score, _)) = tt.probe(key, depth, alpha, beta) {
        return tt_score;
    }

    if depth <= 0 {
        return quiescence(board, alpha, beta, info);
    }

    // ------------------------------
    // NULL-MOVE PRUNING
    // ------------------------------
    if depth >= 3 && nullmove_allowed(board) {
        let (old_ep, old_side) = make_null_move(board);

        let r = 2;
        let score = -alpha_beta(
            board,
            depth - 1 - r,
            -beta,
            -beta + 1,
            info,
            tt,
            zob,
            killers,
            history,
            ply + 1,
        );

        unmake_null_move(board, old_ep, old_side);

        if score >= beta {
            return beta;
        }
    }

    let moves = generate_legal_moves(board);

    if moves.is_empty() {
        return quiescence(board, alpha, beta, info);
    }

    // Score moves
    let mut scored_moves: Vec<(Move, i32)> = moves
        .moves
        .iter()
        .map(|&m| {
            let score = if move_is_capture(m) {
                10_000 + mvv_lva_score(m)
            } else if killers.is_killer(ply, m) {
                9_000
            } else {
                let piece = board.piece_at(move_from(m)).unwrap().1;
                let idx = piece_index(board.side_to_move, piece);
                history.score(idx, move_to(m) as usize)
            };
            (m, score)
        })
        .collect();

    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    let mut best_score = -1_000_000;
    let mut best_move: Option<Move> = None;
    let orig_alpha = alpha;

    // ------------------------------
    // MOVE LOOP WITH LMR
    // ------------------------------
    for (index, (m, _s)) in scored_moves.iter().enumerate() {
        let m = *m;

        let state = make_move_with_state(board, m);

        let mut score;

        // ------------------------------
        // LMR CONDITIONS
        // ------------------------------
        let can_reduce =
            depth >= 3 &&
            index >= 3 &&                 // late move
            !move_is_capture(m) &&        // quiet move
            !killers.is_killer(ply, m);   // not a killer

        if can_reduce {
            // Reduced-depth search
            let reduced_depth = depth - 2;

            score = -alpha_beta(
                board,
                reduced_depth,
                -alpha - 1,
                -alpha,
                info,
                tt,
                zob,
                killers,
                history,
                ply + 1,
            );

            // If it improves alpha, re-search at full depth
            if score > alpha {
                score = -alpha_beta(
                    board,
                    depth - 1,
                    -beta,
                    -alpha,
                    info,
                    tt,
                    zob,
                    killers,
                    history,
                    ply + 1,
                );
            }
        } else {
            // Normal full-depth search
            score = -alpha_beta(
                board,
                depth - 1,
                -beta,
                -alpha,
                info,
                tt,
                zob,
                killers,
                history,
                ply + 1,
            );
        }

        unmake_move(board, m, state);

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }

        if score > alpha {
            alpha = score;

            if !move_is_capture(m) {
                let piece = board.piece_at(move_from(m)).unwrap().1;
                let idx = piece_index(board.side_to_move, piece);
                history.add(idx, move_to(m) as usize, depth);
            }
        }

        if alpha >= beta {
            if !move_is_capture(m) {
                killers.add(ply, m);
            }
            break;
        }
    }

    let bound = if best_score <= orig_alpha {
        Bound::Alpha
    } else if best_score >= beta {
        Bound::Beta
    } else {
        Bound::Exact
    };

    tt.store(key, depth, best_score, bound, best_move);

    best_score
}
