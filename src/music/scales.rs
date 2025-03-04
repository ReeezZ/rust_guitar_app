use std::{borrow::Cow, str::FromStr};

use crate::music::notes::Note;

use super::heptatonic_scales::{HeptaScaleImpl, HeptaScaleType};

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum ScaleType {
  Hepatonic(HeptaScaleType),
  Chromatic,
}

impl ScaleType {
  pub fn from_str(s: &str) -> Option<ScaleType> {
    // TODO this is not so nice for maintainability
    match s {
      "Major" => Some(ScaleType::Hepatonic(HeptaScaleType::Major)),
      "Minor" => Some(ScaleType::Hepatonic(HeptaScaleType::Minor)),
      "Chromatic" => Some(ScaleType::Chromatic),
      _ => None,
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      ScaleType::Hepatonic(hepta_scale_type) => hepta_scale_type.to_string(),
      ScaleType::Chromatic => "Chromatic".to_string(),
    }
  }

  pub fn all_scale_types() -> Vec<ScaleType> {
    // TODO this is not so nice for maintainability

    HeptaScaleType::all_scale_types()
      .iter()
      .map(|&hepta_scale_type| ScaleType::Hepatonic(hepta_scale_type))
      .chain(Some(ScaleType::Chromatic).into_iter())
      .collect()
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
  fn new(root_note: Note, scale_type: ScaleType) -> Scale;
}

// TODO into / from methods from std might help avoiding the hard coded "Chromatic.to_string()" call
impl Into<Cow<'static, str>> for Scale {
  fn into(self) -> Cow<'static, str> {
    match self {
      Scale::Heptatonic(scale) => Cow::Owned(scale.to_string()),
      Scale::Chromatic => Cow::Borrowed("Chromatic"),
    }
  }
}

impl ToString for Scale {
  fn to_string(&self) -> String {
    Cow::<'static, str>::from(*self).to_string()
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
