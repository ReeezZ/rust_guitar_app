#!/bin/bash

# 🎨 Frontend-only development server
# For when you only need the frontend running

echo "🎨 Starting Frontend Development Server"
echo ""

cd frontend || exit 1

echo "📍 Frontend will be available at: http://127.0.0.1:3010"
echo "📋 Press Ctrl+C to stop"
echo ""

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open
