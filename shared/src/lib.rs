pub mod models;
pub mod music;
pub mod utils;

// Re-export commonly used types
pub use models::exercise::{Exercise, ExerciseType};
pub use music::notes::Note;
pub use music::scales::{Scale, ScaleExt, ScaleType};
pub use utils::{generate_id, IdGenerator};
