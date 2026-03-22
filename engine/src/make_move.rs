use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::{bb};
use crate::moves::{Move, move_from, move_to, move_promo, move_is_capture, move_is_ep, move_is_castling};

/// Apply a move to the board (no legality checking)
pub fn make_move(board: &mut Board, m: Move) {
    let from = move_from(m);
    let to = move_to(m);

    let promo = move_promo(m);
    let is_capture = move_is_capture(m);
    let is_ep = move_is_ep(m);
    let is_castle = move_is_castling(m);

    let side = board.side_to_move;
    let enemy = side.opposite();

    // ------------------------------
    // 1. Identify moving piece
    // ------------------------------
    let (color, piece) = board.piece_at(from).expect("No piece on from-square");

    let idx = piece_index(color, piece);

    // Remove piece from source
    board.pieces[idx] &= !bb(from);

    // ------------------------------
    // 2. Handle captures
    // ------------------------------
    if is_capture {
        if is_ep {
            // En passant capture square
            let ep_target = match side {
                Color::White => to - 8,
                Color::Black => to + 8,
            };
            let ep_idx = piece_index(enemy, Piece::Pawn);
            board.pieces[ep_idx] &= !bb(ep_target);
        } else {
            // Normal capture: remove enemy piece on 'to'
            for p in Piece::ALL {
                let eidx = piece_index(enemy, p);
                if board.pieces[eidx] & bb(to) != 0 {
                    board.pieces[eidx] &= !bb(to);
                    break;
                }
            }
        }
    }

    // ------------------------------
    // 3. Promotions
    // ------------------------------
    let final_piece = if promo != 0 {
        match promo {
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            _ => piece,
        }
    } else {
        piece
    };

    let final_idx = piece_index(color, final_piece);

    // Place piece on destination
    board.pieces[final_idx] |= bb(to);

    // ------------------------------
    // 4. Castling
    // ------------------------------
    if is_castle {
        match (side, to) {
            (Color::White, 6) => { // White king-side
                board.pieces[piece_index(Color::White, Piece::Rook)] &= !bb(7);
                board.pieces[piece_index(Color::White, Piece::Rook)] |= bb(5);
            }
            (Color::White, 2) => { // White queen-side
                board.pieces[piece_index(Color::White, Piece::Rook)] &= !bb(0);
                board.pieces[piece_index(Color::White, Piece::Rook)] |= bb(3);
            }
            (Color::Black, 62) => { // Black king-side
                board.pieces[piece_index(Color::Black, Piece::Rook)] &= !bb(63);
                board.pieces[piece_index(Color::Black, Piece::Rook)] |= bb(61);
            }
            (Color::Black, 58) => { // Black queen-side
                board.pieces[piece_index(Color::Black, Piece::Rook)] &= !bb(56);
                board.pieces[piece_index(Color::Black, Piece::Rook)] |= bb(59);
            }
            _ => {}
        }
    }

    // ------------------------------
    // 5. Update castling rights
    // ------------------------------
    update_castling_rights(board, from, to);

    // ------------------------------
    // 6. Update en passant square
    // ------------------------------
    board.en_passant = None;

    if piece == Piece::Pawn {
        if (from as i16 - to as i16).abs() == 16 {
            // Pawn double push
            let ep_sq = match side {
                Color::White => from + 8,
                Color::Black => from - 8,
            };
            board.en_passant = Some(ep_sq);
        }
    }

    // ------------------------------
    // 7. Update halfmove clock
    // ------------------------------
    if piece == Piece::Pawn || is_capture {
        board.halfmove_clock = 0;
    } else {
        board.halfmove_clock += 1;
    }

    // ------------------------------
    // 8. Update fullmove number
    // ------------------------------
    if side == Color::Black {
        board.fullmove_number += 1;
    }

    // ------------------------------
    // 9. Switch side
    // ------------------------------
    board.side_to_move = enemy;
}

/// Update castling rights based on moved or captured pieces
fn update_castling_rights(board: &mut Board, from: u8, to: u8) {
    // White king moved
    if from == 4 {
        board.castling &= !0b0011;
    }

    // Black king moved
    if from == 60 {
        board.castling &= !0b1100;
    }

    // White rooks moved or captured
    if from == 0 || to == 0 {
        board.castling &= !0b0010; // Q
    }
    if from == 7 || to == 7 {
        board.castling &= !0b0001; // K
    }

    // Black rooks moved or captured
    if from == 56 || to == 56 {
        board.castling &= !0b1000; // q
    }
    if from == 63 || to == 63 {
        board.castling &= !0b0100; // k
    }
}
