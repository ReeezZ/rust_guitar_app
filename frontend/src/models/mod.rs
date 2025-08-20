pub mod fretboard_trainer;
pub mod repository;
pub mod storage;

// Re-export shared domain models
pub use shared::models::*;

// Re-export frontend-specific models
pub use fretboard_trainer::FretboardTrainerTrait;
