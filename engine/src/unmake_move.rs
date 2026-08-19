use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::bb;
use crate::moves::{
    Move, move_from, move_to, move_promo,
    move_is_capture, move_is_ep, move_is_castling
};

/// Stores reversible state for unmake_move
#[derive(Clone)]
pub struct MoveState {
    pub castling: u8,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u8,
    pub captured_piece: Option<(Color, Piece)>,
}

/// Apply a move and return the reversible state
pub fn make_move_with_state(board: &mut Board, m: Move) -> MoveState {
    let state = MoveState {
        castling: board.castling,
        en_passant: board.en_passant,
        halfmove_clock: board.halfmove_clock,
        captured_piece: captured_piece(board, m),
    };

    crate::make_move::make_move(board, m);
    state
}

/// Undo a move using the saved state
pub fn unmake_move(board: &mut Board, m: Move, state: MoveState) {
    let from = move_from(m);
    let to = move_to(m);
    let promo = move_promo(m);
    let is_ep = move_is_ep(m);
    let is_castle = move_is_castling(m);

    let side = board.side_to_move.opposite(); // we already switched sides in make_move
    let _enemy = board.side_to_move;

    // Restore board state
    board.castling = state.castling;
    board.en_passant = state.en_passant;
    board.halfmove_clock = state.halfmove_clock;

    // ------------------------------
    // 1. Identify moved piece
    // ------------------------------
    let moved_piece = if promo != 0 {
        // Promotion: piece on 'to' is promoted piece
        match promo {
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            _ => Piece::Pawn,
        }
    } else {
        // Normal move: find piece at 'to'
        board.piece_at(to).unwrap().1
    };

    let moved_idx = piece_index(side, moved_piece);

    // Remove piece from destination
    board.pieces[moved_idx] &= !bb(to);

    // Restore original piece (pawn if promotion)
    let original_piece = if promo != 0 { Piece::Pawn } else { moved_piece };
    let original_idx = piece_index(side, original_piece);

    // Place piece back on source
    board.pieces[original_idx] |= bb(from);

    // ------------------------------
    // 2. Restore captured piece
    // ------------------------------
    if let Some((cap_color, cap_piece)) = state.captured_piece {
        let cap_idx = piece_index(cap_color, cap_piece);

        if is_ep {
            // En passant restore
            let cap_sq = match side {
                Color::White => to - 8,
                Color::Black => to + 8,
            };
            board.pieces[cap_idx] |= bb(cap_sq);
        } else {
            board.pieces[cap_idx] |= bb(to);
        }
    }

    // ------------------------------
    // 3. Undo castling rook move
    // ------------------------------
    if is_castle {
        match (side, to) {
            (Color::White, 6) => { // White king-side
                board.pieces[piece_index(Color::White, Piece::Rook)] &= !bb(5);
                board.pieces[piece_index(Color::White, Piece::Rook)] |= bb(7);
            }
            (Color::White, 2) => { // White queen-side
                board.pieces[piece_index(Color::White, Piece::Rook)] &= !bb(3);
                board.pieces[piece_index(Color::White, Piece::Rook)] |= bb(0);
            }
            (Color::Black, 62) => { // Black king-side
                board.pieces[piece_index(Color::Black, Piece::Rook)] &= !bb(61);
                board.pieces[piece_index(Color::Black, Piece::Rook)] |= bb(63);
            }
            (Color::Black, 58) => { // Black queen-side
                board.pieces[piece_index(Color::Black, Piece::Rook)] &= !bb(59);
                board.pieces[piece_index(Color::Black, Piece::Rook)] |= bb(56);
            }
            _ => {}
        }
    }

    // ------------------------------
    // 4. Switch side back
    // ------------------------------
    board.side_to_move = side;
}

/// Determine which piece was captured (if any)
fn captured_piece(board: &Board, m: Move) -> Option<(Color, Piece)> {
    if !move_is_capture(m) {
        return None;
    }

    let to = move_to(m);
    let is_ep = move_is_ep(m);
    let side = board.side_to_move;
    let enemy = side.opposite();

    if is_ep {
        return Some((enemy, Piece::Pawn));
    }

    for p in Piece::ALL {
        let idx = piece_index(enemy, p);
        if board.pieces[idx] & bb(to) != 0 {
            return Some((enemy, p));
        }
    }

    None
}
