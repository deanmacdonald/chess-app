import "./styles.css";

const PIECES = {
  r: "♜", n: "♞", b: "♝", q: "♛", k: "♚", p: "♟",
  R: "♖", N: "♘", B: "♗", Q: "♕", K: "♔", P: "♙",
};

export default function Chessboard({
  position,
  selected,
  onSquareClick,
  capturedWhite = [],
  capturedBlack = [],
  whiteTime,
  blackTime
}) {
  // Convert FEN → 2D array
  const rows = position.split(" ")[0].split("/");
  const board = rows.map((row) =>
    row.replace(/[1-8]/g, (n) => ".".repeat(parseInt(n))).split("")
  );

  // Flip board so White is always at the bottom
  const displayBoard = [...board].reverse();

  return (
    <div className="chess-container">

      {/* Black Clock */}
      <div className="clock black-clock">{formatTime(blackTime)}</div>

      {/* Black Captured */}
      <div className="captured black-captured">
        {capturedBlack.map((p, i) => (
          <span key={i} className="captured-piece">{PIECES[p]}</span>
        ))}
      </div>

      {/* Board */}
      <div className="board">
        {displayBoard.map((rank, r) =>
          rank.map((sq, c) => {
            const piece = PIECES[sq] || null;
            const isLight = ((7 - r) + c) % 2 === 0;
            const isSelected =
              selected?.r === 7 - r && selected?.c === c;

            return (
              <div
                key={`${r}-${c}`}
                className={`square ${isLight ? "light" : "dark"}`}
                onClick={() => onSquareClick(7 - r, c)}
              >
                {isSelected && <div className="highlight" />}
                {piece && <span className="piece">{piece}</span>}
              </div>
            );
          })
        )}
      </div>

      {/* White Captured */}
      <div className="captured white-captured">
        {capturedWhite.map((p, i) => (
          <span key={i} className="captured-piece">{PIECES[p]}</span>
        ))}
      </div>

      {/* White Clock */}
      <div className="clock white-clock">{formatTime(whiteTime)}</div>

    </div>
  );
}

function formatTime(seconds) {
  if (seconds == null) return "--:--";
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}
