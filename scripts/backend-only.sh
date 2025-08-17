#!/bin/bash

# 🔧 Backend-only development server  
# For backend API development and testing

echo "🔧 Starting Backend Development Server"
echo ""

cd backend || exit 1

echo "📍 Backend API will be available at: http://127.0.0.1:8080"
echo "📝 API endpoints: http://127.0.0.1:8080/api/exercises"
echo "📋 Press Ctrl+C to stop"
echo ""

cargo run
