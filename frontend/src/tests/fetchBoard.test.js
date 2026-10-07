import { afterEach, describe, expect, it, vi } from "vitest";
import { API_URL, getFEN } from "../gameLogic";

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("getFEN()", () => {
  it("returns the FEN string when the backend responds", async () => {
    const fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify({ fen }), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      }),
    );

    vi.stubGlobal("fetch", fetchMock);

    await expect(getFEN()).resolves.toBe(fen);

    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledWith(`${API_URL}/fen`, {});
  });

  it("throws an error when the backend rejects the request", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ error: "Failed to load position" }), {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }),
      ),
    );

    await expect(getFEN()).rejects.toThrow(
      "API request to /fen failed: Failed to load position",
    );
  });
});
