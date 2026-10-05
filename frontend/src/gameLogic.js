// --------------------------------------
// Backend URL
//
// Vercel Production / Preview:
//   VITE_API_URL=https://black-knight-production.up.railway.app
//
// Local Vite development fallback:
//   http://localhost:3000
// --------------------------------------
export const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

// --------------------------------------
// Shared API request helper
// --------------------------------------
async function request(path, options = {}) {
  const url = `${API_URL}${path}`;

  let response;

  try {
    response = await fetch(url, options);
  } catch {
    throw new Error(
      `Cannot reach the chess API at ${API_URL}. ` +
        "Check that the Railway service is running and CORS is configured.",
    );
  }

  const contentType = response.headers.get("content-type") || "";

  let data = null;
  let text = "";

  if (contentType.includes("application/json")) {
    data = await response.json();
  } else {
    text = await response.text();
  }

  if (!response.ok) {
    const message =
      data?.error ||
      data?.message ||
      text ||
      `${response.status} ${response.statusText}`;

    throw new Error(`API request to ${path} failed: ${message}`);
  }

  if (!data) {
    throw new Error(`API request to ${path} did not return JSON.`);
  }

  return data;
}

// --------------------------------------
// Get current chess position as FEN
// --------------------------------------
export async function getFEN() {
  const data = await request("/fen");

  if (typeof data.fen !== "string") {
    throw new Error(
      'The "/fen" response did not contain a valid "fen" string.',
    );
  }

  return data.fen;
}

// --------------------------------------
// Get all legal moves
//
// Expected backend response:
// { "legal_moves": ["e2e4", "e2e3", ...] }
// --------------------------------------
export async function getLegalMoves() {
  const data = await request("/legal_moves");

  if (!Array.isArray(data.legal_moves)) {
    throw new Error(
      'The "/legal_moves" response did not contain a "legal_moves" array.',
    );
  }

  return data.legal_moves;
}

// --------------------------------------
// Make a strict chess move
//
// Example:
// makeMove("e2", "e4")
// --------------------------------------
export async function makeMove(from, to) {
  if (!from || !to) {
    throw new Error(
      "Both a source square and destination square are required.",
    );
  }

  return request("/move", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ from, to }),
  });
}

// --------------------------------------
// Reset the current game
// --------------------------------------
export async function resetGame() {
  return request("/reset", {
    method: "POST",
  });
}
