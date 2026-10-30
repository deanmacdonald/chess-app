import { test, expect, vi } from "vitest";
import { getFEN } from "../gameLogic";

test("returns the FEN string when the backend responds", async () => {
  const fetchMock = vi.fn(() =>
    Promise.resolve({ json: () => Promise.resolve({ fen: "startpos" }) }),
  );
  global.fetch = fetchMock;

  const fen = await getFEN();
  expect(fen).toBe("startpos");
  expect(fetchMock).toHaveBeenCalledWith(
    "https://black-knight-production-1f67.up.railway.app/fen",
    expect.objectContaining({
      headers: expect.objectContaining({
        Accept: "application/json",
      }),
    }),
  );
});
