import { useState } from "react";
import { Chess } from "chess.js";
import Chessboard from "./Chessboard";

async function getBestMove(fen) {
  const res = await fetch("http://localhost:8000/best-move", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ fen }),
  });

  const move = await res.json();
  return move; // e.g., "e2e4"
}

function toSquare(file, rank) {
  const fileChar = String.fromCharCode("a".charCodeAt(0) + file);
  const rankChar = (rank + 1).toString();
  return fileChar + rankChar;
}

export default function App() {
  const [game, setGame] = useState(new Chess());
  const [selected, setSelected] = useState(null);

  async function handleSquareClick(file, rank) {
    const square = toSquare(file, rank);
    const current = new Chess(game.fen());
    const piece = current.get(square);

    // First click: select a piece
    if (!selected) {
      if (piece && piece.color === current.turn()) {
        setSelected({ file, rank });
      }
      return;
    }

    // Second click: attempt move
    const fromSquare = toSquare(selected.file, selected.rank);
    const move = current.move({
      from: fromSquare,
      to: square,
      promotion: "q",
    });

    setSelected(null);

    if (!move) {
      return;
    }

    // Update board with player's move
    setGame(current);

    // Engine move
    const engineMove = await getBestMove(current.fen());
    if (!engineMove || engineMove.length < 4) return;

    const engine = new Chess(current.fen());
    engine.move({
      from: engineMove.slice(0, 2),
      to: engineMove.slice(2, 4),
      promotion: "q",
    });

    setGame(engine);
  }

  return (
    <div style={{ padding: "20px" }}>
      <h1>Black Knight Chess</h1>
      <Chessboard
        position={game.fen()}
        selected={selected}
        onSquareClick={handleSquareClick}
      />
    </div>
  );
}
