use crate::music::notes::Note;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, EnumIter)]
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

impl Interval {
  pub fn of(self, note: Note) -> Note {
    note.add_steps(self.half_tone_steps())
  }

  pub fn half_tone_steps(self) -> usize {
    match self {
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
    }
  }
}

impl fmt::Display for Interval {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Interval::Unison => write!(f, "Unison"),
      Interval::MinorSecond => write!(f, "Minor Second"),
      Interval::MajorSecond => write!(f, "Major Second"),
      Interval::MinorThird => write!(f, "Minor Third"),
      Interval::MajorThird => write!(f, "Major Third"),
      Interval::PerfectFourth => write!(f, "Perfect Fourth"),
      Interval::Tritone => write!(f, "Tritone"),
      Interval::PerfectFifth => write!(f, "Perfect Fifth"),
      Interval::MinorSixth => write!(f, "Minor Sixth"),
      Interval::MajorSixth => write!(f, "Major Sixth"),
      Interval::MinorSeventh => write!(f, "Minor Seventh"),
      Interval::MajorSeventh => write!(f, "Major Seventh"),
      Interval::Octave => write!(f, "Octave"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_intervals_of_c() {
    assert_eq!(Interval::Unison.of(Note::C), Note::C);
    assert_eq!(Interval::MinorSecond.of(Note::C), Note::CSharpOrDFlat);
    assert_eq!(Interval::MajorSecond.of(Note::C), Note::D);
    assert_eq!(Interval::MinorThird.of(Note::C), Note::DSharpOrEFlat);
    assert_eq!(Interval::MajorThird.of(Note::C), Note::E);
    assert_eq!(Interval::PerfectFourth.of(Note::C), Note::F);
    assert_eq!(Interval::Tritone.of(Note::C), Note::FSharpOrGFlat);
    assert_eq!(Interval::PerfectFifth.of(Note::C), Note::G);
    assert_eq!(Interval::MinorSixth.of(Note::C), Note::GSharpOrAFlat);
    assert_eq!(Interval::MajorSixth.of(Note::C), Note::A);
    assert_eq!(Interval::MinorSeventh.of(Note::C), Note::ASharpOrBFlat);
    assert_eq!(Interval::MajorSeventh.of(Note::C), Note::B);
    assert_eq!(Interval::Octave.of(Note::C), Note::C);
  }
}
