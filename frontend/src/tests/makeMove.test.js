import { test, expect, vi } from "vitest";
import { makeMove } from "../gameLogic";

test("sends the move and returns the backend response", async () => {
  const fetchMock = vi.fn(() =>
    Promise.resolve({ json: () => Promise.resolve({ ok: true }) }),
  );
  global.fetch = fetchMock;

  const res = await makeMove("e2", "e4");
  expect(res.ok).toBe(true);
  expect(fetchMock).toHaveBeenCalledWith(
    "https://black-knight-production-1f67.up.railway.app/move",
    expect.objectContaining({
      method: "POST",
      headers: expect.objectContaining({
        "Content-Type": "application/json",
        Accept: "application/json",
      }),
      body: JSON.stringify({ from: "e2", to: "e4" }),
    }),
  );
});
