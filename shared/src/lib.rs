pub mod music;
pub mod models;
pub mod utils;

// Re-export commonly used types
pub use models::exercise::{Exercise, ExerciseType};
pub use music::notes::Note;
pub use music::scales::{Scale, ScaleType, ScaleTrait};
pub use utils::{generate_id, IdGenerator};
