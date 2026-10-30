#!/data/data/com.termux/files/usr/bin/bash

set -u

BACKEND_PID=""
FRONTEND_PID=""

cleanup() {
  echo "Stopping frontend and backend..."

  if [ -n "$FRONTEND_PID" ]; then
    kill "$FRONTEND_PID" 2>/dev/null || true
  fi

  if [ -n "$BACKEND_PID" ]; then
    kill "$BACKEND_PID" 2>/dev/null || true
  fi

  wait 2>/dev/null || true
}

trap cleanup EXIT INT TERM

echo "Starting Rust backend on http://localhost:3000..."
cargo watch -q -c -w src -w engine/src -x 'run --bin chess-api' &
BACKEND_PID=$!

cd frontend || exit 1

echo "Starting frontend..."
npm run dev &
FRONTEND_PID=$!

wait
