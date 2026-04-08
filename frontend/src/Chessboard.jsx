import { useRef } from "react";
import "./styles.css";

const PIECES = {
  r: "♜",
  n: "♞",
  b: "♝",
  q: "♛",
  k: "♚",
  p: "♟",
  R: "♖",
  N: "♘",
  B: "♗",
  Q: "♕",
  K: "♔",
  P: "♙",
};

export default function Chessboard({
  position,
  selected,
  lastMove,
  legalMoves = [],
  onSquareClick,
  capturedWhite = [],
  capturedBlack = [],
  whiteTime,
  blackTime,
}) {
  const dragFrom = useRef(null);

  // Convert FEN → 2D array
  const rows = position.split(" ")[0].split("/");
  const board = rows.map((row) =>
    row.replace(/[1-8]/g, (n) => ".".repeat(parseInt(n))).split("")
  );

  const displayBoard = [...board].reverse();

  function handleDragStart(e, r, c) {
    dragFrom.current = { r, c };
    e.dataTransfer.effectAllowed = "move";
  }

  function handleDrop(e, r, c) {
    e.preventDefault();
    if (!dragFrom.current) return;
    onSquareClick(r, c, dragFrom.current);
    dragFrom.current = null;
  }

  function isLegalTarget(r, c) {
    return legalMoves.some((m) => m.r === r && m.c === c);
  }

  function isLastMove(r, c) {
    return (
      lastMove &&
      ((lastMove.from.r === r && lastMove.from.c === c) ||
        (lastMove.to.r === r && lastMove.to.c === c))
    );
  }

  return (
    <div className="chess-container">
      {/* Black Clock */}
      <div className="clock black-clock">{formatTime(blackTime)}</div>

      {/* Black Captured */}
      <div className="captured black-captured">
        {capturedBlack.map((p, i) => (
          <span key={i} className="captured-piece">
            {PIECES[p]}
          </span>
        ))}
      </div>

      {/* Board */}
      <div className="board">
        {displayBoard.map((rank, r) =>
          rank.map((sq, c) => {
            const piece = PIECES[sq] || null;

            const realR = 7 - r;
            const isLight = (realR + c) % 2 === 0;
            const isSelected =
              selected?.r === realR && selected?.c === c;
            const legal = isLegalTarget(realR, c);
            const last = isLastMove(realR, c);

            return (
              <div
                key={`${r}-${c}`}
                className={`square ${isLight ? "light" : "dark"}`}
                onClick={() => onSquareClick(realR, c)}
                onDragOver={(e) => e.preventDefault()}
                onDrop={(e) => handleDrop(e, realR, c)}
              >
                {last && <div className="last-move" />}
                {isSelected && <div className="highlight" />}
                {legal && <div className="legal-dot" />}

                {piece && (
                  <span
                    className="piece"
                    draggable
                    onDragStart={(e) =>
                      handleDragStart(e, realR, c)
                    }
                  >
                    {piece}
                  </span>
                )}
              </div>
            );
          })
        )}
      </div>

      {/* White Captured */}
      <div className="captured white-captured">
        {capturedWhite.map((p, i) => (
          <span key={i} className="captured-piece">
            {PIECES[p]}
          </span>
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
