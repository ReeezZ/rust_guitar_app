use std::ops::Index;

use super::intervals::Interval;
use super::notes::Note;
use super::scales::ScaleTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeptaScaleDegree {
  First,
  Second,
  Third,
  Fourth,
  Fifth,
  Sixth,
  Seventh,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeptaScaleType {
  Major,
  Minor,
  // TODO add more scale types, the list below is not complete
  // MelodicMinor,
  // HarmonicMinor,
  // Dorian,
  // Phrygian,
  // Lydian,
  // Mixolydian,
  // Locrian,
}

impl HeptaScaleType {
  pub fn to_string(&self) -> String {
    match self {
      HeptaScaleType::Major => "Major".to_string(),
      HeptaScaleType::Minor => "Minor".to_string(),
    }
  }

  pub fn all_scale_types() -> Vec<HeptaScaleType> {
    vec![HeptaScaleType::Major, HeptaScaleType::Minor]
  }
}

pub trait HeptaScale: Index<HeptaScaleDegree> + ScaleTrait {
  fn get_note_by_degree(&self, degree: HeptaScaleDegree) -> Note;
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaScaleImpl {
  notes: [Note; 7],
  scale_type: HeptaScaleType,
}

impl HeptaScaleImpl {
  pub fn new(root_note: Note, scale_type: HeptaScaleType) -> HeptaScaleImpl {
    let notes = Self::get_notes(root_note, scale_type);
    HeptaScaleImpl { notes, scale_type }
  }

  pub fn get_note_by_degree(&self, degree: HeptaScaleDegree) -> &Note {
    match degree {
      HeptaScaleDegree::First => &self.notes[0],
      HeptaScaleDegree::Second => &self.notes[1],
      HeptaScaleDegree::Third => &self.notes[2],
      HeptaScaleDegree::Fourth => &self.notes[3],
      HeptaScaleDegree::Fifth => &self.notes[4],
      HeptaScaleDegree::Sixth => &self.notes[5],
      HeptaScaleDegree::Seventh => &self.notes[6],
    }
  }

  fn get_notes(root_note: Note, scale_type: HeptaScaleType) -> [Note; 7] {
    match scale_type {
      HeptaScaleType::Major => Self::generate_major_scale(root_note),
      HeptaScaleType::Minor => Self::generate_minor_scale(root_note),
    }
  }

  fn generate_major_scale(root_note: Note) -> [Note; 7] {
    let intervals = [
      Interval::Unison,
      Interval::MajorSecond,
      Interval::MajorThird,
      Interval::PerfectFourth,
      Interval::PerfectFifth,
      Interval::MajorSixth,
      Interval::MajorSeventh,
    ];
    Self::generate_scale(root_note, intervals)
  }

  fn generate_minor_scale(root_note: Note) -> [Note; 7] {
    let intervals = [
      Interval::Unison,
      Interval::MajorSecond,
      Interval::MinorThird,
      Interval::PerfectFourth,
      Interval::PerfectFifth,
      Interval::MinorSixth,
      Interval::MinorSeventh,
    ];

    Self::generate_scale(root_note, intervals)
  }

  fn generate_scale(root_note: Note, intervals: [Interval; 7]) -> [Note; 7] {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == root_note).unwrap();

    intervals.map(move |interval| {
      let index = (start_index + interval.half_tone_steps()) % all_notes.len();
      all_notes[index]
    })
  }

  pub fn contains_note(&self, note: Note) -> bool {
    self.notes.contains(&note)
  }

  pub fn root_note(&self) -> Note {
    self.notes[0]
  }

  pub fn to_string(&self) -> String {
    todo!()
  }
}

impl Index<HeptaScaleDegree> for HeptaScaleImpl {
  type Output = Note;

  fn index(&self, index: HeptaScaleDegree) -> &Self::Output {
    &self.get_note_by_degree(index)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // TODO add more tests

  #[test]
  fn test_c_major_scale() {
    let c_major = HeptaScaleImpl::new(Note::C, HeptaScaleType::Major);
    assert_eq!(c_major[HeptaScaleDegree::First], Note::C);
    assert_eq!(c_major[HeptaScaleDegree::Second], Note::D);
    assert_eq!(c_major[HeptaScaleDegree::Third], Note::E);
    assert_eq!(c_major[HeptaScaleDegree::Fourth], Note::F);
    assert_eq!(c_major[HeptaScaleDegree::Fifth], Note::G);
    assert_eq!(c_major[HeptaScaleDegree::Sixth], Note::A);
    assert_eq!(c_major[HeptaScaleDegree::Seventh], Note::H);
  }

  #[test]
  fn test_a_minor_scale() {
    let a_minor = HeptaScaleImpl::new(Note::A, HeptaScaleType::Minor);
    assert_eq!(a_minor[HeptaScaleDegree::First], Note::A);
    assert_eq!(a_minor[HeptaScaleDegree::Third], Note::C);
  }

  #[test]
  fn test_a_major_scale() {
    let a_minor = HeptaScaleImpl::new(Note::A, HeptaScaleType::Major);
    assert_eq!(a_minor[HeptaScaleDegree::First], Note::A);
    assert_eq!(a_minor[HeptaScaleDegree::Second], Note::H);
    assert_eq!(a_minor[HeptaScaleDegree::Third], Note::CisOrDes);
    assert_eq!(a_minor[HeptaScaleDegree::Fourth], Note::D);
    assert_eq!(a_minor[HeptaScaleDegree::Fifth], Note::E);
    assert_eq!(a_minor[HeptaScaleDegree::Sixth], Note::FisOrGes);
    assert_eq!(a_minor[HeptaScaleDegree::Seventh], Note::GisOrAs);
  }
}
