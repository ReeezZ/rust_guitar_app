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
    let interval_steps = fun_name(interval);
    let index = (start_index + interval_steps) % all_notes.len();
    all_notes[index]
  }
}

fn fun_name(interval: Interval) -> usize {
  let half_tone_steps = match interval {
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
  };
  half_tone_steps
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_note_by_interval() {
    assert_eq!(Note::C.get_note_by_interval(Interval::MajorThird), Note::E);
    assert_eq!(Note::A.get_note_by_interval(Interval::MinorThird), Note::C);
    assert_eq!(
      Note::A.get_note_by_interval(Interval::MajorThird),
      Note::CisOrDes
    );
  }
}
