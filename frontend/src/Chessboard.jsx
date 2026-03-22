import { useState } from "react";
import "./styles.css";

const PIECES = {
  r: "♜", n: "♞", b: "♝", q: "♛", k: "♚", p: "♟",
  R: "♖", N: "♘", B: "♗", Q: "♕", K: "♔", P: "♙",
};

const START_POS = [
  "rnbqkbnr",
  "pppppppp",
  "........",
  "........",
  "........",
  "........",
  "PPPPPPPP",
  "RNBQKBNR"
];

export default function Chessboard() {
  const [board, setBoard] = useState(START_POS.map(row => row.split("")));
  const [selected, setSelected] = useState(null);

  function handleSquareClick(file, rank) {
    const id = `${file}${rank}`;

    if (!selected) {
      // Select a piece
      if (board[7 - rank][file] !== ".") {
        setSelected({ file, rank });
      }
      return;
    }

    // Move piece
    const newBoard = board.map(row => [...row]);
    const piece = newBoard[7 - selected.rank][selected.file];

    newBoard[7 - selected.rank][selected.file] = ".";
    newBoard[7 - rank][file] = piece;

    setBoard(newBoard);
    setSelected(null);
  }

  const squares = [];
  for (let rank = 7; rank >= 0; rank--) {
    for (let file = 0; file < 8; file++) {
      const isDark = (rank + file) % 2 === 1;
      const pieceChar = board[7 - rank][file];
      const piece = pieceChar !== "." ? PIECES[pieceChar] : null;

      squares.push(
        <div
          key={`${file}${rank}`}
          className={isDark ? "square dark" : "square light"}
          onClick={() => handleSquareClick(file, rank)}
        >
          {piece && <div className="piece">{piece}</div>}
          {selected &&
            selected.file === file &&
            selected.rank === rank &&
            <div className="highlight" />}
        </div>
      );
    }
  }

  return <div className="board">{squares}</div>;
}
