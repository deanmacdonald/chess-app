use crate::board::Board;
use crate::movegen::generate_moves;
use crate::moves::MoveList;
use crate::attack::is_square_attacked;
use crate::make_move::make_move;
use crate::pieces::{Color, Piece, piece_index};

/// Generate only LEGAL moves (king not left in check)
pub fn generate_legal_moves(board: &Board) -> MoveList {
    let pseudo = generate_moves(board);
    let mut legal = MoveList::new();

    for m in pseudo.moves {
        let mut new_board = board.clone();
        make_move(&mut new_board, m);

        if !king_in_check(&new_board, board.side_to_move) {
            legal.push(m);
        }
    }

    legal
}

/// Check if the king of a given color is in check
pub fn king_in_check(board: &Board, color: Color) -> bool {
    let king_bb = board.pieces[piece_index(color, Piece::King)];
    if king_bb == 0 {
        return false; // shouldn't happen
    }

    let king_sq = king_bb.trailing_zeros() as u8;
    is_square_attacked(board, king_sq, color.opposite())
}
