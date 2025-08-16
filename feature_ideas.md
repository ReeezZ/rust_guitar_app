# Guitar Practice Tracker - Vision & Roadmap

## Vision Statement
Build a **personal guitar practice tracker** that makes it easy to log practice sessions, track progress over time, and stay motivated through simple but effective progress visualization. The app should be simple to use daily but provide rich long-term insights.

**Core Philosophy:** Start simple, grow iteratively, focus on actual usage over feature complexity.

---

## User Story
*As a guitar player, I want to easily select what I'm practicing, start a timer with metronome, and automatically track my progress so that I can stay motivated and see my improvement over time.*

### 🚧 Current Status - August 2025

#### ✅ Completed (Phase 0.5 + Phase 1)
- **Exercise Management Foundation** ✅
  - Type-safe Exercise model with Scale, Triad, Technique, Song variants
  - Exercise creation, listing, deletion with localStorage persistence
  - Clean /exercises UI with form and navigation integration
  - WASM-compatible architecture with js-sys timestamps
  - Individual exercise loading and detail pages with real data

- **Practice Timer System** ✅
  - Count-up timer with 15-minute default target
  - Start/pause/stop functionality with leptos-use intervals
  - Visual feedback when target time reached
  - Integrated into exercise detail workflow
  - Clean layout with exercise info above timer

#### 🔜 Next Up (Phase 1.5: Session Logging)
- **Basic Metronome** - Essential for rhythm practice
- **Session History** - Track practice over time
- **Session Auto-save** - Automatic logging of practice sessions

#### 🔮 Planned (Phase 2+)
- **Enhanced UI/UX** - Better exercise management, fretboard integration
- **Backend & Analytics** - Data persistence and progress tracking  
- **Advanced Features** - Routines, gamification, mobile support

## Core Workflow
1. **Select Exercise** ✅ - Choose what to practice (scales, triads, technique, songs)
2. **View Exercise Details** ✅ - See exercise type, description, fret range
3. **Start Practice Session** ✅ - Timer counts up, visual target feedback
4. **Practice** ✅ - Focus on playing, minimal app interaction needed
5. **Stop Session** ✅ - Manual stop when practice complete
6. **Session Logging** 🔜 - Auto-save session with details (next phase)
7. **Review Progress** � - View practice history and trends over time

---

## Development Phases

### Phase 0.5: Exercise Management Foundation ✅ COMPLETE
**Goal:** Build exercise CRUD system as foundation for practice tracking  
**Status:** Complete - August 2025

#### ✅ Completed Features
- **Exercise Management**
  - Simple exercise types: Scales, Triads, Technique, Songs ✅
  - Each exercise has: name, type, optional description ✅
  - CRUD operations: Create ✅, List ✅, Delete ✅, Read ✅
  - Local storage persistence with serde serialization ✅
  - Individual exercise lookup by ID ✅
  
#### ✅ UI Components
- Exercise list/selector ✅
- Exercise creation form with type selection ✅
- /exercises route with navigation ✅
- Exercise detail page with routing and real data loading ✅

#### ✅ Technical Implementation
- Type-safe Exercise domain model with Rust enums ✅
- WASM-compatible ID generation using js-sys::Date ✅
- Full serde integration across music domain models ✅
- Leptos 0.7 reactive components with correct API patterns ✅
- Proper WASM configuration in .cargo/config.toml ✅

#### ✅ Critical Issues Fixed (Aug 16, 2025)
- **Exercise Detail Loading** - Now loads actual exercise data instead of mock ✅
- **Storage.load_exercise_by_id()** - Implemented for individual exercise lookup ✅
- **Timer Configuration** - Set to 5-minute default for realistic practice sessions ✅
- **Layout Issues** - Clean exercise details above compact timer section ✅
- **WASM Build Issues** - Proper getrandom configuration for browser compatibility ✅

---

### Phase 1: Core Practice Tracking ✅ PHASE 1 COMPLETE
**Goal:** Complete basic timer + session tracking to make app actually useful for practice

#### ✅ Completed Core Features
- **Practice Timer** ✅
  - Count-up timer (shows elapsed time) ✅
  - 5-minute default target time for realistic practice sessions ✅
  - Start/pause/stop functionality ✅
  - Built with leptos-use for reliable intervals ✅
  - Visual feedback when target time is reached ✅

- **Exercise Detail Integration** ✅
  - Fixed exercise detail page to load real exercise data ✅
  - Implemented load_exercise_by_id() function in storage layer ✅
  - Updated component to use correct Exercise model fields ✅
  - Clean layout with exercise info above timer ✅
  - Practice timer integrated into exercise detail workflow ✅

- **Technical Foundation** ✅
  - WASM configuration properly set up (.cargo/config.toml) ✅
  - Leptos 0.7 API patterns implemented correctly ✅
  - Storage layer supports individual exercise lookup ✅
  - Component architecture ready for session logging ✅

#### Recent Commits (Aug 16, 2025)
- `5ea3767`: Refactor ExerciseDetail component and update routing
- `0477bff`: Move WASM rustflags configuration to proper location
- `7349022`: Add PracticeTimer component to ExerciseDetailPage

#### Missing

- Do another review session of the existing code


#### Ready for Phase 1.5: Session Logging
- **Basic Metronome** 🔜
  - Adjustable BPM (30-250)
  - Simple click sound using Web Audio API
  - 4/4 time signature
  - Visual beat indicator (dots)
  
- **Session Logging** 🔜
  - Auto-save practice sessions with: exercise, duration, BPM, timestamp
  - Local storage persistence
  - Simple session history list

### Phase 1.5: Session Logging & Metronome 🔜 NEXT
**Goal:** Add session persistence and basic metronome to complete core practice loop

#### Session Logging Features
- **Practice Session Model**
  - Session ID, exercise ID, start/end timestamps
  - Duration, target time (if set)
  - Optional BPM setting (when metronome added)
  
- **Session Storage**
  - localStorage persistence alongside exercises
  - Automatic session creation on timer start
  - Session completion on timer stop
  - Session list/history view
  
- **Session History UI**
  - Simple chronological list of past sessions
  - Show: exercise name, duration, date
  - Basic filtering (by date, exercise type)

#### Basic Metronome Features
- **Core Metronome**
  - Adjustable BPM (30-250 range)
  - Simple click sound using Web Audio API
  - 4/4 time signature (quarter note clicks)
  - Visual beat indicator
  
- **Metronome Integration**
  - Optional metronome toggle in practice timer
  - BPM setting saved with practice sessions
  - Independent start/stop from main timer

#### Success Criteria
- Sessions automatically saved when timer used
- Session history shows meaningful practice data
- Metronome provides steady, audible beat
- Combined timer + metronome creates complete practice environment

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
