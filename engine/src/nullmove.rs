use crate::board::Board;
use crate::pieces::Color;

/// Conditions where null-move pruning is allowed.
/// For now, keep it simple and always allow it.
/// You can reintroduce sophisticated checks once Board has the needed methods.
pub fn nullmove_allowed(_board: &Board) -> bool {
    true
}

/// Make a null move: flip side to move, clear en passant
pub fn make_null_move(board: &mut Board) -> (Option<u8>, u8) {
    let old_ep = board.en_passant;
    let old_side = board.side_to_move;

    board.en_passant = None;
    board.side_to_move = match old_side {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    (old_ep, old_side as u8)
}

/// Undo null move
pub fn unmake_null_move(board: &mut Board, old_ep: Option<u8>, old_side: u8) {
    board.en_passant = old_ep;
    board.side_to_move = match old_side {
        0 => Color::White,
        1 => Color::Black,
        _ => Color::White,
    };
}
