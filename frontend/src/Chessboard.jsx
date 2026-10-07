import { useEffect, useState } from "react";
import { getFEN, getLegalMoves, makeMove, resetGame } from "./gameLogic.js";
import "./App.css";

const STARTING_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Convert FEN → 8x8 board array
function fenToBoard(fen) {
  if (typeof fen !== "string") {
    throw new Error("Expected a FEN string.");
  }

  const rows = fen.trim().split(" ")[0].split("/");

  if (rows.length !== 8) {
    throw new Error(`Invalid FEN: expected 8 rows, received ${rows.length}.`);
  }

  return rows.map((row) => {
    const expanded = [];

    for (const char of row) {
      if (/^[1-8]$/.test(char)) {
        expanded.push(...Array(Number(char)).fill(null));
      } else if (/^[prnbqkPRNBQK]$/.test(char)) {
        expanded.push(char);
      } else {
        throw new Error(`Invalid FEN character: ${char}`);
      }
    }

    if (expanded.length !== 8) {
      throw new Error(
        `Invalid FEN row: expected 8 squares, received ${expanded.length}.`,
      );
    }

    return expanded;
  });
}

export default function Chessboard() {
  // Always show a playable-looking board first.
  const [board, setBoard] = useState(() => fenToBoard(STARTING_FEN));
  const [selected, setSelected] = useState(null);
  const [legalSquares, setLegalSquares] = useState([]);
  const [turn, setTurn] = useState("White");
  const [status, setStatus] = useState("Ongoing");
  const [message, setMessage] = useState("");

  useEffect(() => {
    let active = true;

    async function load() {
      try {
        const fen = await getFEN();

        if (!active) {
          return;
        }

        setBoard(fenToBoard(fen));
        setMessage("");
      } catch (error) {
        console.error("Could not load game from Railway:", error);

        if (active) {
          setMessage("Backend unavailable — using local starting position.");
        }
      }
    }

    load();

    return () => {
      active = false;
    };
  }, []);

  async function handleSquareClick(square) {
    try {
      setMessage("");

      // First click: select a square and request legal moves.
      if (!selected) {
        const legal = await getLegalMoves();
        const movesFromSquare = Array.isArray(legal)
          ? legal.filter((move) => move.startsWith(square))
          : [];

        setSelected(square);
        setLegalSquares(movesFromSquare.map((move) => move.slice(2)));
        return;
      }

      // Second click: attempt the selected move.
      const result = await makeMove(selected, square);

      if (result?.result === "OK") {
        setBoard(fenToBoard(result.fen));
        setTurn(result.turn ?? "White");
        setStatus(result.game_status ?? "Ongoing");
      } else {
        setMessage(result?.message ?? "That move is not legal.");
      }
    } catch (error) {
      console.error("Move failed:", error);
      setMessage(
        error instanceof Error
          ? error.message
          : "Unable to contact the chess backend.",
      );
    } finally {
      if (selected) setSelected(null);
      if (selected) setLegalSquares([]);
    }
  }

  async function handleReset() {
    try {
      setMessage("");

      const result = await resetGame();

      setBoard(fenToBoard(result.fen));
      setTurn(result.turn ?? "White");
      setStatus(result.game_status ?? "Ongoing");
    } catch (error) {
      console.error("Reset failed:", error);

      setBoard(fenToBoard(STARTING_FEN));
      setTurn("White");
      setStatus("Ongoing");
      setMessage("Backend unavailable — local starting position restored.");
    } finally {
      setSelected(null);
      setLegalSquares([]);
    }
  }

  return (
    <div className="chess-container">
      <h1>Dean&apos;s Chess Game</h1>
      <h2>Turn: {turn}</h2>
      <h3>Status: {status}</h3>

      <button type="button" onClick={handleReset}>
        Reset Game
      </button>

      {message && <p className="game-message">{message}</p>}

      <div className="board" aria-label="Chessboard">
        {board.map((row, r) =>
          row.map((piece, c) => {
            const square = "abcdefgh"[c] + (8 - r);
            const isLegal = legalSquares.includes(square);
            const isSelected = selected === square;
            const squareColor = (r + c) % 2 === 0 ? "light" : "dark";

            return (
              <button
                type="button"
                key={square}
                className={`square ${squareColor} ${
                  isSelected ? "selected" : ""
                }`}
                onClick={() => handleSquareClick(square)}
                aria-label={`${square}${piece ? ` ${piece}` : ""}`}
              >
                {piece && (
                  <span className="piece" aria-hidden="true">{({K:"♔",Q:"♕",R:"♖",B:"♗",N:"♘",P:"♙",k:"♚",q:"♛",r:"♜",b:"♝",n:"♞",p:"♟"})[piece]}</span>
                )}

                {isLegal && <span className="highlight" />}
              </button>
            );
          }),
        )}
      </div>
    </div>
  );
}
