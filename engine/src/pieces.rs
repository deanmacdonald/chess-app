#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn from_u8(v: u8) -> Color {
        match v {
            0 => Color::White,
            1 => Color::Black,
            _ => Color::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];
}

/// Index into the 12 bitboards:
/// 0–5 = white pieces, 6–11 = black pieces
#[inline]
pub fn piece_index(color: Color, piece: Piece) -> usize {
    let base = match color {
        Color::White => 0,
        Color::Black => 6,
    };

    base + match piece {
        Piece::Pawn => 0,
        Piece::Knight => 1,
        Piece::Bishop => 2,
        Piece::Rook => 3,
        Piece::Queen => 4,
        Piece::King => 5,
    }
}
