#![allow(dead_code)]
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
  fn as_str(&self) -> &'static str {
    match self {
      Note::C => "C",
      Note::CisOrDes => "C♯/D♭",
      Note::D => "D",
      Note::DisOrEs => "D♯/E♭",
      Note::E => "E",
      Note::F => "F",
      Note::FisOrGes => "F♯/G♭",
      Note::G => "G",
      Note::GisOrAs => "G♯/A♭",
      Note::A => "A",
      Note::AisOrB => "A♯/B",
      Note::H => "H",
    }
  }

  fn iter() -> impl Iterator<Item = Note> {
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
    .into_iter()
  }

  fn get_notes_from(start: Note, length: usize) -> impl Iterator<Item = Note> {
    let notes: Vec<Note> = Note::iter().collect();
    let start_index: usize = notes.iter().position(|&n| n == start).unwrap();

    notes.into_iter().cycle().skip(start_index).take(length)
  }
}
