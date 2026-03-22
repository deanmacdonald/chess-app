pub type Bitboard = u64;

#[inline]
pub fn bb(square: u8) -> Bitboard {
    1u64 << square
}

#[inline]
pub fn pop_lsb(bb: &mut Bitboard) -> u8 {
    let sq = bb.trailing_zeros() as u8;
    *bb &= *bb - 1;
    sq
}

#[inline]
pub fn is_set(bb: Bitboard, square: u8) -> bool {
    (bb & (1u64 << square)) != 0
}

#[inline]
pub fn popcount(bb: Bitboard) -> i32 {
    bb.count_ones() as i32
}

// Minimal bb namespace so eval_* modules compile.
// Masks are zeroed for now; you can replace with real tables later.
pub mod bb {
    use super::Bitboard;

    pub const FILE: [Bitboard; 8] = [0; 8];
    pub const RANK: [Bitboard; 8] = [0; 8];

    pub const PASSED: [[Bitboard; 64]; 2] = [[0; 64]; 2];
    pub const FRONT_SPANS: [[Bitboard; 64]; 2] = [[0; 64]; 2];
    pub const PAWN_ATTACKS: [[Bitboard; 64]; 2] = [[0; 64]; 2];
    pub const KING_SHIELD: [[Bitboard; 64]; 2] = [[0; 64]; 2];

    pub const BELOW: [Bitboard; 64] = [0; 64];
    pub const ABOVE: [Bitboard; 64] = [0; 64];

    pub const CENTER: Bitboard = 0;
}
