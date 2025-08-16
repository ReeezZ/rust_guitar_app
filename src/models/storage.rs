use crate::models::exercise::Exercise;
use web_sys::{Storage, window};

/// Simple exercise storage - just save and load exercises
pub fn save_exercise(exercise: &Exercise) {
    let storage = get_storage().expect("Storage not available");
    let key = format!("exercise_{}", exercise.id);
    
    // Simple JSON - just the basics we need for now
    let json = format!(
        r#"{{"id":"{}","name":"{}","type":"{}"}}"#,
        exercise.id,
        exercise.name.replace('"', r#"\""#),
        exercise.exercise_type.type_name()
    );
    
    storage.set_item(&key, &json).expect("Failed to save exercise");
}

pub fn load_exercises() -> Vec<Exercise> {
    // For now, return empty - we'll implement when we need it
    vec![]
}

pub fn delete_exercise(exercise_id: &str) {
    let storage = get_storage().expect("Storage not available");
    let key = format!("exercise_{}", exercise_id);
    storage.remove_item(&key).expect("Failed to delete exercise");
}

fn get_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Needs browser environment
    fn test_exercise_storage() {
        let exercise = Exercise::new("Test".to_string(), crate::models::exercise::ExerciseType::Technique);
        save_exercise(&exercise);
        // Would test loading here in browser environment
    }
}
