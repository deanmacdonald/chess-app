use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::Bitboard;
use crate::moves::MoveList;

/// Main entry point: generate all pseudo-legal moves
pub fn generate_moves(board: &Board) -> MoveList {
    let mut list = MoveList::new();

    match board.side_to_move {
        Color::White => generate_color_moves(board, Color::White, &mut list),
        Color::Black => generate_color_moves(board, Color::Black, &mut list),
    }

    list
}

/// Generate all moves for a given color
fn generate_color_moves(board: &Board, color: Color, list: &mut MoveList) {
    generate_pawn_moves(board, color, list);
    generate_knight_moves(board, color, list);
    generate_bishop_moves(board, color, list);
    generate_rook_moves(board, color, list);
    generate_queen_moves(board, color, list);
    generate_king_moves(board, color, list);
}

/// ------------------------------
/// Piece-specific generators
/// ------------------------------

fn generate_pawn_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: implement pawn pushes, captures, promotions, en passant
}

fn generate_knight_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: knight move generation
}

fn generate_bishop_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: sliding bishop moves
}

fn generate_rook_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: sliding rook moves
}

fn generate_queen_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: sliding queen moves (rook + bishop)
}

fn generate_king_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: king moves + castling
}

/// ------------------------------
/// Helpers
/// ------------------------------

/// Get all pieces of a given type for a color
fn piece_bb(board: &Board, color: Color, piece: Piece) -> Bitboard {
    board.pieces[piece_index(color, piece)]
}

/// Get occupancy of enemy pieces
fn enemy_occupancy(board: &Board, color: Color) -> Bitboard {
    board.occupancy_color(color.opposite())
}

/// Get occupancy of friendly pieces
fn friendly_occupancy(board: &Board, color: Color) -> Bitboard {
    board.occupancy_color(color)
}
