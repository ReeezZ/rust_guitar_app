# Guitar Practice Tracker - Vision & Roadmap

## Vision Statement
Build a **personal guitar practice tracker** that makes it easy to log practice sessions, track progress over time, and stay motivated through simple but effective progress visualization. The app should be simple to use daily but provide rich long-term insights.

**Core Philosophy:** Start simple, grow iteratively, focus on actual usage over feature complexity.

---

## User Story
*As a guitar player, I want to easily select what I'm practicing, start a timer with metronome, and automatically track my progress so that I can stay motivated and see my improvement over time.*

## Current Status - August 2025

### ✅ Completed (Phase 0.5)
- **Exercise Management Foundation**
  - Type-safe Exercise model with Scale, Triad, Technique, Song variants
  - Exercise creation, listing, deletion with localStorage persistence
  - Clean /exercises UI with form and navigation integration
  - WASM-compatible architecture with js-sys timestamps

### 🚧 In Progress (Phase 1)
- **Practice Timer & Session Logging** - Core functionality needed for actual practice tracking
- **Basic Metronome** - Essential for rhythm practice
- **Session History** - Track practice over time

### 🔮 Planned (Phase 2+)
- **Enhanced UI/UX** - Better exercise management, fretboard integration
- **Backend & Analytics** - Data persistence and progress tracking  
- **Advanced Features** - Routines, gamification, mobile support

## Core Workflow
1. **Select Exercise** ✅ - Choose what to practice (scales, triads, technique, songs)
2. **Configure Practice** 🚧 - Set key, position, target time (optional) 
3. **Start Session** 🚧 - Timer counts up, metronome provides rhythm
4. **Practice** 🚧 - Focus on playing, minimal app interaction needed
5. **Finish & Save** 🚧 - Session automatically logged with all details
6. **Review Progress** 🚧 - View practice history and trends over time

---

## Development Phases

### Phase 0.5: Exercise Management Foundation ✅ COMPLETE
**Goal:** Build exercise CRUD system as foundation for practice tracking  
**Status:** Completed August 2025 - Commit a951287

#### Completed Features ✅
- **Exercise Management**
  - Simple exercise types: Scales, Triads, Technique, Songs ✅
  - Each exercise has: name, type (key/position in data model but not UI yet) ✅
  - CRUD operations: Create ✅, Read ✅, Delete ✅ (Update pending)
  - Local storage persistence ✅
  
#### UI Components ✅
- Exercise list/selector ✅
- Simple exercise creation form ✅
- /exercises route with navigation ✅

#### Technical Implementation ✅
- Type-safe Exercise domain model with Rust enums ✅
- WASM-compatible ID generation using js-sys::Date ✅
- Simple localStorage functions (no complex traits) ✅
- Leptos 0.7 reactive components ✅

---

### Phase 1: Core Practice Tracking 🚧 NEXT
**Goal:** Complete basic timer + session tracking to make app actually useful for practice

#### Remaining Core Features
- **Practice Timer**
  - Count-up timer (shows elapsed time)
  - Optional target time (timer turns green when reached)
  - Start/pause/stop functionality
  
- **Basic Metronome**
  - Adjustable BPM (30-250)
  - Simple click sound
  - 4/4 time signature
  - Visual beat indicator (dots)
  
- **Session Logging**
  - Auto-save practice sessions with: exercise, duration, BPM, timestamp
  - Local storage persistence
  - Simple session history list

#### UI Components Needed
- Practice screen (timer + metronome controls)
- Basic session history view

---

### Phase 2A: Metronome Polish
**Goal:** Improve metronome functionality and user experience

#### Enhanced Metronome Features
- **Multiple sound options** - Different click/beep sounds
- **Accent on beat 1** - Stronger first beat in 4/4 time
- **Mid-session BPM adjustment** - Change tempo during practice with tracking
- **Visual improvements** - Better beat indicator visualization

---

### Phase 2B: Fretboard Integration  
**Goal:** Integrate existing fretboard system with practice sessions

#### Fretboard Features
- **Compact fretboard view** - Solve current layout space issues
- **Exercise-specific visualization** - Show relevant scales/triads during practice
- **Position integration** - Display selected fret ranges clearly

---

### Phase 2C: Exercise Management Polish
**Goal:** Improve exercise creation and organization

#### Exercise Management Features
- **Exercise templates/presets** - Common scales, triads, techniques
- **Quick exercise creation** - Streamlined creation workflow
- **Exercise categories** - Basic grouping and filtering

---

### Phase 3: Backend & Analytics
**Goal:** Add backend persistence and basic progress analytics

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

#### Architecture Notes
- Design local storage abstraction that can be swapped for backend
- Research simple self-hosting options (Axum + database choice)
- GDPR considerations for friend accounts (research needed)
- Data migration likely not needed (minimal local data expected)

---

### Phase 4: Routines & Organization
**Goal:** Add routine management without over-complicating UX

#### Routine Features
- **Simple Routines**
  - List of exercises (not sequences)
  - Estimated total time
  - Quick routine start
  
- **Routine Management**
  - Easy routine creation/editing
  - Routine templates
  - Reusable exercises across routines
  
- **Enhanced Organization**
  - Exercise categories and tagging
  - Routine categorization
  - Search and filtering

#### UI Focus
- Avoid Justin Guitar's navigation problems
- Show routine contents at a glance
- In-context editing where possible

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
- **Audio Recording** - Record practice sessions for review

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
- **Circle of Fifths** - Interactive music theory tools

---

## Technical Architecture

### Current Foundation
- **Frontend:** Leptos + WASM + Tailwind CSS
- **Storage:** Local storage → Backend (Phase 3)
- **Audio:** Web Audio API for metronome
- **Timing:** Performance API for precision

### Key Architectural Decisions
1. **Exercise Type System** - Use Rust enums for type safety and extensibility
2. **Storage Abstraction** - Abstract storage layer for easy local→backend migration
3. **Component Reuse** - Leverage existing SVG fretboard system
4. **Offline-First** - Work without internet, sync when available (nice to have)

### Development Principles
- **Iterative Development** - Each phase should be fully functional
- **User-Driven Features** - Only add complexity when proven needed
- **Performance Focus** - Smooth timer/metronome performance critical
- **Desktop-First Design** - Primary focus on desktop use, mobile support added later if needed

---

## Technical Challenges & Risks

### Known Technical Risks
- **Web Audio API Complexity** - Metronome timing precision is notoriously difficult in browsers
- **Timer Accuracy** - Browser tabs throttle timers when not active, affecting practice session tracking
- **Fretboard Layout Integration** - Current fretboard "takes too much space" - layout challenges
- **Storage Abstraction** - Abstracting local→backend migration may be more complex than anticipated
- **Audio Latency** - Web Audio API latency may affect metronome usability
- **Browser Compatibility** - Web Audio API support varies across browsers/devices

### Mitigation Strategies
- Start with simple timer/metronome, iterate based on real usage
- Test timer behavior extensively in background tabs
- Design fretboard integration as optional/collapsible
- Keep storage abstraction simple initially
- Research Web Audio API best practices early

---

## Success Criteria

### Phase 1 Success
- Can create exercises and log practice sessions
- Timer and metronome work reliably
- Data persists across browser sessions
- Basic history view shows past practice

### Phase 2A Success (Metronome Polish)
- Metronome feels professional and accurate
- Multiple sound options work well
- BPM changes during practice are tracked properly

### Phase 2B Success (Fretboard Integration)
- Fretboard display enhances practice experience
- Layout issues resolved (compact, usable design)
- Exercise-specific visualization works smoothly

### Phase 2C Success (Exercise Management)
- Exercise creation is quick and intuitive
- Templates speed up common exercise setup
- Organization features reduce friction

### Phase 3 Success
- Backend migration is seamless
- Basic analytics provide motivational insights
- Data is safe and accessible across devices
- Self-hosting setup is straightforward

### Phase 4 Success
- Routine management solves real workflow problems
- App becomes primary practice tracking tool
- UX clearly superior to existing solutions

----

*This vision captures current thinking as of August 2025. Expected to evolve as development progresses and usage patterns emerge.*
