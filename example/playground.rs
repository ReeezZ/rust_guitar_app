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
  UnisonOrOctave,
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
}

pub enum ScaleType {
  Major,
  Minor,
  MajorPentatonic,
  MinorPentatonic,
  MajorBlues,
  MinorBlues,
}

pub struct Scale {
  root_note: Note,
  notes: Vec<Note>,
}

fn get_note_half_tone_steps_from(start: Note, half_tone_steps: usize) -> Note {
  let notes = Note::all_notes();
  let start_index: usize = notes.iter().position(|&n| n == start).unwrap();

  notes
    .into_iter()
    .cycle()
    .skip(start_index)
    .skip(half_tone_steps)
    .next()
    .expect("test")
}

impl Scale {
  pub fn new(root_note: Note, scale_type: ScaleType) -> Self {
    let notes = match scale_type {
      ScaleType::Major => Self::generate_major_scale(root_note),
      ScaleType::Minor => Self::generate_minor_scale(root_note),
      _ => panic!("Scale type not implemented"),
    };
    Scale { root_note, notes }
  }

  fn generate_major_scale(root_note: Note) -> Vec<Note> {
    let intervals = [
      Interval::MajorSecond,
      Interval::MajorThird,
      Interval::PerfectFourth,
      Interval::PerfectFifth,
      Interval::MajorSixth,
      Interval::MajorSeventh,
      Interval::UnisonOrOctave,
    ];
    Self::generate_scale(root_note, &intervals)
  }

  fn generate_minor_scale(root_note: Note) -> Vec<Note> {
    let intervals = [
      Interval::MajorSecond,
      Interval::MinorThird,
      Interval::PerfectFourth,
      Interval::PerfectFifth,
      Interval::MinorSixth,
      Interval::MinorSeventh,
      Interval::UnisonOrOctave,
    ];
    Self::generate_scale(root_note, &intervals)
  }

  fn generate_scale(root_note: Note, intervals: &[Interval]) -> Vec<Note> {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == root_note).unwrap();
    intervals
      .iter()
      .map(|&interval| {
        let index = (start_index + interval as usize) % all_notes.len();
        all_notes[index]
      })
      .collect()
  }
}

impl Note {
  fn all_notes() -> [Note; 12] {
    [
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
    ]
  }
}

impl Index<Interval> for Note {
  type Output = Note;

  fn index(&self, interval: Interval) -> &Self::Output {
    let index = match interval {
      Interval::UnisonOrOctave => 0,
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
    };

    &self.notes[index % self.notes.len()]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_note_as_str() {
    let c_major = Scale::new(Note::C, ScaleType::Major);
    assert_eq!(c_major[Interval::UnisonOrOctave], Note::C);
    assert_eq!(c_major[Interval::MajorThird], Note::E);
  }
}

fn main() {
  println!("Hello Leptos");
}
