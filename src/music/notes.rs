use std::fmt;

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

  pub fn mapping() -> &'static [(Note, &'static str)] {
    static MAPPING: [(Note, &'static str); 12] = [
      (Note::C, "C"),
      (Note::CisOrDes, "C#/Db"),
      (Note::D, "D"),
      (Note::DisOrEs, "D#/Eb"),
      (Note::E, "E"),
      (Note::F, "F"),
      (Note::FisOrGes, "F#/Gb"),
      (Note::G, "G"),
      (Note::GisOrAs, "G#/Ab"),
      (Note::A, "A"),
      (Note::AisOrB, "A#/Bb"),
      (Note::H, "H"),
    ];
    &MAPPING
  }

  pub fn as_str(&self) -> &'static str {
    Self::mapping()
      .iter()
      .find_map(|(note, s)| if note == self { Some(*s) } else { None })
      .unwrap()
  }

  pub fn from_str(s: &str) -> Option<Self> {
    Self::mapping()
      .iter()
      .find_map(|(note, str)| if *str == s { Some(*note) } else { None })
  }

  /// Returns the note that is `steps` half-tone steps away from the current note.
  pub fn add_steps(&self, steps: usize) -> Note {
    let all_notes = Note::all_notes();
    let index = all_notes.iter().position(|&n| n == *self).unwrap();
    let new_index = (index + steps) % all_notes.len();
    all_notes[new_index]
  }

  pub fn add_interval(&self, interval: Interval) -> Note {
    let all_notes = Note::all_notes();
    let start_index = all_notes.iter().position(|&n| n == *self).unwrap();
    let interval_steps = interval.half_tone_steps();
    let index = (start_index + interval_steps) % all_notes.len();
    all_notes[index]
  }
}

impl fmt::Display for Note {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let note_str = match self {
      Note::C => "C",
      Note::CisOrDes => "C#/Db",
      Note::D => "D",
      Note::DisOrEs => "D#/Eb",
      Note::E => "E",
      Note::F => "F",
      Note::FisOrGes => "F#/Gb",
      Note::G => "G",
      Note::GisOrAs => "G#/Ab",
      Note::A => "A",
      Note::AisOrB => "A#/Bb",
      Note::H => "H",
    };
    write!(f, "{}", note_str)
  }
}

#[derive(Clone, Copy, PartialEq)]
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
    note.add_interval(self)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_intervals_of_c() {
    assert_eq!(Interval::Unison.of(Note::C), Note::C);
    assert_eq!(Interval::MinorSecond.of(Note::C), Note::CisOrDes);
    assert_eq!(Interval::MajorSecond.of(Note::C), Note::D);
    assert_eq!(Interval::MinorThird.of(Note::C), Note::DisOrEs);
    assert_eq!(Interval::MajorThird.of(Note::C), Note::E);
    assert_eq!(Interval::PerfectFourth.of(Note::C), Note::F);
    assert_eq!(Interval::Tritone.of(Note::C), Note::FisOrGes);
    assert_eq!(Interval::PerfectFifth.of(Note::C), Note::G);
    assert_eq!(Interval::MinorSixth.of(Note::C), Note::GisOrAs);
    assert_eq!(Interval::MajorSixth.of(Note::C), Note::A);
    assert_eq!(Interval::MinorSeventh.of(Note::C), Note::AisOrB);
    assert_eq!(Interval::MajorSeventh.of(Note::C), Note::H);
    assert_eq!(Interval::Octave.of(Note::C), Note::C);
  }

  #[test]
  fn test_add_steps_of_c() {
    assert_eq!(Note::C.add_steps(0), Note::C);
    assert_eq!(Note::C.add_steps(1), Note::CisOrDes);
    assert_eq!(Note::C.add_steps(2), Note::D);
    assert_eq!(Note::C.add_steps(3), Note::DisOrEs);
    assert_eq!(Note::C.add_steps(4), Note::E);
    assert_eq!(Note::C.add_steps(5), Note::F);
    assert_eq!(Note::C.add_steps(6), Note::FisOrGes);
    assert_eq!(Note::C.add_steps(7), Note::G);
    assert_eq!(Note::C.add_steps(8), Note::GisOrAs);
    assert_eq!(Note::C.add_steps(9), Note::A);
    assert_eq!(Note::C.add_steps(10), Note::AisOrB);
    assert_eq!(Note::C.add_steps(11), Note::H);
    assert_eq!(Note::C.add_steps(12), Note::C);
  }
}
