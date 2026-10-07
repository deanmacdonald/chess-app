import { afterEach, describe, expect, it, vi } from "vitest";
import { API_URL, makeMove } from "../gameLogic";

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("makeMove()", () => {
  it("sends the move and returns the backend response", async () => {
    const mockResponse = { fen: "new-fen-string" };

    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify(mockResponse), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      }),
    );

    vi.stubGlobal("fetch", fetchMock);

    await expect(makeMove("e2", "e4")).resolves.toEqual(mockResponse);

    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledWith(`${API_URL}/move`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ from: "e2", to: "e4" }),
    });
  });

  it("throws an error when the backend rejects the move", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(
        new Response(JSON.stringify({ error: "Move failed" }), {
          status: 400,
          headers: { "Content-Type": "application/json" },
        }),
      ),
    );

    await expect(makeMove("e2", "e4")).rejects.toThrow(
      "API request to /move failed: Move failed",
    );
  });
});
