use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::{Bitboard, bb};
use crate::moves::MoveList;

#[derive(Clone)]
pub struct Board {
    /// 12 bitboards: white pawn..king, black pawn..king
    pub pieces: [Bitboard; 12],

    /// Occupancy for each color
    pub occupied: [Bitboard; 2],

    pub side_to_move: Color,
    pub castling: u8,              // bitmask: 1=K,2=Q,4=k,8=q
    pub en_passant: Option<u8>,    // square index 0–63
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pieces: [0; 12],
            occupied: [0; 2],
            side_to_move: Color::White,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Standard chess starting position
    pub fn startpos() -> Self {
        let mut b = Board::empty();

        // White
        b.pieces[piece_index(Color::White, Piece::Pawn)]   = 0x0000_0000_0000_ff00;
        b.pieces[piece_index(Color::White, Piece::Rook)]   = 0x0000_0000_0000_0081;
        b.pieces[piece_index(Color::White, Piece::Knight)] = 0x0000_0000_0000_0042;
        b.pieces[piece_index(Color::White, Piece::Bishop)] = 0x0000_0000_0000_0024;
        b.pieces[piece_index(Color::White, Piece::Queen)]  = 0x0000_0000_0000_0008;
        b.pieces[piece_index(Color::White, Piece::King)]   = 0x0000_0000_0000_0010;

        // Black
        b.pieces[piece_index(Color::Black, Piece::Pawn)]   = 0x00ff_0000_0000_0000;
        b.pieces[piece_index(Color::Black, Piece::Rook)]   = 0x8100_0000_0000_0000;
        b.pieces[piece_index(Color::Black, Piece::Knight)] = 0x4200_0000_0000_0000;
        b.pieces[piece_index(Color::Black, Piece::Bishop)] = 0x2400_0000_0000_0000;
        b.pieces[piece_index(Color::Black, Piece::Queen)]  = 0x0800_0000_0000_0000;
        b.pieces[piece_index(Color::Black, Piece::King)]   = 0x1000_0000_0000_0000;

        b.castling = 0b1111; // KQkq
        b.update_occupied();
        b
    }

    /// Convert (color, piece) into index 0–11
    pub fn index(&self, color: Color, piece: Piece) -> usize {
        (color as usize) * 6 + (piece as usize)
    }

    /// Combined occupancy of all pieces
    pub fn occupancy(&self) -> Bitboard {
        self.pieces.iter().copied().fold(0, |a, b| a | b)
    }

    /// Occupancy of one color
    pub fn occupancy_color(&self, color: Color) -> Bitboard {
        self.occupied[color as usize]
    }

    /// Recompute occupied[] arrays
    pub fn update_occupied(&mut self) {
        self.occupied[Color::White as usize] = 0;
        self.occupied[Color::Black as usize] = 0;

        for piece in Piece::ALL {
            let w = self.index(Color::White, piece);
            let b = self.index(Color::Black, piece);

            self.occupied[Color::White as usize] |= self.pieces[w];
            self.occupied[Color::Black as usize] |= self.pieces[b];
        }
    }

    /// Get piece on a square (if any)
    pub fn piece_at(&self, square: u8) -> Option<(Color, Piece)> {
        for color in [Color::White, Color::Black] {
            for piece in Piece::ALL {
                let idx = piece_index(color, piece);
                if (self.pieces[idx] & bb(square)) != 0 {
                    return Some((color, piece));
                }
            }
        }
        None
    }

    /// Return the king square for a given color
    pub fn king_square(&self, color: Color) -> usize {
        let bb = self.pieces[self.index(color, Piece::King)];
        bb.trailing_zeros() as usize
    }

    /// Material count for a color
    pub fn material(&self, color: Color) -> i32 {
        let mut total = 0;

        for piece in Piece::ALL {
            let idx = self.index(color, piece);
            let count = self.pieces[idx].count_ones() as i32;

            let val = match piece {
                Piece::Pawn   => 100,
                Piece::Knight => 320,
                Piece::Bishop => 330,
                Piece::Rook   => 500,
                Piece::Queen  => 900,
                Piece::King   => 0,
            };

            total += val * count;
        }

        total
    }

    /// Count all pieces for a color
    pub fn piece_count(&self, color: Color) -> usize {
        let mut total = 0;
        for piece in Piece::ALL {
            let idx = self.index(color, piece);
            total += self.pieces[idx].count_ones() as usize;
        }
        total
    }

    /// Attacks from a square (delegates to your movegen tables)
    pub fn attacks_from(&self, sq: usize, piece: Piece, color: Color) -> Bitboard {
        match piece {
            Piece::Pawn   => crate::pawn_moves::pawn_attacks(color, sq as u8),
            Piece::Knight => crate::knight_moves::KNIGHT_ATTACKS[sq],
            Piece::Bishop => crate::bishop_moves::bishop_attacks(sq, self.occupancy()),
            Piece::Rook   => crate::rook_moves::rook_attacks(sq, self.occupancy()),
            Piece::Queen  => {
                crate::bishop_moves::bishop_attacks(sq, self.occupancy()) |
                crate::rook_moves::rook_attacks(sq, self.occupancy())
            }
            Piece::King   => crate::king_moves::KING_ATTACKS[sq],
        }
    }

    /// All attacks for a color
    pub fn attacks_for(&self, color: Color) -> Bitboard {
        let mut attacks = 0;

        for piece in Piece::ALL {
            let bb = self.pieces[self.index(color, piece)];
            let mut b = bb;

            while b != 0 {
                let sq = b.trailing_zeros() as usize;
                b &= b - 1;

                attacks |= self.attacks_from(sq, piece, color);
            }
        }

        attacks
    }

    /// Is the given color in check?
    pub fn in_check(&self, color: Color) -> bool {
        let king_sq = self.king_square(color);
        let enemy = color.opposite();
        let attacks = self.attacks_for(enemy);
        (attacks & bb(king_sq as u8)) != 0
    }

    /// Wrapper for your move generator
    pub fn generate_legal_moves(&self) -> MoveList {
        crate::movegen::generate_moves(self)
    }
}
