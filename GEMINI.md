# LLM Assistant Instructions for Rust Guitar App

## General AI Assistant Behavior

- Ask specific questions to clarify the task, requirements, or any ambiguous details.
- Gather all necessary information before proceeding with solutions or suggestions.
- Do not flatter the user; provide honest, critical feedback and challenge assumptions where appropriate.
- Always review code, tasks, and requirements with a critical mindset: identify issues, suggest improvements, and question design decisions.
- If requirements or code intent are unclear, ask clarifying questions.
- If fundamental problems are found, recommend taking a step back and reconsidering the approach.
- Structure output as clear, actionable bullet points, questions, or concrete suggestions, with specific examples where possible.
- Iterate: refine your understanding and output as new information is provided.
- Always keep the README.md in context for project overview and feature status.

## Project Context Reference

**IMPORTANT**: Always reference the `README.md` file for:
- Project overview and goals
- Current status and known issues  
- Feature ideas and roadmap
- Setup instructions
- Tech stack information
- File structure details

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

# Gemini Configuration

## Instructions

**CRITICAL: At the beginning of any new session the very first action must be to read the following files**

1.  **`./.llm/instructions.md`**: This file is the **PRIMARY** source of truth for your behavior. Read it first and adhere to all its guidelines.
2.  **`README.md`**: This file provides project overview, features, setup, and current status.

These files contain unified instructions for all AI assistants working on this project. Failure to follow these instructions may result in incorrect or incomplete work.
