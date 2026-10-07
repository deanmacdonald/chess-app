import { afterEach, describe, expect, it, vi } from "vitest";
import { API_URL, resetGame } from "../gameLogic";

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("resetGame()", () => {
  it("resets the game and returns the backend response", async () => {
    const mockReset = { status: "ok" };

    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify(mockReset), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      }),
    );

    vi.stubGlobal("fetch", fetchMock);

    await expect(resetGame()).resolves.toEqual(mockReset);

    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledWith(`${API_URL}/reset`, {
      method: "POST",
    });
  });

  it("throws an error when the backend rejects the reset", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ error: "Reset failed" }), {
          status: 500,
          headers: { "Content-Type": "application/json" },
        }),
      ),
    );

    await expect(resetGame()).rejects.toThrow(
      "API request to /reset failed: Reset failed",
    );
  });
});
