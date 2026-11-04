const configuredApiUrl = import.meta.env.VITE_API_URL?.trim();

// Production uses the same-origin Vercel proxy.
// Local development can use VITE_API_URL or localhost:3000.
export const API_URL = import.meta.env.PROD
  ? "/api"
  : configuredApiUrl || "http://localhost:3000";

function apiUrl(path) {
  const baseUrl = API_URL.endsWith("/") ? API_URL.slice(0, -1) : API_URL;

  const normalizedPath = path.startsWith("/") ? path : `/${path}`;

  return `${baseUrl}${normalizedPath}`;
}

async function request(path, options = {}) {
  const url = apiUrl(path);
  let response;

  try {
    response = await fetch(url, {
      ...options,
      headers: {
        Accept: "application/json",
        ...options.headers,
      },
    });
  } catch (error) {
    throw new Error(
      `Cannot reach the chess API at ${url}. ` +
        (error instanceof Error ? error.message : "Network request failed."),
    );
  }

  const contentType = response.headers.get("content-type") || "";
  const isJson = contentType.includes("application/json");

  let data = null;
  let text = "";

  try {
    if (isJson) {
      data = await response.json();
    } else {
      text = await response.text();
    }
  } catch {
    throw new Error(`API request to ${url} returned unreadable data.`);
  }

  if (!response.ok) {
    const message =
      data?.error ||
      data?.message ||
      text ||
      response.statusText ||
      "Request failed";

    throw new Error(
      `API request to ${url} failed (${response.status}): ${message}`,
    );
  }

  if (!data || typeof data !== "object") {
    throw new Error(`API request to ${url} did not return JSON.`);
  }

  return data;
}

export async function getFEN() {
  const data = await request("/fen");

  if (typeof data.fen !== "string") {
    throw new Error(
      'The "/fen" response did not contain a valid "fen" string.',
    );
  }

  return data.fen;
}

export async function getLegalMoves() {
  const data = await request("/legal_moves");

  if (!Array.isArray(data.legal_moves)) {
    throw new Error(
      'The "/legal_moves" response did not contain a "legal_moves" array.',
    );
  }

  return data.legal_moves;
}

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

export async function resetGame() {
  return request("/reset", {
    method: "POST",
  });
}
