# 🎸 Rust Guitar Practice App

A full-stack guitar practice application built with Rust, featuring fretboard visualization, practice timers, metronome, and exercise management.

## 🏗️ Architecture

This project uses a **Rust workspace** with three crates:

- **`shared/`** - Domain models, music theory, and common utilities  
- **`backend/`** - Axum REST API server with exercise management
- **`frontend/`** - Leptos/WASM frontend with interactive fretboard components

## 🚀 Quick Start

### Development (Frontend + Backend)
```bash
./scripts/dev.sh
```
- **Frontend:** http://127.0.0.1:3010
- **Backend API:** http://127.0.0.1:8080/api/exercises

### Frontend Only
```bash
./scripts/frontend-only.sh
```

### Backend Only  
```bash
./scripts/backend-only.sh
```

## 🛠️ Manual Setup

### Backend Development
```bash
cd backend
cargo run
```

### Frontend Development
```bash
cd frontend  
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open
```

## 📁 Project Structure

```
rust_guitar_app/
├── shared/           # Domain models & music theory
│   ├── src/
│   │   ├── models/   # Exercise, practice session models
│   │   └── music/    # Notes, scales, intervals
├── backend/          # Axum REST API
│   ├── src/
│   │   └── main.rs   # API server with CRUD endpoints
├── frontend/         # Leptos/WASM frontend
│   ├── src/
│   │   ├── components/ # UI components
│   │   ├── pages/      # Route handlers
│   │   └── models/     # Frontend-specific models
├── scripts/          # Development utilities
└── BACKEND_MIGRATION_PLAN.md  # Implementation progress
```

## 🎯 Features

### ✅ Current Features
- **Exercise Management** - Create/edit scales, triads, techniques, songs
- **Practice Timer** - Count-up timer with target time tracking
- **Metronome** - Adjustable BPM (30-250), 4/4 time signature
- **Fretboard Visualization** - Interactive SVG fretboard with scale display
- **Practice Sessions** - Combined timer + metronome interface
- **Full-Stack Architecture** - Backend API with frontend integration ready

### 🔜 Coming Soon
- **Session Logging** - Automatic practice session tracking
- **Progress Analytics** - Practice consistency and improvement tracking
- **Enhanced Fretboard** - Multiple visualization modes
- **Database Integration** - Persistent storage (considering alternatives to SQL)

## 🧪 Technology Stack

- **Backend:** Axum, Tokio, Serde, in-memory storage (database TBD)
- **Frontend:** Leptos 0.7, WASM, Trunk, Tailwind CSS  
- **Shared:** Pure Rust domain models, music theory
- **Dev Tools:** Cargo workspace, concurrent development scripts

## 📋 API Endpoints

```
GET    /api/exercises     # List all exercises
POST   /api/exercises     # Create new exercise  
GET    /api/exercises/:id # Get specific exercise
PUT    /api/exercises/:id # Update exercise
DELETE /api/exercises/:id # Delete exercise
```

## 🔧 Development Notes

- The backend uses **in-memory storage** for development
- Database technology decision is **postponed** to explore alternatives
- Frontend gracefully degrades to **localStorage** if backend unavailable
- All crates build independently and as workspace

---

*For detailed implementation progress, see [`BACKEND_MIGRATION_PLAN.md`](BACKEND_MIGRATION_PLAN.md)*
