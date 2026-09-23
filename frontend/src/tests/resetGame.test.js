import { describe, it, expect, vi } from "vitest";
import { resetGame, API_URL } from "../gameLogic";

describe("resetGame()", () => {
  it("resets the game and returns JSON", async () => {
    const mockReset = { status: "ok" };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockReset),
    });

    const result = await resetGame();
    expect(result).toEqual(mockReset);
    expect(fetch).toHaveBeenCalledWith(`${API_URL}/reset`, {
      method: "POST",
    });
  });

  it("returns error object when reset fails", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    const result = await resetGame();
    expect(result.error).toBe("Reset failed");
  });
});
