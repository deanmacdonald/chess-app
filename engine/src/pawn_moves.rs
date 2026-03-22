use crate::board::Board;
use crate::moves::MoveList;
use crate::pieces::Color;

pub fn generate_pawn_moves(_board: &Board, _color: Color, _list: &mut MoveList) {
    // TODO: real pawn movegen
}

pub fn pawn_attacks(color: Color, sq: u8) -> u64 {
    // Minimal correct attack masks
    let mut attacks = 0u64;
    let file = sq % 8;
    let _rank = sq / 8;

    match color {
        Color::White => {
            if file > 0     { attacks |= 1u64 << (sq + 7); }
            if file < 7     { attacks |= 1u64 << (sq + 9); }
        }
        Color::Black => {
            if file > 0     { attacks |= 1u64 << (sq - 9); }
            if file < 7     { attacks |= 1u64 << (sq - 7); }
        }
    }

    attacks
}
