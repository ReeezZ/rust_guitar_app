# Guitar Practice Tracker - Vision & Roadmap

## Vision
Build a **personal guitar practice tracker** that makes it easy to log practice sessions, track progress over time, and stay motivated through simple but effective progress visualization.

**Philosophy:** Start simple, grow iteratively, focus on actual usage over feature complexity.

---

## Current Status - August 16, 2025

### ✅ Completed
- **Exercise Management** - Create, edit, delete exercises (scales, triads, techniques, songs)
  - This will be changed again, things like scales and triads dont have to be created individually, then can just be selected
  - 
- **Practice Timer** - Count-up timer with target time and visual feedback  
- **Metronome** - Adjustable BPM (30-250), 4/4 time, accent on beat 1, reactive updates
- **Practice Session** - Combined timer + metronome with show/hide toggle

### Exercise Routine Tracking ideas
- Exercise Type chord change
  - No chords yet, yould need chord displaying 

### 🔜 Next Up
- **Session Logging** - Auto-save practice sessions with exercise, duration, BPM, timestamp
- **Session History** - View past practice sessions chronologically

### 🔮 Future Ideas
- Setting goals
   - Choose a set of exercises as goals
- **Enhanced Metronome** - Multiple sounds, time signatures, volume control
- **Fretboard Integration** - Visual exercise display during practice
- **Backend & Analytics** - Data sync, progress tracking, charts
- **Routines** - Group exercises into practice routines
- **Gamification** - Streaks, achievements, progress visualization
- **Mobile/PWA** - Mobile-optimized experience

---

## Development Phases

### Phase 1.5: Session Logging (Current)
**Goal:** Complete core practice loop with session persistence

**Remaining:**
- Practice session model (ID, exercise, timestamps, duration, BPM)
- Auto-save sessions on timer start/stop
- Session history UI with filtering

### Phase 2: Polish & Enhancement
- **2A: Metronome Polish** - Multiple sounds, time signatures, volume control
- **2B: Fretboard Integration** - Visual exercise display during practice  
- **2C: Exercise Management** - Templates, categories, better UX

### Phase 3: Backend & Analytics
- Backend API with Axum + SQLite
- Practice analytics and progress tracking
- Self-hosted deployment



#### Backend Features
- **Data Persistence**
  - Backend API using Axum framework
  - Simple password-protected access (no public registration)
  - Self-hosted solution with SQLite or similar simple database
  - Optional: Friend accounts for sharing on one backend instance instances
  
- **Basic Analytics**
  - Practice consistency tracking
  - Total time per exercise type
  - Progress over time (duration, frequency)
  - Simple charts and graphs

- **Ideas to consider**
  - From react i know tan stack query, should we use something similar? or is that too much?

#### Architecture Notes
- **Project Structure**: Migrate to Rust workspace with multiple crates
  - `shared` crate: Domain models (Exercise, Note, Scale) + DTOs for serialization
  - `backend` crate: Axum server, use cases, business logic
  - `frontend` crate: Current Leptos app (renamed from root)
  - `infrastructure` crate: Database, external APIs (optional initially)
- **Clean Architecture**: Separate domain models from serialization concerns using DTOs
- **Storage Abstraction**: Design local storage that can be swapped for backend API calls
- **Self-hosting**: Research simple deployment options (Axum + SQLite/PostgreSQL)
- **Data Migration**: Convert current localStorage to DTO pattern, then to backend
- **GDPR Considerations**: Friend accounts research needed for multi-user features

---

## Core Workflow
1. Select Exercise → 2. Start Practice Session (Timer + Metronome) → 3. Auto-save Session → 4. Review Progress

---


## Future Ideas (Rough Concepts)

### Gamification & Motivation
- **Practice Streaks** - Daily/weekly practice tracking
- **Achievement System** - Unlock badges for milestones
- **Profile View** - Most practiced keys, exercise types, etc.
- **Progress Visualization** - Heat maps, progress bars
- **Goal Setting** - Daily/weekly practice targets

### Advanced Training Integration
- **Training Mode Integration** - Track fretboard trainer performance
- **Key Tracking** - Visual representation of keys practiced
- **Position Mastery** - Track progress across fret positions
- **Exercise Recommendations** - Suggest exercises based on practice gaps

### Mobile & Notifications
- **PWA Conversion** - Full mobile app experience
- **Practice Reminders** - Configurable notifications
- **Offline Sync** - Practice tracking without internet
- **Mobile-Optimized UI** - Touch-friendly controls

### Advanced Audio Features
- **Metronome Variations**
  - Different time signatures (3/4, 6/8, etc.)
  - Complex accent patterns
  - Tempo ramping (gradual speed increases)
  - Subdivision options (eighth notes, triplets)
  
- **Drum Machine** - Configurable drum patterns instead of basic metronome

### Social & Sharing Features
- **Practice Sharing** - Share/export routines with others

### Advanced Analytics
- **Detailed Metrics**
  - Tempo progression tracking
  - Difficulty ratings over time
  - Practice session quality trends
  - Weak area identification
  
- **Smart Recommendations**
  - Suggest practice focus based on data
  - Recommend new exercises
  - Adaptive routine generation

### Integration Features
- **MIDI Integration** - Use keyboard for scale degree input
- **Audio Analysis** - Pitch detection for tuning help
- **Tuner Integration** - Built-in guitar tuner
- **Circle of Fifths** - Interactive music theory tools for learning the CoF

---


## Technical Stack
- **Frontend:** Leptos 0.7 + WASM + Tailwind CSS
- **Storage:** localStorage → Backend (Phase 3)
- **Audio:** Web Audio API (square wave metronome)
- **State:** Leptos signals with reactive memos

---

*Updated August 16, 2025*
