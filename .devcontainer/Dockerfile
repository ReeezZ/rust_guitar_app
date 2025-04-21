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

# Install Rust WASM target
RUN rustup target add wasm32-unknown-unknown

# Install Trunk (WASM bundler) and Leptosfmt (for formatting Leptos macros)
RUN cargo install trunk leptosfmt

# Install Tailwind CSS IntelliSense (optional, for VS Code support)
RUN npm install -g tailwindcss

# Set the working directory
WORKDIR /workspaces/leptos_stuff