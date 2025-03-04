use std::fmt;

// TODO use strum::enumIter
// https://docs.rs/strum/latest/strum/derive.EnumIter.html
// it's even a simple derive macro :>
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
    // TODO maybe lazy init
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
  pub const fn add_steps(&self, steps: usize) -> Note {
    let all_notes = Note::all_notes();
    let len = all_notes.len();
    let mut i = 0;
    while i < len {
      // TODO if this worked it would be super nice
      if all_notes[i] == *self {
        let new_index = (i + steps) % len;
        return all_notes[new_index];
      }
      i += 1;
    }

    // This should never happen if the Note is valid
    all_notes[0] // Return C as a fallback
  }
}

impl fmt::Display for Note {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // TODO use unicode sharp and flat symbols
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
