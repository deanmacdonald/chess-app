import { test, expect, vi } from "vitest";
import { resetGame } from "../gameLogic";

test("resets the game and returns the backend response", async () => {
  const fetchMock = vi.fn(() =>
    Promise.resolve({ json: () => Promise.resolve({ ok: true }) }),
  );
  global.fetch = fetchMock;

  const res = await resetGame();
  expect(res.ok).toBe(true);
  expect(fetchMock).toHaveBeenCalledWith(
    "https://black-knight-production-1f67.up.railway.app/reset",
    expect.objectContaining({
      method: "POST",
      headers: expect.objectContaining({
        Accept: "application/json",
      }),
    }),
  );
});
