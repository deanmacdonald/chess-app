import { describe, it, expect, vi } from "vitest";
import { makeMove, API_URL } from "../gameLogic";

describe("makeMove()", () => {
  it("sends move to backend and returns updated data", async () => {
    const mockResponse = { fen: "new-fen-string" };

    // Mock backend move API
    global.fetch = vi
      .fn()
      .mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockResponse),
      })
      // Mock webhook call
      .mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({ ok: true }),
      });

    const result = await makeMove("e2", "e4");

    expect(result.fen).toBe("new-fen-string");
    expect(fetch).toHaveBeenCalledWith(`${API_URL}/move`, expect.any(Object));
  });

  it("returns error object when backend fails", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    const result = await makeMove("e2", "e4");
    expect(result.error).toBe("Move failed");
  });
});
