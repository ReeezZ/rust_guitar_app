use crate::music::notes::Note;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, EnumIter, Debug)]
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

  pub fn from_notes(from: Note, to: Note) -> Option<Interval> {
    let all_notes = Note::all_notes();
    let from_index = all_notes.iter().position(|&n| n == from)?;
    let to_index = all_notes.iter().position(|&n| n == to)?;

    let half_tone_steps = if to_index >= from_index {
      to_index - from_index
    } else {
      (to_index + all_notes.len()) - from_index
    };

    match half_tone_steps {
      0 => Some(Interval::Unison),
      1 => Some(Interval::MinorSecond),
      2 => Some(Interval::MajorSecond),
      3 => Some(Interval::MinorThird),
      4 => Some(Interval::MajorThird),
      5 => Some(Interval::PerfectFourth),
      6 => Some(Interval::Tritone),
      7 => Some(Interval::PerfectFifth),
      8 => Some(Interval::MinorSixth),
      9 => Some(Interval::MajorSixth),
      10 => Some(Interval::MinorSeventh),
      11 => Some(Interval::MajorSeventh),
      12 => Some(Interval::Octave),
      _ => None,
    }
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

  #[test]
  fn test_intervals_from_notes() {
    assert_eq!(
      Interval::from_notes(Note::C, Note::C),
      Some(Interval::Unison)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::CSharpOrDFlat),
      Some(Interval::MinorSecond)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::D),
      Some(Interval::MajorSecond)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::DSharpOrEFlat),
      Some(Interval::MinorThird)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::E),
      Some(Interval::MajorThird)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::F),
      Some(Interval::PerfectFourth)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::FSharpOrGFlat),
      Some(Interval::Tritone)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::G),
      Some(Interval::PerfectFifth)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::GSharpOrAFlat),
      Some(Interval::MinorSixth)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::A),
      Some(Interval::MajorSixth)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::ASharpOrBFlat),
      Some(Interval::MinorSeventh)
    );
    assert_eq!(
      Interval::from_notes(Note::C, Note::B),
      Some(Interval::MajorSeventh)
    );
    // assert_eq!(
    //   Interval::from_notes(Note::C, Note::C),
    //   Some(Interval::Octave)
    // );

    // Wrap around
    assert_eq!(
      Interval::from_notes(Note::B, Note::C),
      Some(Interval::MinorSecond)
    );
    assert_eq!(
      Interval::from_notes(Note::B, Note::D),
      Some(Interval::MinorThird)
    );
  }
}
