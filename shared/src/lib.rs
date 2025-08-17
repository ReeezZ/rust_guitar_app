pub mod music;
pub mod models;

// Re-export commonly used types
pub use models::exercise::{Exercise, ExerciseType};
pub use music::notes::Note;
pub use music::scales::{Scale, ScaleType, ScaleTrait};

/// Generate a simple unique ID using timestamp
pub fn generate_id() -> String {
  #[cfg(feature = "wasm")]
  {
    // Use JavaScript's Date.now() for WASM compatibility
    let timestamp = js_sys::Date::now() as u64;
    format!("ex_{timestamp}")
  }
  
  #[cfg(not(feature = "wasm"))]
  {
    // Use system timestamp for backend
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis() as u64;
    format!("ex_{timestamp}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_id_generation() {
    let id1 = generate_id();
    let id2 = generate_id();
    assert_ne!(id1, id2);
    assert!(!id1.is_empty());
  }
}
