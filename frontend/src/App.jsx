import { useState, useEffect } from "react";
import Chessboard from "./Chessboard.jsx";
import { fetchBoard, makeMove, resetGame } from "./gameLogic.js";

export default function App() {
  /* -------- STATE -------- */

  const [position, setPosition] = useState(null);
  const [selected, setSelected] = useState(null);
  const [legalMoves, setLegalMoves] = useState([]);

  const [capturedWhite, setCapturedWhite] = useState([]);
  const [capturedBlack, setCapturedBlack] = useState([]);

<<<<<<< HEAD
  const [whiteTime, setWhiteTime] = useState(300);
  const [blackTime, setBlackTime] = useState(300);

  const gameOver = whiteTime === 0 || blackTime === 0;

  const [whiteRunning, setWhiteRunning] = useState(true);
  const [blackRunning, setBlackRunning] = useState(false);

  /* -------- INITIAL LOAD FROM API -------- */

  useEffect(() => {
    let mounted = true;

    fetchBoard()
      .then((state) => {
        if (!mounted) return;

        setPosition(state.fen);
        setCapturedWhite(state.captured_white || []);
        setCapturedBlack(state.captured_black || []);

        setWhiteTime(state.white_time ?? 300);
        setBlackTime(state.black_time ?? 300);

        if (state.turn === "w") {
          setWhiteRunning(true);
          setBlackRunning(false);
        } else {
          setWhiteRunning(false);
          setBlackRunning(true);
        }

        if (state.game_over) {
          setWhiteRunning(false);
          setBlackRunning(false);
        }
      })
      .catch((err) => console.error("Failed to load state:", err));

    return () => {
      mounted = false;
    };
  }, []);

  /* -------- TIMER EFFECT -------- */
=======
  // Clocks
  const [whiteTime, setWhiteTime] = useState(300);
  const [blackTime, setBlackTime] = useState(300);

  // Whose turn is running the clock
  const [currentTurn, setCurrentTurn] = useState("white");

  const gameOver = whiteTime === 0 || blackTime === 0;

  // Derived running state
  const whiteRunning = !gameOver && currentTurn === "white" && whiteTime > 0;
  const blackRunning = !gameOver && currentTurn === "black" && blackTime > 0;
>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)

  useEffect(() => {
    if (gameOver) return;

    const interval = setInterval(() => {
<<<<<<< HEAD
      setWhiteTime((t) => (whiteRunning ? Math.max(t - 1, 0) : t));
      setBlackTime((t) => (blackRunning ? Math.max(t - 1, 0) : t));
=======
      if (whiteRunning) {
        setWhiteTime((t) => Math.max(t - 1, 0));
      }
      if (blackRunning) {
        setBlackTime((t) => Math.max(t - 1, 0));
      }
>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)
    }, 1000);

    return () => clearInterval(interval);
  }, [whiteRunning, blackRunning, gameOver]);

<<<<<<< HEAD
  /* -------- GAME OVER CHECK (lint‑safe) -------- */

  useEffect(() => {
    if (!gameOver && (whiteTime === 0 || blackTime === 0)) {
      Promise.resolve().then(() => {
        setWhiteRunning(false);
        setBlackRunning(false);
      });
    }
  }, [whiteTime, blackTime, gameOver]);

  /* -------- CLICK HANDLER -------- */

  async function onSquareClick(r, c) {
    if (gameOver || !position) return;
=======
  // Handle board clicks
  async function onSquareClick(r, c) {
    if (gameOver) return;
>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)

    if (!selected) {
      setSelected({ r, c });
      setLegalMoves([]);
      return;
    }

<<<<<<< HEAD
    const from = selected;
    const to = { r, c };

=======
    await handleMove(selected, { r, c });
>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)
    setSelected(null);
    setLegalMoves([]);

    await handleMove(from, to);
  }

<<<<<<< HEAD
  /* -------- MOVE HANDLER -------- */

  async function handleMove(from, to) {
    try {
      const state = await makeMove(from, to);

      setPosition(state.fen);
      setCapturedWhite(state.captured_white || []);
      setCapturedBlack(state.captured_black || []);

      if (state.turn === "w") {
        setWhiteRunning(true);
        setBlackRunning(false);
      } else {
        setWhiteRunning(false);
        setBlackRunning(true);
      }

      if (state.game_over) {
        setWhiteRunning(false);
        setBlackRunning(false);
      }
    } catch (err) {
      console.error("Move rejected:", err);
=======
  // Call Rust engine to validate and apply move
  async function handleMove(from, to) {
    try {
      const res = await fetch("http://localhost:8000/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          from,      // { r, c }
          to,        // { r, c }
          fen: position,
        }),
      });

      if (!res.ok) {
        console.error("Move request failed with status", res.status);
        return;
      }

      const result = await res.json();

      if (!result.legal) {
        console.log("Illegal move:", result.reason);
        return;
      }

      // Update FEN from engine
      setPosition(result.fen);

      // Update captured pieces if any
      if (result.captured) {
        if (result.captured === result.captured.toUpperCase()) {
          setCapturedWhite((prev) => [...prev, result.captured]);
        } else {
          setCapturedBlack((prev) => [...prev, result.captured]);
        }
      }

      // Update turn from engine
      if (result.turn === "white" || result.turn === "black") {
        setCurrentTurn(result.turn);
      }

      // Optionally stop clocks if game over
      if (result.game_over) {
        // You can add extra UI here if needed
      }
    } catch (err) {
      console.error("Error calling move API:", err);
>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)
    }
  }

  /* -------- RENDER -------- */

  if (!position) return <h2>Loading game...</h2>;

  return (
    <div>
      <h1>Dean’s Chess App</h1>

<<<<<<< HEAD
=======
      {/* Black ends turn → White starts (manual override if you keep it) */}
      <div style={{ display: "flex", justifyContent: "center", marginBottom: "10px" }}>
        <button
          onClick={() => {
            if (gameOver) return;
            setCurrentTurn("white");
          }}
        >
          Black Move (End Turn)
        </button>
      </div>

>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)
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

<<<<<<< HEAD
=======
      {/* White ends turn → Black starts (manual override if you keep it) */}
      <div style={{ display: "flex", justifyContent: "center", marginTop: "10px" }}>
        <button
          onClick={() => {
            if (gameOver) return;
            setCurrentTurn("black");
          }}
        >
          White Move (End Turn)
        </button>
      </div>

>>>>>>> 00c7ae2 (Frontend + engine updates, removed old styles.css)
      {gameOver && (
        <h2 style={{ marginTop: "20px" }}>
          Game Over — {whiteTime === 0 ? "Black wins" : "White wins"}
        </h2>
      )}
    </div>
  );
}

