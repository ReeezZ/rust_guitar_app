# Use the official Rust devcontainer image as the base
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    nodejs \
    npm \
    && apt-get clean

# Update Rust to latest stable version
RUN rustup update stable && rustup default stable

# Install rustfmt for code formatting and clippy for linting
RUN rustup component add rustfmt rust-src clippy

# Install Rust WASM target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler), Leptosfmt (for formatting Leptos macros), and wasm-bindgen-cli
RUN cargo install cargo-leptos trunk leptosfmt wasm-bindgen-cli

# Install Gemini CLI for AI assistance during development
# NOTE: This is a dev container only - production builds use separate processes
RUN npm install -g @google/gemini-cli

# Fix cargo registry permissions for the vscode user
# Clear any existing cache first, then set proper ownership
RUN rm -rf /usr/local/cargo/registry/cache/* && \
    chown -R vscode:vscode /usr/local/cargo

# Set the working directory
WORKDIR /workspaces/leptos_stuff