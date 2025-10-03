use crate::music::{
  notes::Note,
  scales::{Scale, ScaleType},
};

#[derive(Copy, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScaleExercise {
  // TODO: replace root_note and scale_type with Scale, but the Scale type is terrible atm
  pub root_note: Note,
  pub scale_type: ScaleType,
  pub fret_range: (u8, u8), // (min_fret, max_fret)
}

impl std::fmt::Display for ScaleExercise {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} {} (frets {}-{})",
      self.root_note, self.scale_type, self.fret_range.0, self.fret_range.1
    )
  }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Song {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub author: Option<String>,
  pub current_bpm: Option<usize>,
  pub target_bpm: Option<usize>,
}

impl Song {
  pub fn new(id: String, name: String) -> Self {
    Self {
      id,
      name,
      description: None,
      author: None,
      current_bpm: None,
      target_bpm: None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Exercise {
  Scale(ScaleExercise),
  Song(Song),
  // TODO add more types like triads
}

impl std::fmt::Display for Exercise {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Exercise::Scale(scale) => {
        write!(f, "{}", scale)
      }
      Exercise::Song { .. } => write!(f, "Song"),
    }
  }
}

impl Exercise {
  /// Get the scale if this exercise type uses one
  pub fn get_scale(&self) -> Option<Scale> {
    match self {
      Exercise::Scale(scale) => Some(Scale::new(scale.root_note, scale.scale_type)),
      Exercise::Song { .. } => None,
    }
  }

  pub fn create_id(&self) -> String {
    match self {
      Exercise::Scale(scale) => format!(
        "scale_{}_{}_{}-{}",
        scale.root_note, scale.scale_type, scale.fret_range.0, scale.fret_range.1
      ),
      Exercise::Song { .. } => "song".to_string(),
    }
  }

  /// Get the fret range if this exercise type uses one
  pub fn get_fret_range(&self) -> Option<(u8, u8)> {
    match self {
      Exercise::Scale(scale) => Some(scale.fret_range),
      Exercise::Song { .. } => None,
    }
  }

  /// Display name for the exercise type
  pub fn type_name(&self) -> &'static str {
    match self {
      Exercise::Scale { .. } => "Scale",
      Exercise::Song { .. } => "Song",
    }
  }

  pub fn set_root_note(&mut self, new_root: Note) {
    match self {
      Exercise::Scale(scale) => scale.root_note = new_root,
      Exercise::Song { .. } => {}
    }
  }

  // TODO: could return bool to indicate if there was a change made
  pub fn set_scale_type(&mut self, new_scale_type: ScaleType) {
    match self {
      Exercise::Scale(scale) => scale.scale_type = new_scale_type,
      Exercise::Song { .. } => {}
    }
  }

  pub fn set_fret_range(&mut self, new_range: (u8, u8)) {
    match self {
      Exercise::Scale(scale) => scale.fret_range = new_range,
      Exercise::Song { .. } => {}
    }
  }
}
