use crate::music::{
  notes::Note,
  scales::{Scale, ScaleTrait, ScaleType},
};

/// Generate a simple unique ID using browser timestamp
fn generate_id() -> String {
  // Use JavaScript's Date.now() which works in WASM
  let timestamp = js_sys::Date::now() as u64;
  format!("ex_{timestamp}")
}

/// Exercise types with their specific configuration
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExerciseType {
  /// Scale practice with specific scale and fret range
  Scale {
    root_note: Note,
    scale_type: ScaleType,
    fret_range: (u8, u8), // (min_fret, max_fret)
  },
  /// Triad practice with specific scale and fret range
  Triad {
    root_note: Note,
    scale_type: ScaleType,
    fret_range: (u8, u8),
  },
  /// Technique practice (no key or fret range needed)
  Technique,
  /// Song practice (no key or fret range needed)
  Song,
}

impl ExerciseType {
  /// Get the scale if this exercise type uses one
  pub fn get_scale(&self) -> Option<Scale> {
    match self {
      ExerciseType::Scale {
        root_note,
        scale_type,
        ..
      } => Some(Scale::new(*root_note, *scale_type)),
      ExerciseType::Triad {
        root_note,
        scale_type,
        ..
      } => Some(Scale::new(*root_note, *scale_type)),
      ExerciseType::Technique | ExerciseType::Song => None,
    }
  }

  /// Get the fret range if this exercise type uses one
  pub fn get_fret_range(&self) -> Option<(u8, u8)> {
    match self {
      ExerciseType::Scale { fret_range, .. } => Some(*fret_range),
      ExerciseType::Triad { fret_range, .. } => Some(*fret_range),
      ExerciseType::Technique | ExerciseType::Song => None,
    }
  }

  /// Display name for the exercise type
  pub fn type_name(&self) -> &'static str {
    match self {
      ExerciseType::Scale { .. } => "Scale",
      ExerciseType::Triad { .. } => "Triad",
      ExerciseType::Technique => "Technique",
      ExerciseType::Song => "Song",
    }
  }
}

impl std::fmt::Display for ExerciseType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExerciseType::Scale {
        root_note,
        scale_type,
        fret_range,
      } => {
        write!(
          f,
          "{} {} (frets {}-{})",
          root_note,
          scale_type.to_string(),
          fret_range.0,
          fret_range.1
        )
      }
      ExerciseType::Triad {
        root_note,
        scale_type,
        fret_range,
      } => {
        write!(
          f,
          "{} {} Triad (frets {}-{})",
          root_note,
          scale_type.to_string(),
          fret_range.0,
          fret_range.1
        )
      }
      ExerciseType::Technique => write!(f, "Technique"),
      ExerciseType::Song => write!(f, "Song"),
    }
  }
}

/// A practice exercise
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Exercise {
  pub id: String,
  pub name: String,
  pub exercise_type: ExerciseType,
  pub description: Option<String>,
}

impl Exercise {
  pub fn new(name: String, exercise_type: ExerciseType) -> Self {
    Self {
      id: generate_id(),
      name,
      exercise_type,
      description: None,
    }
  }

  pub fn with_description(mut self, description: String) -> Self {
    self.description = Some(description);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scale_exercise_creation() {
    let exercise = Exercise::new(
      "C Major Scale Practice".to_string(),
      ExerciseType::Scale {
        root_note: Note::C,
        scale_type: ScaleType::Hepatonic(crate::music::heptatonic_scales::HeptaScaleType::Major),
        fret_range: (0, 5),
      },
    );

    assert_eq!(exercise.name, "C Major Scale Practice");
    assert_eq!(exercise.exercise_type.type_name(), "Scale");
    assert_eq!(exercise.exercise_type.get_fret_range(), Some((0, 5)));
    assert!(exercise.exercise_type.get_scale().is_some());
  }

  #[test]
  fn test_technique_exercise_creation() {
    let exercise = Exercise::new("Alternate Picking".to_string(), ExerciseType::Technique);

    assert_eq!(exercise.exercise_type.type_name(), "Technique");
    assert_eq!(exercise.exercise_type.get_fret_range(), None);
    assert!(exercise.exercise_type.get_scale().is_none());
  }
}
