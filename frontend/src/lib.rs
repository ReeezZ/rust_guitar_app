pub mod app;
pub mod audio;
pub mod components;
mod fretboard;
pub mod models;
pub(crate) mod pages;

// Re-export shared types for convenience
pub use shared::{models::*, music::*, *};
