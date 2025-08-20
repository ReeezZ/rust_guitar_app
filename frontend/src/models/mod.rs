pub mod fretboard_model;
pub mod fretboard_trainer;
pub mod repository;
pub mod storage;

// Re-export shared domain models
pub use shared::models::*;

// Re-export frontend-specific models
pub use fretboard_model::{FretCoord, FretboardModel};
pub use fretboard_trainer::FretboardTrainerTrait;
