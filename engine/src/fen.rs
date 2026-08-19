use crate::board::Board;
use crate::pieces::{Color, Piece, piece_index};
use crate::bitboard::bb;

/// Load a board from a FEN string
pub fn from_fen(fen: &str) -> Result<Board, String> {
    let mut board = Board::empty();

    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 4 {
        return Err("Invalid FEN: not enough fields".into());
    }

    // ------------------------------
    // 1. Piece placement
    // ------------------------------
    let mut sq: i32 = 56; // start at a8

    for c in parts[0].chars() {
        match c {
            '/' => sq -= 16, // next rank
            '1'..='8' => sq += c as i32 - '0' as i32 ,
            _ => {
                let (color, piece) = match c {
                    'p' => (Color::Black, Piece::Pawn),
                    'n' => (Color::Black, Piece::Knight),
                    'b' => (Color::Black, Piece::Bishop),
                    'r' => (Color::Black, Piece::Rook),
                    'q' => (Color::Black, Piece::Queen),
                    'k' => (Color::Black, Piece::King),

                    'P' => (Color::White, Piece::Pawn),
                    'N' => (Color::White, Piece::Knight),
                    'B' => (Color::White, Piece::Bishop),
                    'R' => (Color::White, Piece::Rook),
                    'Q' => (Color::White, Piece::Queen),
                    'K' => (Color::White, Piece::King),

                    _ => return Err(format!("Invalid FEN piece: {}", c)),
                };

                let idx = piece_index(color, piece);
                board.pieces[idx] |= bb(sq as u8);

                sq += 1;
            }
        }
    }

    // ------------------------------
    // 2. Side to move
    // ------------------------------
    board.side_to_move = match parts[1] {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err("Invalid FEN: side to move".into()),
    };

    // ------------------------------
    // 3. Castling rights
    // ------------------------------
    board.castling = 0;

    if parts[2].contains('K') { board.castling |= 0b0001; }
    if parts[2].contains('Q') { board.castling |= 0b0010; }
    if parts[2].contains('k') { board.castling |= 0b0100; }
    if parts[2].contains('q') { board.castling |= 0b1000; }

    // ------------------------------
    // 4. En passant
    // ------------------------------
    board.en_passant = if parts[3] == "-" {
        None
    } else {
        Some(square_from_algebraic(parts[3])?)
    };

    // ------------------------------
    // 5. Halfmove clock
    // ------------------------------
    board.halfmove_clock = if parts.len() > 4 {
        parts[4].parse().unwrap_or(0)
    } else {
        0
    };

    // ------------------------------
    // 6. Fullmove number
    // ------------------------------
    board.fullmove_number = if parts.len() > 5 {
        parts[5].parse().unwrap_or(1)
    } else {
        1
    };

    Ok(board)
}

/// Convert algebraic square like "e4" → 28
fn square_from_algebraic(s: &str) -> Result<u8, String> {
    if s.len() != 2 {
        return Err("Invalid en passant square".into());
    }

    let file = s.chars().nth(0).unwrap();
    let rank = s.chars().nth(1).unwrap();

    if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
        return Err("Invalid en passant square".into());
    }

    let f = (file as u8 - b'a') as u8;
    let r = (rank as u8 - b'1') as u8;

    Ok(r * 8 + f)
}

/// Convert board back to FEN (useful for debugging)
pub fn to_fen(board: &Board) -> String {
    let mut s = String::new();

    for rank in (0..8).rev() {
        let mut empty = 0;

        for file in 0..8 {
            let sq = rank * 8 + file;

            if let Some((color, piece)) = board.piece_at(sq as u8) {
                if empty > 0 {
                    s.push_str(&empty.to_string());
                    empty = 0;
                }

                let c = match (color, piece) {
                    (Color::White, Piece::Pawn) => 'P',
                    (Color::White, Piece::Knight) => 'N',
                    (Color::White, Piece::Bishop) => 'B',
                    (Color::White, Piece::Rook) => 'R',
                    (Color::White, Piece::Queen) => 'Q',
                    (Color::White, Piece::King) => 'K',

                    (Color::Black, Piece::Pawn) => 'p',
                    (Color::Black, Piece::Knight) => 'n',
                    (Color::Black, Piece::Bishop) => 'b',
                    (Color::Black, Piece::Rook) => 'r',
                    (Color::Black, Piece::Queen) => 'q',
                    (Color::Black, Piece::King) => 'k',
                };

                s.push(c);
            } else {
                empty += 1;
            }
        }

        if empty > 0 {
            s.push_str(&empty.to_string());
        }

        if rank > 0 {
            s.push('/');
        }
    }

    // Side to move
    s.push(' ');
    s.push(match board.side_to_move {
        Color::White => 'w',
        Color::Black => 'b',
    });

    // Castling
    s.push(' ');
    if board.castling == 0 {
        s.push('-');
    } else {
        if board.castling & 0b0001 != 0 { s.push('K'); }
        if board.castling & 0b0010 != 0 { s.push('Q'); }
        if board.castling & 0b0100 != 0 { s.push('k'); }
        if board.castling & 0b1000 != 0 { s.push('q'); }
    }

    // En passant
    s.push(' ');
    if let Some(ep) = board.en_passant {
        s.push_str(&square_to_algebraic(ep));
    } else {
        s.push('-');
    }

    // Halfmove + fullmove
    s.push(' ');
    s.push_str(&board.halfmove_clock.to_string());
    s.push(' ');
    s.push_str(&board.fullmove_number.to_string());

    s
}

/// Convert square index to algebraic (e.g., 0 → a1)
fn square_to_algebraic(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;

    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;

    format!("{}{}", file_char, rank_char)
}
