// -------------------------------
// Cloudflare Tunnel URL
// -------------------------------
export const CLOUDFLARE_URL =
  "https://wishing-nodes-assurance-amanda.trycloudflare.com";

// -------------------------------
// Local fallback (Wi‑Fi only)
// -------------------------------
export const LOCAL_URL = "http://0.0.0.0:8000";

// -------------------------------
// Active API URL
// -------------------------------
export const API_URL = CLOUDFLARE_URL || LOCAL_URL;

// -------------------------------
// Core game logic functions
// -------------------------------

export async function fetchBoard() {
  const res = await fetch(`${API_URL}/board`);
  return await res.json();
}

export async function makeMove(from, to) {
  const res = await fetch(`${API_URL}/move`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ from, to }),
  });

  return await res.json();
}

export async function resetGame() {
  const res = await fetch(`${API_URL}/reset`, {
    method: "POST",
  });

  return await res.json();
}
