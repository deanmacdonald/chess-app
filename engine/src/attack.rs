use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::is_set;

/// Check if a square is attacked by a given color
pub fn is_square_attacked(board: &Board, sq: u8, by: Color) -> bool {
    // ------------------------------
    // Pawn attacks
    // ------------------------------
    let pawns = board.pieces[piece_index(by, Piece::Pawn)];

    if by == Color::White {
        // White pawns attack +7 and +9
        if sq >= 9 && is_set(pawns, sq - 7) && (sq % 8 != 0) {
            return true;
        }
        if sq >= 9 && is_set(pawns, sq - 9) && (sq % 8 != 7) {
            return true;
        }
    } else {
        // Black pawns attack -7 and -9
        if sq <= 54 && is_set(pawns, sq + 7) && (sq % 8 != 7) {
            return true;
        }
        if sq <= 54 && is_set(pawns, sq + 9) && (sq % 8 != 0) {
            return true;
        }
    }

    // ------------------------------
    // Knight attacks
    // ------------------------------
    let knights = board.pieces[piece_index(by, Piece::Knight)];
    const KNIGHT_OFFSETS: [i8; 8] = [17, 15, 10, 6, -17, -15, -10, -6];

    for &off in KNIGHT_OFFSETS.iter() {
        let t = sq as i16 + off as i16;
        if t >= 0 && t < 64 {
            let to = t as u8;
            if knight_file_ok(sq, to) && is_set(knights, to) {
                return true;
            }
        }
    }

    // ------------------------------
    // King attacks
    // ------------------------------
    let king = board.pieces[piece_index(by, Piece::King)];
    const KING_OFFSETS: [i8; 8] = [8, -8, 1, -1, 9, 7, -7, -9];

    for &off in KING_OFFSETS.iter() {
        let t = sq as i16 + off as i16;
        if t >= 0 && t < 64 {
            let to = t as u8;
            if knight_file_ok(sq, to) && is_set(king, to) {
                return true;
            }
        }
    }

    // ------------------------------
    // Sliding attacks: bishops / rooks / queens
    // ------------------------------
    let bishops = board.pieces[piece_index(by, Piece::Bishop)];
    let rooks   = board.pieces[piece_index(by, Piece::Rook)];
    let queens  = board.pieces[piece_index(by, Piece::Queen)];

    let occ = board.occupancy();

    // Bishop/queen diagonals
    const BISHOP_DIRS: [i8; 4] = [9, 7, -7, -9];
    for &dir in BISHOP_DIRS.iter() {
        let mut t = sq as i16 + dir as i16;
        while t >= 0 && t < 64 {
            let to = t as u8;
            if !bishop_file_ok(sq, to, dir) {
                break;
            }
            if is_set(bishops | queens, to) {
                return true;
            }
            if is_set(occ, to) {
                break;
            }
            t += dir as i16;
        }
    }

    // Rook/queen orthogonals
    const ROOK_DIRS: [i8; 4] = [8, -8, 1, -1];
    for &dir in ROOK_DIRS.iter() {
        let mut t = sq as i16 + dir as i16;
        while t >= 0 && t < 64 {
            let to = t as u8;
            if !rook_file_ok(sq, to, dir) {
                break;
            }
            if is_set(rooks | queens, to) {
                return true;
            }
            if is_set(occ, to) {
                break;
            }
            t += dir as i16;
        }
    }

    false
}

/// Prevent wrap-around for knight moves
fn knight_file_ok(from: u8, to: u8) -> bool {
    let f1 = from % 8;
    let f2 = to % 8;
    let df = if f1 > f2 { f1 - f2 } else { f2 - f1 };
    df <= 2
}

/// Prevent wrap-around for bishop moves
fn bishop_file_ok(from: u8, to: u8, dir: i8) -> bool {
    let f1 = from % 8;
    let f2 = to % 8;
    match dir {
        9 | -7 => f2 > f1,
        7 | -9 => f2 < f1,
        _ => false,
    }
}

/// Prevent wrap-around for rook moves
fn rook_file_ok(from: u8, to: u8, dir: i8) -> bool {
    let f1 = from % 8;
    let f2 = to % 8;
    match dir {
        1 => f2 > f1,
        -1 => f2 < f1,
        8 | -8 => true,
        _ => false,
    }
}
