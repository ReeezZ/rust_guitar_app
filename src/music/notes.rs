// TODO split into multiple files

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

impl Note {
  pub fn all_notes() -> &'static [Note; 12] {
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

  pub fn get_note_by_interval(&self, interval: Interval) -> Note {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == *self).unwrap();
    let interval_steps = interval.half_tone_steps();
    let index = (start_index + interval_steps) % all_notes.len();
    all_notes[index]
  }
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

impl Interval {
  pub fn of(note: Note) -> Note {
    match note {
      // TODO: Implement this
    }
  }

  fn half_tone_steps(self) -> usize {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_interval_half_tone_steps() {
    assert_eq!(Interval::Unison.half_tone_steps(), 0);
    assert_eq!(Interval::MinorSecond.half_tone_steps(), 1);
    assert_eq!(Interval::MajorSecond.half_tone_steps(), 2);
    assert_eq!(Interval::MinorThird.half_tone_steps(), 3);
    assert_eq!(Interval::MajorThird.half_tone_steps(), 4);
    assert_eq!(Interval::PerfectFourth.half_tone_steps(), 5);
    assert_eq!(Interval::Tritone.half_tone_steps(), 6);
    assert_eq!(Interval::PerfectFifth.half_tone_steps(), 7);
    assert_eq!(Interval::MinorSixth.half_tone_steps(), 8);
    assert_eq!(Interval::MajorSixth.half_tone_steps(), 9);
    assert_eq!(Interval::MinorSeventh.half_tone_steps(), 10);
    assert_eq!(Interval::MajorSeventh.half_tone_steps(), 11);
    assert_eq!(Interval::Octave.half_tone_steps(), 12);
  }
}
