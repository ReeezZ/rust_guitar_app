#!/bin/bash

# 🎸 Guitar Practice App - Development Server
# Starts both frontend (Leptos/Trunk) and backend (Axum) concurrently

echo "🎸 Starting Guitar Practice App Development Environment"
echo ""

# Function to cleanup background processes on exit
cleanup() {
    echo ""
    echo "🛑 Shutting down development servers..."
    jobs -p | xargs -r kill
    exit 0
}

# Set up cleanup trap
trap cleanup SIGINT SIGTERM

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(dirname "$SCRIPT_DIR")"

# Start backend in background
echo "🔧 Starting backend server (http://127.0.0.1:8080)..."
(cd "$WORKSPACE_ROOT/backend" && cargo run) &
BACKEND_PID=$!

# Give backend time to start
sleep 2

# Start frontend in background  
echo "🎨 Starting frontend server (http://127.0.0.1:3010)..."
(cd "$WORKSPACE_ROOT/frontend" && RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open) &
FRONTEND_PID=$!

echo ""
echo "✅ Development servers started!"
echo ""
echo "📍 Services:"
echo "   🎨 Frontend: http://127.0.0.1:3010"
echo "   🔧 Backend:  http://127.0.0.1:8080"
echo "   📝 API:      http://127.0.0.1:8080/api/exercises"
echo ""
echo "📋 Press Ctrl+C to stop both servers"
echo ""

# Wait for any process to complete
wait
