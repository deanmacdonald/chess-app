use rand::random;

pub struct Zobrist {
    pub piece: [[u64; 64]; 12],
    pub castling: [u64; 16],
    pub en_passant: [u64; 64],
    pub side: u64,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut piece = [[0u64; 64]; 12];
        let mut castling = [0u64; 16];
        let mut en_passant = [0u64; 64];

        for p in 0..12 {
            for sq in 0..64 {
                piece[p][sq] = random();
            }
        }

        for i in 0..16 {
            castling[i] = random();
        }

        for sq in 0..64 {
            en_passant[sq] = random();
        }

        let side = random();

        Zobrist {
            piece,
            castling,
            en_passant,
            side,
        }
    }

    pub fn hash<B>(&self, _board: &B) -> u64 {
        // Temporary stub so iterative_deepening.rs and search.rs compile.
        // We can wire this to your real Board later.
        0
    }
}
