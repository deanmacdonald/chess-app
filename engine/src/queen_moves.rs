use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::is_set;
use crate::moves::{MoveList, encode_move};

/// Queen directions = rook dirs + bishop dirs
const QUEEN_DIRS: [i8; 8] = [8, -8, 1, -1, 9, 7, -7, -9];

pub fn generate_queen_moves(board: &Board, color: Color, list: &mut MoveList) {
    let queens = board.pieces[piece_index(color, Piece::Queen)];
    let friendly = board.occupancy_color(color);
    let enemy = board.occupancy_color(color.opposite());

    let mut bb_queens = queens;

    while bb_queens != 0 {
        let sq = bb_queens.trailing_zeros() as u8;

        for &dir in QUEEN_DIRS.iter() {
            let mut target = sq as i16 + dir as i16;

            while target >= 0 && target < 64 {
                let to = target as u8;

                // Prevent wrap-around across files
                if !queen_file_ok(sq, to, dir) {
                    break;
                }

                if is_set(friendly, to) {
                    break; // blocked by own piece
                }

                let capture = is_set(enemy, to);
                list.push(encode_move(sq, to, 0, capture, false, false));

                if capture {
                    break; // stop after capturing
                }

                target += dir as i16;
            }
        }

        bb_queens &= bb_queens - 1;
    }
}

/// Prevents wrap-around for sliding moves
fn queen_file_ok(from: u8, to: u8, dir: i8) -> bool {
    let f1 = from % 8;
    let f2 = to % 8;

    match dir {
        1 => f2 > f1,   // right
        -1 => f2 < f1,  // left
        9 | -7 => f2 > f1,  // diagonal right
        7 | -9 => f2 < f1,  // diagonal left
        8 | -8 => true,     // vertical always safe
        _ => false,
    }
}
