import { describe, it, expect, vi } from "vitest";
import { fetchBoard, API_URL } from "../gameLogic";

describe("fetchBoard()", () => {
  it("returns board JSON when backend responds", async () => {
    const mockBoard = { board: ["r", "n", "b"] };

    // Mock fetch
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockBoard),
    });

    const result = await fetchBoard();
    expect(result).toEqual(mockBoard);
    expect(fetch).toHaveBeenCalledWith(`${API_URL}/board`);
  });

  it("returns error object when backend fails", async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    const result = await fetchBoard();
    expect(result.error).toBe("Failed to load board");
  });
});
