import "./styles.css";

const PIECES = {
  r: "♜", n: "♞", b: "♝", q: "♛", k: "♚", p: "♟",
  R: "♖", N: "♘", B: "♗", Q: "♕", K: "♔", P: "♙",
};

export default function Chessboard({ position, selected, onSquareClick }) {
  // Convert FEN → 2D array
  const rows = position.split(" ")[0].split("/");
  const board = rows.map((row) =>
    row
      .replace(/[1-8]/g, (n) => ".".repeat(parseInt(n)))
      .split("")
  );

  const squares = [];
  for (let rank = 7; rank >= 0; rank--) {
    for (let file = 0; file < 8; file++) {
      const isDark = (rank + file) % 2 === 1;
      const pieceChar = board[7 - rank][file];
      const piece = pieceChar !== "." ? PIECES[pieceChar] : null;

      const isSelected =
        selected &&
        selected.file === file &&
        selected.rank === rank;

      squares.push(
        <div
          key={`${file}${rank}`}
          className={isDark ? "square dark" : "square light"}
          onClick={() => onSquareClick(file, rank)}
        >
          {piece && <div className="piece">{piece}</div>}
          {isSelected && <div className="highlight" />}
        </div>
      );
    }
  }

  return <div className="board">{squares}</div>;
}
