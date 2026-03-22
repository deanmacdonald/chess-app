use std::io::{self, Write};

use engine::board::Board;
use engine::fen::from_fen;
use engine::search_best_move::search_best_move;
use engine::make_move::make_move;

fn main() {
    println!("Black Knight Chess Engine — CLI Mode");
    println!("Commands:");
    println!("  position startpos");
    println!("  position fen <FEN>");
    println!("  go depth <N>");
    println!("  print");
    println!("  quit");

    let mut board = Board::startpos();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "quit" => break,

            "print" => {
                println!("{:?}", board);
            }

            "position" => {
                if parts.len() >= 2 && parts[1] == "startpos" {
                    board = Board::startpos();
                    println!("Loaded startpos");
                } else if parts.len() >= 3 && parts[1] == "fen" {
                    let fen = parts[2..].join(" ");
                    match from_fen(&fen) {
                        Ok(b) => {
                            board = b;
                            println!("Loaded FEN");
                        }
                        Err(e) => println!("FEN error: {}", e),
                    }
                }
            }

            "go" => {
                if parts.len() == 3 && parts[1] == "depth" {
                    let depth: i32 = parts[2].parse().unwrap_or(4);

                    if let Some((best, score, info)) = search_best_move(&mut board, depth) {
                        println!("score {}", score);
                        println!("nodes {}", info.nodes);

                        // apply the move to the board
                        make_move(&mut board, best);
                    } else {
                        println!("No legal moves");
                    }
                }
            }

            _ => println!("Unknown command"),
        }
    }
}
