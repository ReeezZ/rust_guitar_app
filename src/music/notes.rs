use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Note {
  C,
  CSharpOrDFlat,
  D,
  DSharpOrEFlat,
  E,
  F,
  FSharpOrGFlat,
  G,
  GSharpOrAFlat,
  A,
  ASharpOrBFlat,
  B,
}

impl Note {
  pub const fn all_notes() -> &'static [Note; 12] {
    static ALL_NOTES: [Note; 12] = [
      Note::C,
      Note::CSharpOrDFlat,
      Note::D,
      Note::DSharpOrEFlat,
      Note::E,
      Note::F,
      Note::FSharpOrGFlat,
      Note::G,
      Note::GSharpOrAFlat,
      Note::A,
      Note::ASharpOrBFlat,
      Note::B,
    ];
    &ALL_NOTES
  }

  pub const fn mapping() -> &'static [(Note, &'static str)] {
    static MAPPING: [(Note, &'static str); 12] = [
      (Note::C, "C"),
      (Note::CSharpOrDFlat, "C♯/D♭"),
      (Note::D, "D"),
      (Note::DSharpOrEFlat, "D♯/E♭"),
      (Note::E, "E"),
      (Note::F, "F"),
      (Note::FSharpOrGFlat, "F♯/G♭"),
      (Note::G, "G"),
      (Note::GSharpOrAFlat, "G♯/A♭"),
      (Note::A, "A"),
      (Note::ASharpOrBFlat, "A♯/B♭"),
      (Note::B, "B"),
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
    assert_eq!(Note::C.add_steps(1), Note::CSharpOrDFlat);
    assert_eq!(Note::C.add_steps(2), Note::D);
    assert_eq!(Note::C.add_steps(3), Note::DSharpOrEFlat);
    assert_eq!(Note::C.add_steps(4), Note::E);
    assert_eq!(Note::C.add_steps(5), Note::F);
    assert_eq!(Note::C.add_steps(6), Note::FSharpOrGFlat);
    assert_eq!(Note::C.add_steps(7), Note::G);
    assert_eq!(Note::C.add_steps(8), Note::GSharpOrAFlat);
    assert_eq!(Note::C.add_steps(9), Note::A);
    assert_eq!(Note::C.add_steps(10), Note::ASharpOrBFlat);
    assert_eq!(Note::C.add_steps(11), Note::B);
    assert_eq!(Note::C.add_steps(12), Note::C);
  }
}
