# Guitar Practice Tracker - Vision & Roadmap

## Vision Statement
Build a **personal guitar practice tracker** that makes it easy to log practice sessions, track progress over time, and stay motivated through simple but effective progress visualization. The app should be simple to use daily but provide rich long-term insights.

**Core Philosophy:** Start simple, grow iteratively, focus on actual usage over feature complexity.

---

## User Story
*As a guitar player, I want to easily select what I'm practicing, start a timer with metronome, and automatically track my progress so that I can stay motivated and see my improvement over time.*

## Core Workflow
1. **Select Exercise** - Choose what to practice (scales, triads, technique, songs)
2. **Configure Practice** - Set key, position, target time (optional)
3. **Start Session** - Timer counts up, metronome provides rhythm
4. **Practice** - Focus on playing, minimal app interaction needed
5. **Finish & Save** - Session automatically logged with all details
6. **Review Progress** - View practice history and trends over time

---

## Development Phases

### Phase 1: Foundation - Basic Practice Logging
**Goal:** Get basic timer + exercise tracking working with local storage

#### Core Features
- **Exercise Management**
  - Simple exercise types: Scales, Triads, Technique, Songs
  - Each exercise has: name, type, optional key, optional position (fret range)
  - CRUD operations for exercises
  
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

#### Data Models
```rust
enum ExerciseType {
    Scale { key: Option<Note>, position: Option<FretRange> },
    Triad { key: Option<Note>, position: Option<FretRange> },
    Technique { description: String },
    Song { key: Option<Note>, name: String },
}

struct Exercise {
    id: Uuid,
    name: String,
    exercise_type: ExerciseType,
    target_duration: Option<Duration>,
}

struct PracticeSession {
    id: Uuid,
    exercise_id: Uuid,
    start_time: DateTime<Utc>,
    duration: Duration,
    metronome_bpm: Option<u32>,
}
```

#### UI Components
- Exercise list/selector
- Practice screen (timer + metronome controls)
- Basic session history
- Simple exercise creation form

---

### Phase 2: Enhanced UX & Fretboard Integration
**Goal:** Improve usability and integrate with existing fretboard visualization

#### Enhanced Features
- **Improved Metronome**
  - Multiple sound options
  - Accent on beat 1
  - Mid-session BPM adjustment with tracking
  
- **Fretboard Integration**
  - Show relevant fretboard visualization during practice
  - Solve layout challenges (compact fretboard view?)
  - Scale/triad visualization for relevant exercises
  
- **Better Exercise Management**
  - Exercise templates/presets
  - Quick exercise creation
  - Exercise categories and filtering
  
- **Improved Timer**
  - Multiple timing modes
  - Break reminders
  - Session quality rating (1-5 stars)

#### UX Improvements
- Better responsive design
- Keyboard shortcuts
- Practice flow optimizations
- Visual polish

---

### Phase 3: Backend & Analytics
**Goal:** Add backend persistence and basic progress analytics

#### Backend Features
- **Data Persistence**
  - Backend API for practice data
  - User accounts (simple, no social features)
  - Data migration from local storage
  - Backup/restore capabilities
  
- **Basic Analytics**
  - Practice consistency tracking
  - Total time per exercise type
  - Progress over time (duration, frequency)
  - Simple charts and graphs

#### Architecture Notes
- Design local storage abstraction that can be swapped for backend
- Consider SQLite for backend (simple, reliable)
- API should support offline-first approach

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
  
- **Audio Recording** - Record practice sessions for review
- **Click Tracks** - Export metronome tracks for external use

### Social & Sharing Features
- **Practice Sharing** - Share routines with others
- **Progress Sharing** - Social media integration
- **Teacher Dashboard** - Track student progress (way future)

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
4. **Offline-First** - Work without internet, sync when available

### Development Principles
- **Iterative Development** - Each phase should be fully functional
- **User-Driven Features** - Only add complexity when proven needed
- **Performance Focus** - Smooth timer/metronome performance critical
- **Mobile-Ready** - Design with mobile usage in mind from start

---

## Success Criteria

### Phase 1 Success
- Can create exercises and log practice sessions
- Timer and metronome work reliably
- Data persists across browser sessions
- Basic history view shows past practice

### Phase 2 Success  
- Fretboard integration enhances practice experience
- Improved metronome feels professional
- Overall UX is smooth and pleasant

### Phase 3 Success
- Backend migration seamless
- Basic analytics provide motivational insights
- Data is safe and accessible across devices

### Phase 4 Success
- Routine management solves real workflow problems
- App becomes primary practice tracking tool
- UX clearly superior to existing solutions

---

## Migration Plan (GitHub Issues/Milestones)

When moving to GitHub, this document will inform:

### Milestones
- **Phase 1: Foundation** - All core features working locally
- **Phase 2: Enhanced UX** - Fretboard integration + UX polish  
- **Phase 3: Backend & Analytics** - Cloud storage + insights
- **Phase 4: Routines** - Practice workflow management

### Issue Creation Strategy
- Break each phase into 2-3 week chunks
- Create detailed user stories for each feature
- Use labels: `phase-1`, `phase-2`, `enhancement`, `bug`, `future`
- Template issues for common patterns (new exercise types, UI components)

### Future Idea Management
- Use GitHub Discussions for rough concepts
- `future` label for ideas not yet ready for implementation
- Regular review/promotion of future ideas to active phases

---

*This vision captures current thinking as of August 2025. Expected to evolve as development progresses and usage patterns emerge.*
