use crate::music::notes::Note;
use std::fmt;
use std::str::FromStr;

use super::heptatonic_scales::{HeptaScaleImpl, HeptaScaleType};

#[derive(Clone, PartialEq, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum ScaleType {
  Hepatonic(HeptaScaleType),
  Chromatic,
}

impl ScaleType {
  pub fn all_scale_types() -> Vec<ScaleType> {
    // TODO this is not so nice for maintainability

    HeptaScaleType::all_scale_types()
      .iter()
      .map(|&hepta_scale_type| ScaleType::Hepatonic(hepta_scale_type))
      .chain(Some(ScaleType::Chromatic))
      .collect()
  }
}

impl FromStr for ScaleType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Major" => Ok(ScaleType::Hepatonic(HeptaScaleType::Major)),
      "Minor" => Ok(ScaleType::Hepatonic(HeptaScaleType::Minor)),
      "Chromatic" => Ok(ScaleType::Chromatic),
      _ => Err(format!("Unknown scale type: {s}")),
    }
  }
}

impl fmt::Display for ScaleType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ScaleType::Hepatonic(hepta_scale_type) => write!(f, "{hepta_scale_type}"),
      ScaleType::Chromatic => write!(f, "Chromatic"),
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Scale {
  Heptatonic(HeptaScaleImpl),
  Chromatic,
  // TODO add more scale types
  // pentatonic,
  // blues (8 notes)
}

pub trait ScaleTrait: ToString {
  fn contains_note(&self, note: Note) -> bool;
  fn root_note(&self) -> Option<Note>;
  fn new(root_note: Note, scale_type: ScaleType) -> Self;
}

impl fmt::Display for Scale {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Scale::Heptatonic(scale) => write!(f, "{scale}"),
      Scale::Chromatic => write!(f, "Chromatic"),
    }
  }
}

impl ScaleTrait for Scale {
  fn contains_note(&self, note: Note) -> bool {
    match self {
      Scale::Heptatonic(scale) => scale.contains_note(note),
      Scale::Chromatic => true,
    }
  }

  fn root_note(&self) -> Option<Note> {
    match self {
      Scale::Heptatonic(scale) => Some(scale.root_note()),
      // chromatic does not really have a root note, so we just return C
      Scale::Chromatic => None,
    }
  }

  fn new(root_note: Note, scale_type: ScaleType) -> Scale {
    match scale_type {
      ScaleType::Hepatonic(hepta_scale_type) => {
        let scale = HeptaScaleImpl::new(root_note, hepta_scale_type);
        Scale::Heptatonic(scale)
      }
      ScaleType::Chromatic => Scale::Chromatic,
    }
  }
}
