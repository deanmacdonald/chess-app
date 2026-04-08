#!/data/data/com.termux/files/usr/bin/sh

# Go to project root
cd ~/dev/chess-app

echo "Starting backend (Rust API)..."
cargo run --bin api &
BACKEND_PID=$!

# Wait for backend to bind to port 8000
sleep 2

echo "Starting frontend (Vite)..."
cd frontend
npm run dev

# When frontend exits, kill backend
echo "Stopping backend..."
kill $BACKEND_PID
