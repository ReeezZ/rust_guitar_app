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

  fn get_all() -> Vec<Note> {
    vec![
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

  fn get_notes_from(start: Note, length: usize) -> Vec<Note> {
    let all_notes: Vec<Note> = Self::get_all();
    let start_index: usize = all_notes.iter().position(|&note| note == start).unwrap();

    (0..length)
      .map(|i: usize| all_notes[(start_index + i) % 12])
      .collect()
  }
}
