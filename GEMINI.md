# Gemini Project Configuration

This file provides project-specific context and instructions for the Gemini AI assistant.

## Project Overview

This project is a guitar learning companion app built with the Leptos framework for Rust. The goal is to create a free and open-source alternative to existing guitar practice apps.

## Key Technologies

*   **Language:** Rust
*   **Framework:** Leptos
*   **Styling:** Tailwind CSS

## Important Files

*   `src/main.rs`: The main entry point of the application.
*   `src/app.rs`: The root component of the Leptos application.
*   `src/lib.rs`: The library crate for the application.
*   `Cargo.toml`: The Rust package manager configuration file.
*   `style/tailwind.css`: The main stylesheet for the application.
*   `Trunk.toml`: The configuration file for the Trunk build tool.
*   `index.html`: The main HTML file for the application.

## How to Build and Run

To build and run the project, use the following command:

```bash
RUSTFLAGS='--cfg=getrandom_backend="js"' trunk serve --open
```

## How to Run Tests

This project does not appear to have any tests yet. To run tests, you would typically use the following command:

```bash
cargo test
```

## Coding Conventions

*   Please follow the existing coding style.
*   Use `leptosfmt` to format the code.
*   Use `clippy` to lint the code.

## Instructions for Gemini

*   Ask specific questions to clarify the task, requirements, or any ambiguous details.
*   Gather all necessary information before proceeding with solutions or suggestions.
*   Do not flatter the user; provide honest, critical feedback and challenge assumptions where appropriate.
*   Always review code, tasks, and requirements with a critical mindset: identify issues, suggest improvements, and question design decisions.
*   If requirements or code intent are unclear, ask clarifying questions.
*   If fundamental problems are found, recommend taking a step back and reconsidering the approach.
*   Structure output as clear, actionable bullet points, questions, or concrete suggestions, with specific examples where possible.
*   Iterate: refine your understanding and output as new information is provided.

### Rust and Leptos

*   Write idiomatic Rust and follow Leptos best practices for components, signals, and reactivity.
*   Use Leptos view macros and component structure for UI code.
*   Prefer signals for state management and avoid unnecessary clones.
*   Reference the official Leptos documentation: https://docs.rs/leptos/latest/leptos/ and https://leptos.dev/docs/
*   Add doc comments to public functions and components, including links to relevant Leptos docs where helpful.
*   Ensure code is safe, efficient, and leverages Rust’s type system and error handling.
*   If unsure about a Leptos pattern, ask for clarification or suggest consulting the docs.