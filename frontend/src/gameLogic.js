// -------------------------------
// Backend URLs
// -------------------------------

// Your NEW Cloudflare Tunnel URL
export const CLOUDFLARE_URL =
  "https://eng-swing-heat-cheaper.trycloudflare.com";

// Local backend for development
export const LOCAL_URL = "http://0.0.0.0:8000";

// Auto-switch:
// - Dev mode → LOCAL backend
// - Production → Cloudflare tunnel
export const API_URL = import.meta.env.DEV ? LOCAL_URL : CLOUDFLARE_URL;

// -------------------------------
// Fetch the current board state
// -------------------------------
export async function fetchBoard() {
  try {
    const res = await fetch(`${API_URL}/board`);
    if (!res.ok) throw new Error("Backend returned an error");
    return await res.json();
  } catch (err) {
    console.error("Failed to fetch board:", err);
    return { error: "Failed to load board" };
  }
}

// -------------------------------
// Make a move + send webhook
// -------------------------------
export async function makeMove(from, to) {
  try {
    const res = await fetch(`${API_URL}/move`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ from, to }),
    });

    if (!res.ok) throw new Error("Move request failed");

    const data = await res.json();

    // Fire webhook to Railway backend
    try {
      await fetch(`${import.meta.env.VITE_API_URL}/webhook`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          event: "move",
          data: {
            from,
            to,
            fen: data.fen,
          },
        }),
      });
    } catch (webhookErr) {
      console.warn("Webhook failed:", webhookErr);
    }

    return data;
  } catch (err) {
    console.error("Move failed:", err);
    return { error: "Move failed" };
  }
}

// -------------------------------
// Reset the game
// -------------------------------
export async function resetGame() {
  try {
    const res = await fetch(`${API_URL}/reset`, {
      method: "POST",
    });

    if (!res.ok) throw new Error("Reset failed");

    return await res.json();
  } catch (err) {
    console.error("Reset failed:", err);
    return { error: "Reset failed" };
  }
}
