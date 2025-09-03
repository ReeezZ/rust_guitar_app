FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN rustup update stable && rustup default stable

RUN rustup component add rustfmt rust-src clippy

# Required for leptos
RUN rustup target add wasm32-unknown-unknown

# Using the --locked flag as shown in the docs
# https://book.leptos.dev/ssr/21_cargo_leptos.html#introducing-cargo-leptos
RUN cargo install --locked cargo-leptos

# https://book.leptos.dev/getting_started/leptos_dx.html#4-set-up-leptosfmt-optional
RUN cargo install leptosfmt

# I dont know why yet, but when i start the container i am still logged in as vscode
# Thats why I require this chown command for leptos watch to work
RUN chown -R vscode:vscode /usr/local/cargo

WORKDIR /workspaces/rust_guitar_app
