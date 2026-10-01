use engine::Engine;

#[test]
fn best_move_on_start_position() {
    let mut eng = Engine::new();
    let fen = "rn1qkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 1 3";
    let mv = eng.best_move(fen);
    assert!(!mv.is_empty(), "best move should not be empty");
}

#[test]
fn evaluate_position() {
    let mut eng = Engine::new();
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let eval = eng.evaluate(fen);
    assert!(!eval.is_empty(), "evaluation should not be empty");
}

#[test]
fn fen_to_pgn_basic() {
    let mut eng = Engine::new();
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pgn = eng.fen_to_pgn(fen);
    assert!(!pgn.is_empty(), "PGN should not be empty");
}
