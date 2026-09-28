export const API_URL = import.meta.env.VITE_API_URL;

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
