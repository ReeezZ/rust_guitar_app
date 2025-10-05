use crate::music::{notes::Note, scales::ScaleType};

#[derive(Copy, Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Exercise {
  Scale(ScaleExercise),
  Song(Song),
  // TODO: add more types like triads
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
