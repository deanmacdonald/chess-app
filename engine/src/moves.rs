/// Move encoding (32 bits):
///  0–5   : from-square (0–63)
///  6–11  : to-square (0–63)
/// 12–14  : promotion piece (0=none, 1=N, 2=B, 3=R, 4=Q)
/// 15     : capture flag
/// 16     : en passant flag
/// 17     : castling flag
///
/// This format is compact, fast, and easy to decode.

pub type Move = u32;

#[inline]
pub fn encode_move(
    from: u8,
    to: u8,
    promo: u8,
    capture: bool,
    en_passant: bool,
    castling: bool,
) -> Move {
    (from as Move)
        | ((to as Move) << 6)
        | ((promo as Move) << 12)
        | ((capture as Move) << 15)
        | ((en_passant as Move) << 16)
        | ((castling as Move) << 17)
}

#[inline]
pub fn move_from(m: Move) -> u8 {
    (m & 0x3F) as u8
}

#[inline]
pub fn move_to(m: Move) -> u8 {
    ((m >> 6) & 0x3F) as u8
}

#[inline]
pub fn move_promo(m: Move) -> u8 {
    ((m >> 12) & 0x7) as u8
}

#[inline]
pub fn move_is_capture(m: Move) -> bool {
    ((m >> 15) & 1) != 0
}

#[inline]
pub fn move_is_ep(m: Move) -> bool {
    ((m >> 16) & 1) != 0
}

#[inline]
pub fn move_is_castling(m: Move) -> bool {
    ((m >> 17) & 1) != 0
}

/// A simple move list container
pub struct MoveList {
    pub moves: Vec<Move>,
}

impl MoveList {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, m: Move) {
        self.moves.push(m);
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}
