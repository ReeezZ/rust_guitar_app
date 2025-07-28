use std::{fmt, str::FromStr};

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
  pub const fn all_notes() -> &'static [Note; 12] {
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

  pub const fn mapping() -> &'static [(Note, &'static str)] {
    static MAPPING: [(Note, &'static str); 12] = [
      (Note::C, "C"),
      (Note::CisOrDes, "C♯/D♭"),
      (Note::D, "D"),
      (Note::DisOrEs, "D♯/E♭"),
      (Note::E, "E"),
      (Note::F, "F"),
      (Note::FisOrGes, "F♯/G♭"),
      (Note::G, "G"),
      (Note::GisOrAs, "G♯/A♭"),
      (Note::A, "A"),
      (Note::AisOrB, "A♯/B♭"),
      (Note::H, "B"),
    ];
    &MAPPING
  }

  /// Returns the note that is `steps` half-tone steps away from the current note.
  pub fn add_steps(&self, steps: usize) -> Note {
    let all_notes = Note::all_notes();
    let index = all_notes.iter().position(|&n| n == *self).unwrap();
    let new_index = (index + steps) % all_notes.len();
    all_notes[new_index]
  }
}

impl fmt::Display for Note {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let note_str = Self::mapping()
      .iter()
      .find_map(|(note, s)| if note == self { Some(*s) } else { None })
      .unwrap()
      .to_string();

    write!(f, "{}", note_str)
  }
}

impl FromStr for Note {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::mapping()
      .iter()
      .find_map(|(note, str)| if *str == s { Some(*note) } else { None })
      .ok_or(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
