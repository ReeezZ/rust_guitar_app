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

// TODO maybe better as composite type
pub enum ScaleType {
  Major,
  Minor,
  MajorPentatonic,
  MinorPentatonic,
  MajorBlues,
  MinorBlues,
}

pub struct Scale {
  root_note: Note,
  current_note: Note,
}

impl Scale {
  fn chromatic(self, root_note: Note) -> Note {
    Note::next_note(self.current_note)
  }

  fn major(self, root_note: Note) -> Note {
    Note::next_note(self.current_note)
  }
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

  fn next_note(previous_note: Note) -> Note {
    match previous_note {
      Note::A => Note::AisOrB,
      Note::AisOrB => Note::H,
      Note::H => Note::C,
      Note::C => Note::CisOrDes,
      Note::CisOrDes => Note::D,
      Note::D => Note::DisOrEs,
      Note::DisOrEs => Note::E,
      Note::E => Note::F,
      Note::F => Note::FisOrGes,
      Note::FisOrGes => Note::G,
      Note::G => Note::GisOrAs,
      Note::GisOrAs => Note::A,
    }
  }

  fn major_second(note: Note) -> Note {
    match note {
      Note::A => Note::H,
      Note::AisOrB => Note::C,
      Note::H => Note::CisOrDes,
      Note::C => Note::D,
      Note::CisOrDes => Note::DisOrEs,
      Note::D => Note::E,
      Note::DisOrEs => Note::F,
      Note::E => Note::FisOrGes,
      Note::F => Note::G,
      Note::FisOrGes => Note::GisOrAs,
      Note::G => Note::A,
      Note::GisOrAs => Note::AisOrB,
    }
  }

  fn all_notes() -> [Note; 12] {
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
  }

  // fn scale(starting_note: Note, scale_type: ScaleType) -> impl Iterator<Item = Note> {
  //   let all_notes = Note::all_notes();

  //   let starting_note_iter = all_notes
  //     .iter()
  //     .copied()
  //     .cycle()
  //     .skip_while(move |n| *n != starting_note);

  //   starting_note_iter
  // }

  fn get_notes_from(start: Note, length: usize) -> impl Iterator<Item = Note> {
    let notes = Note::all_notes();
    let start_index: usize = notes.iter().position(|&n| n == start).unwrap();

    notes.into_iter().cycle().skip(start_index).take(length)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_note_as_str() {
    // let c_major = Scale::major(Note::C);
    // c_major.contains(Note::DisOrEs);
  }
}
