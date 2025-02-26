use std::ops::Index;

use crate::music::notes::Note;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleDegree {
  First,
  Second,
  Third,
  Fourth,
  Fifth,
  Sixth,
  Seventh,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleType {
  Major,
  Minor,
  MajorPentatonic,
  MinorPentatonic,
  MajorBlues,
  MinorBlues,
}

// TODO add more scales
// TODO consider non septa scales
#[derive(Debug, Clone, PartialEq)]
pub struct Scale {
  root_note: Note,
  scale_type: ScaleType,
  notes: Vec<Note>,
}

impl Scale {
  pub fn new(root_note: Note, scale_type: ScaleType) -> Self {
    let notes = Self::get_notes(root_note, scale_type);
    Scale {
      root_note,
      scale_type,
      notes,
    }
  }

  fn get_notes(root_note: Note, scale_type: ScaleType) -> Vec<Note> {
    let notes = match scale_type {
      // TODO open closed principle violoation
      // maybe i should use traits?
      ScaleType::Major => Self::generate_major_scale(root_note),
      ScaleType::Minor => Self::generate_minor_scale(root_note),
      _ => panic!("Scale type not implemented"),
    };

    notes
  }

  fn generate_major_scale(root_note: Note) -> Vec<Note> {
    let intervals = [
      // TODO using intervals might be cleaner
      0,  // Unison
      2,  // Major Second
      4,  // Major Third
      5,  // Perfect Fourth
      7,  // Perfect Fifth
      9,  // Major Sixth
      11, // Major Seventh
    ];
    Self::generate_scale(root_note, &intervals)
  }

  fn generate_minor_scale(root_note: Note) -> Vec<Note> {
    let intervals = [
      0,  // Unison
      2,  // Major Second
      3,  // Minor Third
      5,  // Perfect Fourth
      7,  // Perfect Fifth
      8,  // Minor Sixth
      10, // Minor Seventh
    ];
    Self::generate_scale(root_note, &intervals)
  }

  fn generate_scale(root_note: Note, intervals: &[usize]) -> Vec<Note> {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == root_note).unwrap();
    intervals
      .iter()
      .map(move |&interval| {
        let index = (start_index + interval) % all_notes.len();
        all_notes[index]
      })
      .collect()
  }

  fn get_note_by_degree(&self, degree: ScaleDegree) -> &Note {
    let index = match degree {
      ScaleDegree::First => 0,
      ScaleDegree::Second => 1,
      ScaleDegree::Third => 2,
      ScaleDegree::Fourth => 3,
      ScaleDegree::Fifth => 4,
      ScaleDegree::Sixth => 5,
      ScaleDegree::Seventh => 6,
    };
    &self.notes[index]
  }

  pub fn contains_note(&self, note: Note) -> bool {
    self.notes.contains(&note)
  }
}

impl Index<ScaleDegree> for Scale {
  type Output = Note;

  fn index(&self, index: ScaleDegree) -> &Self::Output {
    &self.get_note_by_degree(index)
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  // TODO add tests

  #[test]
  fn test_c_major_scale() {
    let c_major = Scale::new(Note::C, ScaleType::Major);
    assert_eq!(c_major[ScaleDegree::First], Note::C);
    assert_eq!(c_major[ScaleDegree::Second], Note::D);
    assert_eq!(c_major[ScaleDegree::Third], Note::E);
    assert_eq!(c_major[ScaleDegree::Fourth], Note::F);
    assert_eq!(c_major[ScaleDegree::Fifth], Note::G);
    assert_eq!(c_major[ScaleDegree::Sixth], Note::A);
    assert_eq!(c_major[ScaleDegree::Seventh], Note::H);
  }

  #[test]
  fn test_a_minor_scale() {
    let a_minor = Scale::new(Note::A, ScaleType::Minor);
    assert_eq!(a_minor[ScaleDegree::First], Note::A);
    assert_eq!(a_minor[ScaleDegree::Third], Note::C);
  }

  #[test]
  fn test_a_major_scale() {
    let a_minor = Scale::new(Note::A, ScaleType::Major);
    assert_eq!(a_minor[ScaleDegree::First], Note::A);
    assert_eq!(a_minor[ScaleDegree::Second], Note::H);
    assert_eq!(a_minor[ScaleDegree::Third], Note::CisOrDes);
    assert_eq!(a_minor[ScaleDegree::Fourth], Note::D);
    assert_eq!(a_minor[ScaleDegree::Fifth], Note::E);
    assert_eq!(a_minor[ScaleDegree::Sixth], Note::FisOrGes);
    assert_eq!(a_minor[ScaleDegree::Seventh], Note::GisOrAs);
  }
}
