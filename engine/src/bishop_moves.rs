use crate::bitboard::Bitboard;

/// --- NEW: Bishop attack generator (simple sliding) ---------------
pub fn bishop_attacks(sq: usize, occ: Bitboard) -> Bitboard {
    let dirs: [i8; 4] = [9, 7, -7, -9];
    let mut bb = 0;
    for d in dirs {
        let mut t = sq as i16 + d as i16;
        while t >= 0 && t < 64 {
            let to = t as u8;
            let f1 = sq % 8;
            let f2 = to % 8;
            if (d == 9 || d == -7) && usize::from(f2) <= f1 { break; }
            if (d == 7 || d == -9) && usize::from(f2) >= f1 { break; }
            bb |= 1u64 << to;
            if (occ & (1u64 << to)) != 0 { break; }
            t += d as i16;
        }
    }
    bb
}
// ----------------------------------------------------------------

