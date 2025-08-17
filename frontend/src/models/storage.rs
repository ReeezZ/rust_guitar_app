use shared::models::exercise::Exercise;
use leptos::logging;
use serde_json;
use web_sys::{window, Storage};

/// Simple exercise storage - just save and load exercises
pub fn save_exercise(exercise: &Exercise) -> Result<(), String> {
  let storage = get_storage().ok_or("Storage not available")?;
  let key = format!("exercise_{id}", id = exercise.id);
  let json =
    serde_json::to_string(exercise).map_err(|e| format!("Failed to serialize exercise: {e}"))?;
  storage
    .set_item(&key, &json)
    .map_err(|_| "Failed to save exercise".to_string())?;
  Ok(())
}

pub fn load_exercises() -> Vec<Exercise> {
  let mut exercises = Vec::new();
  let storage = match get_storage() {
    Some(s) => s,
    None => {
      logging::error!("Local storage is not available");
      return exercises;
    }
  };
  let len = storage.length().unwrap_or(0);
  logging::log!("Loading exercises from local storage: {}", len);
  for i in 0..len {
    if let Some(key) = storage.key(i).ok().flatten() {
      if key.starts_with("exercise_") {
        if let Ok(Some(json)) = storage.get_item(&key) {
          if let Ok(ex) = serde_json::from_str::<Exercise>(&json) {
            exercises.push(ex);
          }
        }
      }
    }
  }
  exercises
}

/// Delete an exercise from localStorage
pub fn delete_exercise(id: &str) -> Result<(), String> {
  let storage = get_storage().ok_or("Storage not available")?;
  let key = format!("exercise_{id}");
  storage
    .remove_item(&key)
    .map_err(|_| "Failed to delete exercise".to_string())?;
  Ok(())
}

/// Load a specific exercise by ID from localStorage
pub fn load_exercise_by_id(id: &str) -> Option<Exercise> {
  let storage = get_storage()?;
  let key = format!("exercise_{id}");
  if let Ok(Some(json)) = storage.get_item(&key) {
    serde_json::from_str::<Exercise>(&json).ok()
  } else {
    None
  }
}

/// Update an existing exercise in localStorage
pub fn update_exercise(exercise: &Exercise) -> Result<(), String> {
  let storage = get_storage().ok_or("Storage not available")?;
  let key = format!("exercise_{id}", id = exercise.id);

  // Check if exercise exists
  if storage.get_item(&key).ok().flatten().is_none() {
    return Err("Exercise not found".to_string());
  }

  let json =
    serde_json::to_string(exercise).map_err(|e| format!("Failed to serialize exercise: {e}"))?;

  storage
    .set_item(&key, &json)
    .map_err(|_| "Failed to update exercise".to_string())?;

  Ok(())
}

/// Check if an exercise name already exists (for validation)
pub fn exercise_name_exists(name: &str, exclude_id: Option<&str>) -> bool {
  let exercises = load_exercises();
  exercises
    .iter()
    .any(|ex| ex.name == name && exclude_id.is_none_or(|id| ex.id != id))
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
    let exercise = Exercise::new(
      "Test".to_string(),
      crate::models::exercise::ExerciseType::Technique,
    );
    let _ = save_exercise(&exercise);
    // Would test loading here in browser environment
  }
}
