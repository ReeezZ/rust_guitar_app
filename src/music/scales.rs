use crate::music::notes::Note;

use super::heptatonic_scales::{HeptaScaleImpl, HeptaScaleType};

#[derive(Clone, PartialEq, Copy)]
pub enum ScaleType {
  Hepatonic(HeptaScaleType),
}

impl ScaleType {
  pub fn from_str(s: &str) -> Option<ScaleType> {
    // TODO this is not so nice for maintainability
    match s {
      "Major" => Some(ScaleType::Hepatonic(HeptaScaleType::Major)),
      "Minor" => Some(ScaleType::Hepatonic(HeptaScaleType::Minor)),
      _ => None,
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      ScaleType::Hepatonic(hepta_scale_type) => hepta_scale_type.to_string(),
    }
  }

  pub fn all_scale_types() -> Vec<ScaleType> {
    // TODO this is not so nice for maintainability
    vec![
      ScaleType::Hepatonic(HeptaScaleType::Major),
      ScaleType::Hepatonic(HeptaScaleType::Minor),
    ]
  }
}

#[derive(Clone, PartialEq)]
pub enum Scale {
  Heptatonic(HeptaScaleImpl),
  // TODO add more scale types
  // pentatonic,
  // blues (8 notes)
}

pub trait ScaleCreator {
  fn new(root_note: Note, scale_type: ScaleType) -> Scale;
}

impl ScaleCreator for Scale {
  fn new(root_note: Note, scale_type: ScaleType) -> Scale {
    match scale_type {
      ScaleType::Hepatonic(hepta_scale_type) => {
        let scale = HeptaScaleImpl::new(root_note, hepta_scale_type);
        Scale::Heptatonic(scale)
      }
    }
  }
}

pub trait ScaleTrait {
  fn contains_note(&self, note: Note) -> bool;
  fn root_note(&self) -> Note;
  fn to_string(&self) -> String;
}

impl ScaleTrait for Scale {
  fn contains_note(&self, note: Note) -> bool {
    match self {
      Scale::Heptatonic(scale) => scale.contains_note(note),
    }
  }

  fn root_note(&self) -> Note {
    match self {
      Scale::Heptatonic(scale) => scale.root_note(),
    }
  }

  fn to_string(&self) -> String {
    match self {
      Scale::Heptatonic(scale) => scale.to_string(),
    }
  }
}
