import { useEffect, useMemo, useState } from "react";
import { getFEN, getLegalMoves, makeMove, resetGame } from "./gameLogic.js";
import "./App.css";

const STARTING_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const PIECES = {
  K: "♔",
  Q: "♕",
  R: "♖",
  B: "♗",
  N: "♘",
  P: "♙",
  k: "♚",
  q: "♛",
  r: "♜",
  b: "♝",
  n: "♞",
  p: "♟",
};

function fenToBoard(fen) {
  if (typeof fen !== "string") {
    throw new Error("Expected a FEN string.");
  }

  const rows = fen.trim().split(/\s+/)[0].split("/");

  if (rows.length !== 8) {
    throw new Error(`Invalid FEN: expected 8 rows, received ${rows.length}.`);
  }

  return rows.map((row) => {
    const expanded = [];

    for (const char of row) {
      if (/^[1-8]$/.test(char)) {
        expanded.push(...Array(Number(char)).fill(null));
      } else if (/^[prnbqkPRNBQK]$/.test(char)) {
        expanded.push(char);
      } else {
        throw new Error(`Invalid FEN character: ${char}`);
      }
    }

    if (expanded.length !== 8) {
      throw new Error(
        `Invalid FEN row: expected 8 squares, received ${expanded.length}.`,
      );
    }

    return expanded;
  });
}

function isWhitePiece(piece) {
  return typeof piece === "string" && /^[A-Z]$/.test(piece);
}

function isBlackPiece(piece) {
  return typeof piece === "string" && /^[a-z]$/.test(piece);
}

function sideForPiece(piece) {
  if (isWhitePiece(piece)) return "White";
  if (isBlackPiece(piece)) return "Black";
  return null;
}

function turnFromFEN(fen) {
  const activeColor = fen.trim().split(/ +/)[1];

  if (activeColor !== "w" && activeColor !== "b") {
    throw new Error("Invalid FEN: missing or invalid active color.");
  }

  return activeColor === "b" ? "Black" : "White";
}
export default function Chessboard() {
  const [board, setBoard] = useState(() => fenToBoard(STARTING_FEN));
  const [selected, setSelected] = useState(null);
  const [legalSquares, setLegalSquares] = useState([]);
  const [turn, setTurn] = useState("White");
  const [status, setStatus] = useState("Ongoing");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(true);
  const [busy, setBusy] = useState(false);
  const [flipped, setFlipped] = useState(false);

  const displayRows = useMemo(() => {
    const rows = [];

    for (let r = 0; r < 8; r += 1) {
      for (let c = 0; c < 8; c += 1) {
        const boardRow = flipped ? 7 - r : r;
        const boardCol = flipped ? 7 - c : c;
        const piece = board[boardRow][boardCol];
        const square = "abcdefgh"[boardCol] + (8 - boardRow);

        rows.push({
          piece,
          square,
          row: r,
          col: c,
        });
      }
    }

    return rows;
  }, [board, flipped]);

  function applyGameResult(result) {
    if (!result || typeof result.fen !== "string") {
      throw new Error("The chess API did not return a valid FEN position.");
    }

    const nextBoard = fenToBoard(result.fen);
    const nextTurn = turnFromFEN(result.fen);

    setBoard(nextBoard);
    setTurn(nextTurn);
    setStatus(result.game_status ?? result.status ?? "Ongoing");
  }

  function clearSelection() {
    setSelected(null);
    setLegalSquares([]);
  }

  async function loadGame() {
    setLoading(true);
    clearSelection();
    setMessage("");

    try {
      const fen = await getFEN();
      applyGameResult({ fen, game_status: "Ongoing" });
    } catch (error) {
      console.error("Could not load game from Railway:", error);
      setMessage(
        error instanceof Error
          ? error.message
          : "Could not load the game from the chess backend.",
      );
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    queueMicrotask(loadGame);
  }, []);

  async function selectPiece(square, piece) {
    if (!piece) {
      return;
    }

    if (sideForPiece(piece) !== turn) {
      setMessage(`It is ${turn}'s turn.`);
      return;
    }

    setBusy(true);

    try {
      const legalMoves = await getLegalMoves();

      const movesFromSquare = legalMoves.filter(
        (move) =>
          typeof move === "string" &&
          move.length >= 4 &&
          move.slice(0, 2) === square,
      );

      setSelected(square);
      setLegalSquares(movesFromSquare.map((move) => move.slice(2, 4)));

      if (movesFromSquare.length === 0) {
        setMessage(`No legal moves available from ${square}.`);
      } else {
        setMessage("");
      }
    } catch (error) {
      console.error("Could not load legal moves:", error);
      clearSelection();
      setMessage(
        error instanceof Error
          ? error.message
          : "Could not load legal moves from the chess backend.",
      );
    } finally {
      setBusy(false);
    }
  }

  async function submitMove(from, to) {
    if (!from || !to || from === to || busy) {
      return;
    }

    setBusy(true);
    setMessage("");

    try {
      const result = await makeMove(from, to);

      if (result?.result === "OK" || typeof result?.fen === "string") {
        applyGameResult(result);
      } else {
        setMessage(result?.message ?? "That move is not legal.");
      }
    } catch (error) {
      console.error("Move failed:", error);
      setMessage(
        error instanceof Error
          ? error.message
          : "Unable to submit the move to the chess backend.",
      );
    } finally {
      clearSelection();
      setBusy(false);
    }
  }

  async function handleSquareClick(square, piece) {
    if (loading || busy) {
      return;
    }

    if (!selected) {
      await selectPiece(square, piece);
      return;
    }

    if (square === selected) {
      clearSelection();
      setMessage("");
      return;
    }

    const clickedOwnPiece = piece && sideForPiece(piece) === turn;

    if (clickedOwnPiece) {
      await selectPiece(square, piece);
      return;
    }

    if (!legalSquares.includes(square)) {
      setMessage("That destination is not legal for the selected piece.");
      return;
    }

    await submitMove(selected, square);
  }

  function handleDragStart(event, square, piece) {
    if (loading || busy || sideForPiece(piece) !== turn) {
      event.preventDefault();
      return;
    }

    event.dataTransfer.setData("text/plain", square);
    event.dataTransfer.effectAllowed = "move";
    setSelected(square);
  }

  async function handleDrop(event, destination) {
    event.preventDefault();

    if (loading || busy) {
      return;
    }

    const from = event.dataTransfer.getData("text/plain");

    if (!from || from === destination) {
      clearSelection();
      return;
    }

    await submitMove(from, destination);
  }

  async function handleReset() {
    if (busy) {
      return;
    }

    setBusy(true);
    setMessage("");

    try {
      const result = await resetGame();
      applyGameResult(result);
    } catch (error) {
      console.error("Reset failed:", error);
      setMessage(
        error instanceof Error
          ? error.message
          : "Backend reset failed — displayed position left unchanged.",
      );
    } finally {
      clearSelection();
      setBusy(false);
    }
  }

  const visibleFiles = flipped
    ? ["h", "g", "f", "e", "d", "c", "b", "a"]
    : ["a", "b", "c", "d", "e", "f", "g", "h"];

  const visibleRanks = flipped
    ? ["1", "2", "3", "4", "5", "6", "7", "8"]
    : ["8", "7", "6", "5", "4", "3", "2", "1"];

  return (
    <main className="chess-container">
      <header className="game-header">
        <h1>Dean&apos;s Chess Game</h1>
        <p className="turn-label">
          Turn: <strong>{turn}</strong>
        </p>
        <p className="status-label">Status: {status}</p>
      </header>

      <div className="game-controls">
        <button type="button" onClick={handleReset} disabled={busy}>
          {busy ? "Working…" : "Reset Game"}
        </button>

        <button
          type="button"
          onClick={() => setFlipped((current) => !current)}
          disabled={busy}
        >
          Flip Board
        </button>

        <button type="button" onClick={loadGame} disabled={loading || busy}>
          {loading ? "Loading…" : "Reload Game"}
        </button>
      </div>

      <p className="game-help">
        Tap a piece, then tap a highlighted destination. Drag-and-drop is also
        enabled on browsers that support it.
      </p>

      {message && (
        <p className="game-message" role="alert">
          {message}
        </p>
      )}

      <div className="board-shell">
        <div className="rank-labels" aria-hidden="true">
          {visibleRanks.map((rank) => (
            <span key={rank}>{rank}</span>
          ))}
        </div>

        <div className="board" aria-label="Chessboard">
          {displayRows.map(({ piece, square, row, col }) => {
            const isLegal = legalSquares.includes(square);
            const isSelected = selected === square;
            const squareColor = (row + col) % 2 === 0 ? "light" : "dark";

            return (
              <button
                type="button"
                key={square}
                className={`square ${squareColor} ${
                  isSelected ? "selected" : ""
                } ${isLegal ? "legal" : ""}`}
                onClick={() => handleSquareClick(square, piece)}
                onDragOver={(event) => event.preventDefault()}
                onDrop={(event) => handleDrop(event, square)}
                disabled={loading || busy}
                aria-label={`${square}${piece ? ` ${piece}` : " empty"}`}
              >
                {piece && (
                  <span
                    className={`piece ${
                      isWhitePiece(piece) ? "white-piece" : "black-piece"
                    }`}
                    draggable={
                      !loading && !busy && sideForPiece(piece) === turn
                    }
                    onDragStart={(event) =>
                      handleDragStart(event, square, piece)
                    }
                    aria-hidden="true"
                  >
                    {PIECES[piece]}
                  </span>
                )}

                {isLegal && <span className="highlight" aria-hidden="true" />}
              </button>
            );
          })}
        </div>
      </div>

      <div className="file-labels" aria-hidden="true">
        {visibleFiles.map((file) => (
          <span key={file}>{file}</span>
        ))}
      </div>
    </main>
  );
}
