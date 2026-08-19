import { useState, useEffect } from "react";
import Chessboard from "./Chessboard.jsx";

export default function App() {
  /* -------- STATE -------- */

  const [position, setPosition] = useState(null);
  const [selected, setSelected] = useState(null);
  const [legalMoves, setLegalMoves] = useState([]);

  const [capturedWhite, setCapturedWhite] = useState([]);
  const [capturedBlack, setCapturedBlack] = useState([]);

  // Clocks
  const [whiteTime, setWhiteTime] = useState(300);
  const [blackTime, setBlackTime] = useState(300);

  // Whose turn is running the clock
  const [currentTurn, setCurrentTurn] = useState("white");

  const gameOver = whiteTime === 0 || blackTime === 0;

  // Derived running state
  const whiteRunning = !gameOver && currentTurn === "white" && whiteTime > 0;
  const blackRunning = !gameOver && currentTurn === "black" && blackTime > 0;

  /* -------- CLOCK EFFECT -------- */

  useEffect(() => {
    if (gameOver) return;

    const interval = setInterval(() => {
      if (whiteRunning) {
        setWhiteTime((t) => Math.max(t - 1, 0));
      }
      if (blackRunning) {
        setBlackTime((t) => Math.max(t - 1, 0));
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [whiteRunning, blackRunning, gameOver]);

  /* -------- INITIAL BOARD LOAD -------- */

  useEffect(() => {
    async function load() {
      try {
        const res = await fetch("http://localhost:8000/board");
        const data = await res.json();
        setPosition(data.fen);
      } catch (err) {
        console.error("Failed to load board:", err);
      }
    }
    load();
  }, []);

  /* -------- HANDLE MOVES -------- */

  async function onSquareClick(r, c) {
    if (gameOver) return;

    if (!selected) {
      setSelected({ r, c });
      setLegalMoves([]);
      return;
    }

    await handleMove(selected, { r, c });
    setSelected(null);
    setLegalMoves([]);
  }

  async function handleMove(from, to) {
    try {
      const res = await fetch("http://localhost:8000/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          from,
          to,
          fen: position,
        }),
      });

      if (!res.ok) {
        console.error("Move request failed:", res.status);
        return;
      }

      const result = await res.json();

      if (!result.legal) {
        console.log("Illegal move:", result.reason);
        return;
      }

      // Update FEN
      setPosition(result.fen);

      // Captures
      if (result.captured) {
        if (result.captured === result.captured.toUpperCase()) {
          setCapturedWhite((prev) => [...prev, result.captured]);
        } else {
          setCapturedBlack((prev) => [...prev, result.captured]);
        }
      }

      // Turn update
      if (result.turn === "white" || result.turn === "black") {
        setCurrentTurn(result.turn);
      }

    } catch (err) {
      console.error("Error calling move API:", err);
    }
  }

  /* -------- RENDER -------- */

  if (!position) return <h2>Loading game...</h2>;

  return (
    <div>
      <h1>Dean’s Chess App</h1>

      {/* Black ends turn → White starts */}
      <div style={{ display: "flex", justifyContent: "center", marginBottom: "10px" }}>
        <button
          onClick={() => {
            if (!gameOver) setCurrentTurn("white");
          }}
        >
          Black Move (End Turn)
        </button>
      </div>

      <Chessboard
        position={position}
        selected={selected}
        legalMoves={legalMoves}
        onSquareClick={onSquareClick}
        capturedWhite={capturedWhite}
        capturedBlack={capturedBlack}
        whiteTime={whiteTime}
        blackTime={blackTime}
      />

      {/* White ends turn → Black starts */}
      <div style={{ display: "flex", justifyContent: "center", marginTop: "10px" }}>
        <button
          onClick={() => {
            if (!gameOver) setCurrentTurn("black");
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

