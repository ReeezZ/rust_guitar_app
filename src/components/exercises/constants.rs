// Exercise type constants to avoid string literals everywhere
pub const TECHNIQUE_TYPE: &str = "Technique";
pub const SCALE_TYPE: &str = "Scale";
pub const TRIAD_TYPE: &str = "Triad";
pub const SONG_TYPE: &str = "Song";

// Helper function to check if exercise type has specific settings
pub fn has_specific_settings(exercise_type: &str) -> bool {
    matches!(exercise_type, SCALE_TYPE | TRIAD_TYPE)
}
