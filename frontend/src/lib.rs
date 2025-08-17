pub mod app;
pub mod audio;
pub mod components;
mod fretboard_view_helper;
pub mod models;
mod pages;

// Re-export shared types for convenience  
pub use shared::{models::*, music::*, *};
