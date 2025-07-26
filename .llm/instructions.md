# LLM Assistant Instructions for Rust Guitar App

## General AI Assistant Behavior

- Ask specific questions to clarify the task, requirements, or any ambiguous details.
- Gather all necessary information using available tools like fetch (web requests), getting folder structure, reading files or whatever is available. Do that where necessary.
- Do not flatter the user; provide honest, critical feedback and challenge assumptions where appropriate.
- Always review code, tasks, and requirements with a critical mindset: identify issues, suggest improvements, and question design decisions.
- If requirements, the code or the intent is unclear, ask clarifying questions. Do not assume.
- If fundamental problems are found, recommend taking a step back and reconsidering the approach.
- Structure output as clear, actionable bullet points, questions, or concrete suggestions, with specific examples where possible.
- Outline your next steps and ideas on a problem before trying to solve the problem.
  - Create a plan of what to do before doing it

## Project Context Reference

**IMPORTANT**: Always reference the `README.md` file for a project overview and goals

## Technology-Specific Guidance

### Rust and Leptos

- Write idiomatic Rust and follow Leptos best practices for components, signals, and reactivity.
- Use Leptos view macros and component structure for UI code.
- Prefer signals for state management and avoid unnecessary clones.
- Reference the official Leptos documentation: https://docs.rs/leptos/latest/leptos/ and https://leptos.dev/docs/
- Add doc comments to public functions and components, including links to relevant Leptos docs where helpful.
- Ensure code is safe, efficient, and leverages Rust's type system and error handling.
- If unsure about a Leptos pattern, ask for clarification or suggest consulting the docs.

### Development Commands

**Build and Run:**
```bash
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open
```

**Run Tests:**
```bash
cargo test
```

**Code Formatting:**
```bash
cargo fmt
```

### Coding Conventions

- Follow the existing coding style in the project
- Use `cargo fmt` to format code consistently
- Write clear, descriptive doc comments for public APIs
- Leverage Rust's type system for safety and clarity
- Follow Leptos patterns for component composition and state management

### Important Files to Consider

- `src/main.rs`: Application entry point
- `src/app.rs`: Root Leptos component
- `src/lib.rs`: Library crate definitions
- `Cargo.toml`: Dependencies and project configuration
- `public/styles/tailwind.css`: Styling definitions
- `Trunk.toml`: Build tool configuration
- `index.html`: HTML template

## Music Domain Knowledge

This is a guitar learning app, so consider:
- Musical concepts (notes, scales, intervals, chords)
- Guitar-specific terminology (frets, strings, fretboard)
- Learning progression and educational UX patterns
- Audio/visual feedback for musical interaction

