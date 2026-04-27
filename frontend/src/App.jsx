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

  useEffect(() => {
    if (gameOver) return;

    const interval = setInterval(() => {
      setWhiteTime((t) => (whiteRunning ? Math.max(t - 1, 0) : t));
      setBlackTime((t) => (blackRunning ? Math.max(t - 1, 0) : t));
    }, 1000);

    return () => clearInterval(interval);
  }, [whiteRunning, blackRunning, gameOver]);

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

    if (!selected) {
      setSelected({ r, c });
      setLegalMoves([]);
      return;
    }

    const from = selected;
    const to = { r, c };

    setSelected(null);
    setLegalMoves([]);

    await handleMove(from, to);
  }

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
    }
  }

  /* -------- RENDER -------- */

  if (!position) return <h2>Loading game...</h2>;

  return (
    <div>
      <h1>Dean’s Chess App</h1>

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

      {gameOver && (
        <h2 style={{ marginTop: "20px" }}>
          Game Over — {whiteTime === 0 ? "Black wins" : "White wins"}
        </h2>
      )}
    </div>
  );
}
