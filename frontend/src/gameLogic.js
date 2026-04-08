// gameLogic.js
// Logic + API layer for the Rust engine

// IMPORTANT: Termux WebView cannot reach localhost or 127.0.0.1.
// 0.0.0.0 is the only address guaranteed to work.
export const API_URL = "http://0.0.0.0:8000";

/* ---------------- LOW-LEVEL FETCH WRAPPERS ---------------- */

async function apiGet(path) {
  const res = await fetch(`${API_URL}${path}`);
  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`GET ${path} failed: ${res.status} ${text}`);
  }
  return res.json();
}

async function apiPost(path, body) {
  const res = await fetch(`${API_URL}${path}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const text = await res.text().catch(() => "");
    throw new Error(`POST ${path} failed: ${res.status} ${text}`);
  }
  return res.json();
}

/* ---------------- API CALLS ---------------- */

export async function fetchState() {
  return apiGet("/state");
}

export async function sendMove(from, to) {
  return apiPost("/move", {
    from: convertToEngineCoords(from),
    to: convertToEngineCoords(to),
  });
}

export async function fetchLegalMoves(square) {
  const engineSquare = convertToEngineCoords(square);
  return apiGet(`/legal-moves?from=${encodeURIComponent(engineSquare)}`);
}

export async function fetchBestMove(fen) {
  return apiPost("/best-move", { fen });
}

/* ---------------- FEN UTILITIES ---------------- */

export function fenToBoard(fenBoard) {
  const rows = fenBoard.split("/");
  return rows.map((row) =>
    row.replace(/[1-8]/g, (n) => ".".repeat(parseInt(n, 10))).split("")
  );
}

export function boardToFen(board) {
  return board
    .map((row) => row.join("").replace(/\.{1,8}/g, (m) => m.length.toString()))
    .join("/");
}

/* ---------------- COORDINATES ---------------- */

export function convertToEngineCoords({ r, c }) {
  const rank = 8 - r;
  const file = String.fromCharCode("a".charCodeAt(0) + c);
  return `${file}${rank}`;
}

export function convertFromEngineCoords(square) {
  const file = square[0];
  const rank = parseInt(square[1], 10);

  const c = file.charCodeAt(0) - "a".charCodeAt(0);
  const r = 8 - rank;

  return { r, c };
}

/* ---------------- LOCAL FALLBACK (OPTIONAL) ---------------- */

export function applyMoveLocally(position, from, to) {
  const [fenBoard, turn] = position.split(" ");
  const board = fenToBoard(fenBoard);

  const piece = board[from.r][from.c];
  const target = board[to.r][to.c];

  if (piece === ".") return null;

  const newBoard = board.map((row) => [...row]);
  newBoard[to.r][to.c] = piece;
  newBoard[from.r][from.c] = ".";

  const newFENBoard = boardToFen(newBoard);
  const nextTurn = turn === "w" ? "b" : "w";

  return {
    position: `${newFENBoard} ${nextTurn} - - 0 1`,
    capture: target !== "." ? target : null,
    nextTurn,
  };
}
