use crate::board::Board;
use crate::movegen::generate_moves;
use crate::moves::{move_from, move_to, Move};

/// Run perft and return total node count
pub fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(board);
    let mut nodes = 0;

    for m in moves.moves {
        let mut new_board = board.clone();
        make_move(&mut new_board, m);

        nodes += perft(&new_board, depth - 1);
    }

    nodes
}

/// Print perft breakdown (divide)
pub fn divide(board: &Board, depth: u32) {
    let moves = generate_moves(board);
    let mut total = 0;

    for m in moves.moves {
        let mut new_board = board.clone();
        make_move(&mut new_board, m);

        let count = perft(&new_board, depth - 1);
        total += count;

        println!("{}{}: {}", square_name(move_from(m)), square_name(move_to(m)), count);
    }

    println!("Total: {}", total);
}

/// Temporary make_move stub (will be replaced by full make_move.rs)
fn make_move(board: &mut Board, m: Move) {
    let from = move_from(m);
    let to = move_to(m);

    // Remove piece from source
    for bb in board.pieces.iter_mut() {
        *bb &= !(1u64 << from);
    }

    // Remove captured piece
    for bb in board.pieces.iter_mut() {
        *bb &= !(1u64 << to);
    }

    // Add piece to destination
    // NOTE: This is a placeholder — real make_move will handle promotions, EP, castling
    if let Some((color, piece)) = board.piece_at(from) {
        let idx = crate::pieces::piece_index(color, piece);
        board.pieces[idx] |= 1u64 << to;
    }

    // Switch side
    board.side_to_move = board.side_to_move.opposite();
}

/// Convert square index to algebraic (e.g., 0 -> a1)
fn square_name(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;

    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;

    format!("{}{}", file_char, rank_char)
}
