import { useState, useEffect } from "react";
import Chessboard from "./Chessboard.jsx";
import { fetchState, sendMove, fetchLegalMoves } from "./gameLogic.js";

export default function App() {
  const [position, setPosition] = useState(null);
  const [selected, setSelected] = useState(null);
  const [legalMoves, setLegalMoves] = useState([]); // engine squares: ["e4","e5",...]

  const [capturedWhite, setCapturedWhite] = useState([]);
  const [capturedBlack, setCapturedBlack] = useState([]);

  const [whiteTime, setWhiteTime] = useState(300);
  const [blackTime, setBlackTime] = useState(300);
const gameOver = whiteTime === 0 || blackTime === 0;
  const [whiteRunning, setWhiteRunning] = useState(true);
  const [blackRunning, setBlackRunning] = useState(false);


  /* -------- INITIAL LOAD FROM API -------- */

  useEffect(() => {
    fetchState()
      .then((state) => {
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
      })
      .catch((err) => {
        console.error("Failed to load state:", err);
      });
  }, []);

  /* -------- TIMERS -------- */

  useEffect(() => {
    if (gameOver) return;

    const interval = setInterval(() => {
      if (whiteRunning) setWhiteTime((t) => Math.max(t - 1, 0));
      if (blackRunning) setBlackTime((t) => Math.max(t - 1, 0));
    }, 1000);

    return () => clearInterval(interval);
  }, [whiteRunning, blackRunning, gameOver]);

  useEffect(() => {
    if (whiteTime === 0 || blackTime === 0) {
      setWhiteRunning(false);
      setBlackRunning(false);
    }
  }, [whiteTime, blackTime]);

  /* -------- CLICK HANDLER -------- */

  async function onSquareClick(r, c) {
    if (gameOver || !position) return;

    // First click → select piece + fetch legal moves
    if (!selected) {
      const newSelected = { r, c };
      setSelected(newSelected);

      try {
        const moves = await fetchLegalMoves(newSelected); // ["e4","e5",...]
        setLegalMoves(moves);
      } catch (err) {
        console.error("Failed to fetch legal moves:", err);
        setLegalMoves([]);
      }

      return;
    }

    // Second click → attempt move
    const from = selected;
    const to = { r, c };

    setSelected(null);
    setLegalMoves([]);

    await handleMove(from, to);
  }

  /* -------- MOVE HANDLER (API) -------- */

  async function handleMove(from, to) {
    try {
      const state = await sendMove(from, to);

      setPosition(state.fen);
      setCapturedWhite(state.captured_white || []);
      setCapturedBlack(state.captured_black || []);

      if (state.turn === "w") {
        setBlackRunning(false);
        setWhiteRunning(true);
      } else {
        setWhiteRunning(false);
        setBlackRunning(true);
      }

      if (state.game_over) {
        setGameOver(true);
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
