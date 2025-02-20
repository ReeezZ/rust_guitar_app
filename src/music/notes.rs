// TODO split into multiple files

use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Note {
  C,
  CisOrDes,
  D,
  DisOrEs,
  E,
  F,
  FisOrGes,
  G,
  GisOrAs,
  A,
  AisOrB,
  H,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interval {
  Unison,
  MinorSecond,
  MajorSecond,
  MinorThird,
  MajorThird,
  PerfectFourth,
  Tritone,
  PerfectFifth,
  MinorSixth,
  MajorSixth,
  MinorSeventh,
  MajorSeventh,
  Octave,
}

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
// TODO No copy, contains a Vec, is this lightweight enough for a scale type?
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
}

impl Index<ScaleDegree> for Scale {
  type Output = Note;

  fn index(&self, index: ScaleDegree) -> &Self::Output {
    &self.get_note_by_degree(index)
  }
}

impl Note {
  fn all_notes() -> &'static [Note; 12] {
    static ALL_NOTES: [Note; 12] = [
      Note::C,
      Note::CisOrDes,
      Note::D,
      Note::DisOrEs,
      Note::E,
      Note::F,
      Note::FisOrGes,
      Note::G,
      Note::GisOrAs,
      Note::A,
      Note::AisOrB,
      Note::H,
    ];
    &ALL_NOTES
  }

  pub fn get_note_by_interval(&self, interval: Interval) -> &Note {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == *self).unwrap();
    let interval_steps = fun_name(interval);
    let index = (start_index + interval_steps) % all_notes.len();
    &all_notes[index]
  }
}

fn fun_name(interval: Interval) -> usize {
  let half_tone_steps = match interval {
    Interval::Unison => 0,
    Interval::MinorSecond => 1,
    Interval::MajorSecond => 2,
    Interval::MinorThird => 3,
    Interval::MajorThird => 4,
    Interval::PerfectFourth => 5,
    Interval::Tritone => 6,
    Interval::PerfectFifth => 7,
    Interval::MinorSixth => 8,
    Interval::MajorSixth => 9,
    Interval::MinorSeventh => 10,
    Interval::MajorSeventh => 11,
    Interval::Octave => 12,
  };
  half_tone_steps
}

#[cfg(test)]
mod tests {
  use super::*;

  // TODO add tests

  #[test]
  fn test_major_scale() {
    let c_major = Scale::new(Note::C, ScaleType::Major);
    assert_eq!(c_major[ScaleDegree::First], Note::C);
    assert_eq!(c_major[ScaleDegree::Third], Note::E);
  }

  #[test]
  fn test_minor_scale() {
    let a_minor = Scale::new(Note::A, ScaleType::Minor);
    assert_eq!(a_minor[ScaleDegree::First], Note::A);
    assert_eq!(a_minor[ScaleDegree::Third], Note::C);

    let a_minor = Scale::new(Note::A, ScaleType::Major);
    assert_eq!(a_minor[ScaleDegree::First], Note::A);
    assert_eq!(a_minor[ScaleDegree::Third], Note::CisOrDes);
  }

  #[test]
  fn test_note_by_interval() {
    assert_eq!(*Note::C.get_note_by_interval(Interval::MajorThird), Note::E);
    assert_eq!(*Note::A.get_note_by_interval(Interval::MinorThird), Note::C);
    assert_eq!(
      *Note::A.get_note_by_interval(Interval::MajorThird),
      Note::CisOrDes
    );
  }
}
