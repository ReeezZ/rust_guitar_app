# 🎸 Rust Guitar Practice App

A guitar learning companion built with **Rust**, **Leptos** (ssr with axum). Features interactive SVG fretboard visualization, practice exercises (working PoC), session tracking (in development).

## Quick Start

## 📋 Development Commands

```bash
cargo leptos watch  # Start dev server
cargo test          # Run all tests
cargo check         # Code quality checks
cargo build         # Production build
cargo clippy        # linting
```

**VS Code Integration:** Use `Ctrl+Shift+P` → "Tasks: Run Task" → pick a task


## Current Features

- **Exercise Management** - Create, edit, and organize practice exercises
  - To be changed, exercises like scales should not need to created manually
  - Maybe something like generate the exercise id of those from their configuration
- **SVG Fretboard System** - Scalable and interactive fretboard display
- **Scale Visualization** - Display scales with different note highlighting  
- **Configurable** - Support for different instruments and fret ranges



## 🎯 What's Next

**Frontend ↔ Backend Integration**

- Fix the exercises page. The storage currently just panics with a todo.
- Get exercises tracking working.
- Change how we select exercises
  - Scales must not be created, they can just be selected
  - First goal: support scales and songs for selection and tracking
  - Get tracking for both working
  - Song
    - Title
    - Maybe features/milestones like
      - with singing
      - with backing track or playing alone
      - freeform or with metronome
    - as opposed to scales, these things have to be created and saved and then have to be loaded into an exercise
  - Display previously practiced songs/exercises
  


- For long term mobile compatibility (sending notifications for practice reminder) a pwa should work with ssr (hopefully) 
  - Should do a more thorough analysis at some point of this
  - Alternatives are csr with tauri
    - but then we'd have to switch back to csr :/
  - or switching entirely to dioxus 


### Future Ideas

#### Fretboard Training Exercises
- Switch english and german note system
- Find notes on the fretboard
- Find interval of note
- Find scales
- Find chord shapes (relative to a given note)
- Show scales (triads, modes)

#### Circle of Fifths
Interactive circle of fifths with drag & drop exercises:
- Fill empty circle of fifths
- Place scales in correct positions
- Increasing difficulty with specific ordering requirements

#### Advanced Features
- **Tuner** - Audio input for tuning assistance
- **Metronome** - Configurable drum beats and timing
- **Ear training** - Audio-based exercises
- **Chord book** - Progress tracking for chord knowledge
- **Chords in a key** - Key-based chord relationships
- **MIDI/Keyboard input** - Use keys 1-7 to play scale degrees


## 🛠️ Setup Options

### Option 1: VS Code Dev Container (Recommended)
1. Install VS Code + Docker + Dev Containers extension
2. Clone repo and open in VS Code
3. `Ctrl+Shift+P` → "Dev Containers: Rebuild and Reopen in Container"
4. Run `cargo leptos watch`

### Option 2: Manual Setup

**Prerequisites**

- Node
- Rust + Cargo

```bash
# Install dependencies

rustup component add rustfmt rust-src clippy
rustup target add wasm32-unknown-unknown
# https://book.leptos.dev/ssr/21_cargo_leptos.html
cargo install --locked cargo-leptos
# https://book.leptos.dev/getting_started/leptos_dx.html#4-set-up-leptosfmt-optional
cargo install leptosfmt

# run
cargo leptos watch
```

## 📚 References & Inspiration

**Guitar Learning Tools:**
- [fretmap.app](https://fretmap.app/) - Modern fretboard visualization
- [Fretonomy](https://www.fretonomy.com/) - Guitar learning platform

**Technical References:**
- [Leptos Book](https://leptos.dev/) - Reactive web framework
- [Axum Documentation](https://docs.rs/axum/) - Web application framework
- [SVG Guitar Tutorials](https://www.youtube.com/watch?v=C6VLedW5Dwk) - Fretboard visualization
- [guitar scientist](https://www.editor.guitarscientist.com/new)

**Tech stack ideas**
- [tailwindcss-animated](https://github.com/new-data-services/tailwindcss-animated) - for enhanced animations

## License

**GNU General Public License v3.0** - see [LICENSE](./LICENSE)
