import { useState, useEffect } from "react";
import Chessboard from "./Chessboard.jsx";

const START_FEN =
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

export default function App() {
  const [position, setPosition] = useState(START_FEN);
  const [selected, setSelected] = useState(null);

  // Captured pieces
  const [capturedWhite, setCapturedWhite] = useState([]);
  const [capturedBlack, setCapturedBlack] = useState([]);

  // Clocks (start with White running)
  const [whiteTime, setWhiteTime] = useState(300);
  const [blackTime, setBlackTime] = useState(300);
const gameOver = whiteTime === 0 || blackTime === 0;
  const [whiteRunning, setWhiteRunning] = useState(true);
  const [blackRunning, setBlackRunning] = useState(false);


  // Timer effect
  useEffect(() => {
    if (gameOver) return;

    const interval = setInterval(() => {
      if (whiteRunning) {
        setWhiteTime(t => Math.max(t - 1, 0));
      }
      if (blackRunning) {
        setBlackTime(t => Math.max(t - 1, 0));
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [whiteRunning, blackRunning, gameOver]);

  // Stop game when time hits zero
  useEffect(() => {
    if (whiteTime === 0 || blackTime === 0) {
      setWhiteRunning(false);
      setBlackRunning(false);
    }
  }, [whiteTime, blackTime]);

  // Handle board clicks
  function onSquareClick(r, c) {
    if (gameOver) return;

    if (!selected) {
      setSelected({ r, c });
      return;
    }

    handleMove(selected, { r, c });
    setSelected(null);
  }

  // Simple move logic (no legality checks)
  function handleMove(from, to) {
    const fenBoard = position.split(" ")[0].split("/");
    const board = fenBoard.map((row) =>
      row.replace(/[1-8]/g, (n) => ".".repeat(parseInt(n))).split("")
    );

    const piece = board[from.r][from.c];
    const target = board[to.r][to.c];

    if (piece === ".") return;

    // Capture
    if (target !== ".") {
      if (target === target.toUpperCase()) {
        setCapturedWhite(prev => [...prev, target]);
      } else {
        setCapturedBlack(prev => [...prev, target]);
      }
    }

    // Move piece
    board[to.r][to.c] = piece;
    board[from.r][from.c] = ".";

    // Convert back to FEN (keep side-to-move as-is for now)
    const newFEN = board
      .map((row) =>
        row.join("").replace(/\.{1,8}/g, (m) => m.length.toString())
      )
      .join("/");

    setPosition(`${newFEN} w - - 0 1`);
  }

  return (
    <div>
      <h1>Dean’s Chess App</h1>

      {/* Black's clock control: when Black presses, Black stops, White starts */}
      <div style={{ display: "flex", justifyContent: "center", marginBottom: "10px" }}>
        <button
          onClick={() => {
            if (gameOver) return;
            setBlackRunning(false);
            setWhiteRunning(true);
          }}
        >
          Black Move (End Turn)
        </button>
      </div>

      <Chessboard
        position={position}
        selected={selected}
        onSquareClick={onSquareClick}
        capturedWhite={capturedWhite}
        capturedBlack={capturedBlack}
        whiteTime={whiteTime}
        blackTime={blackTime}
      />

      {/* White's clock control: when White presses, White stops, Black starts */}
      <div style={{ display: "flex", justifyContent: "center", marginTop: "10px" }}>
        <button
          onClick={() => {
            if (gameOver) return;
            setWhiteRunning(false);
            setBlackRunning(true);
          }}
        >
          White Move (End Turn)
        </button>
      </div>

      {gameOver && (
        <h2 style={{ marginTop: "20px" }}>
          Game Over — {whiteTime === 0 ? "Black wins" : "White wins"}
        </h2>
      )}
    </div>
  );
}
