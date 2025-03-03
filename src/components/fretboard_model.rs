use leptos::prelude::*;

use crate::music::{
  notes::Note,
  scales::{Scale, ScaleTrait},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FretState {
  Hidden,
  Normal,
  Root,
}

#[derive(Clone, Debug)]
pub struct FretboardModel {
  frets: Vec<Vec<RwSignal<FretState>>>,
  num_strings: u8,
  num_frets: RwSignal<u8>,
  tuning: Vec<Note>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}

impl FretboardModel {
  pub fn new(num_strings: u8, num_frets: u8, tuning: Vec<Note>) -> Self {
    assert_eq!(
      num_strings as usize,
      tuning.len(),
      "Tuning vector length must match the number of strings"
    );

    let frets = Self::generate_frets(num_strings, num_frets);

    let mut model = FretboardModel {
      frets,
      num_strings,
      num_frets: RwSignal::new(num_frets),
      tuning,
    };

    model
  }

  fn generate_frets(num_strings: u8, num_frets: u8) -> Vec<Vec<RwSignal<FretState>>> {
    let mut frets = Vec::with_capacity(num_strings as usize);

    for _ in 0..=num_strings {
      let mut string_frets = Vec::with_capacity(num_frets as usize);
      for _ in 0..=num_frets {
        string_frets.push(RwSignal::new(FretState::Hidden));
      }
      frets.push(string_frets);
    }

    frets
  }

  pub fn update_num_frets(&mut self, num_frets: u8) {
    if num_frets <= self.num_frets.get() {
      return;
    }

    for i in 0..=self.num_strings as usize {
      for _ in self.frets[i].len() as u8..=num_frets {
        self.frets[i].push(RwSignal::new(FretState::Hidden));
      }
    }

    self.num_frets.set(num_frets);
  }

  pub fn six_string_standard_tuning(num_frets: u8) -> Self {
    Self::new(6, num_frets, Self::standard_tuning())
  }

  fn get_fret_state(&self, coord: FretCoord) -> RwSignal<FretState> {
    self.frets[coord.string_idx as usize][coord.fret_idx as usize]
  }

  fn set_fret_state(&self, coord: FretCoord, state: FretState) {
    self.get_fret_state(coord).set(state);
  }

  fn set_all(&self, state: FretState) {
    for string in &self.frets {
      for &fret in string {
        fret.set(state);
      }
    }
  }

  pub fn get_num_strings(&self) -> u8 {
    self.num_strings
  }

  pub fn get_num_frets(&self) -> RwSignal<u8> {
    self.num_frets
  }

  pub fn get_tuning(&self) -> &[Note] {
    &self.tuning
  }

  pub fn standard_tuning() -> Vec<Note> {
    vec![Note::E, Note::A, Note::D, Note::G, Note::H, Note::E]
  }

  pub fn get_frets_of_string(&self, string_no: u8) -> &Vec<RwSignal<FretState>> {
    &self.frets[string_no as usize]
  }

  fn determine_fret_state(note: Note, scale: &Scale) -> FretState {
    if scale.contains_note(note) {
      match scale.root_note() {
        Some(root_note) if root_note == note => FretState::Root,
        _ => FretState::Normal,
      }
    } else {
      FretState::Hidden
    }
  }

  pub fn update_from_scale(&self, scale: &Scale) {
    for (string_idx, &tuning) in self.tuning.iter().enumerate() {
      let string_idx = string_idx as u8;

      // Open string (fret 0)
      let open_state = Self::determine_fret_state(tuning, scale);

      if string_idx < self.num_strings {
        let coord = FretCoord {
          string_idx,
          fret_idx: 0,
        };
        self.set_fret_state(coord, open_state);
      }

      // Fretted notes
      for fret_idx in 1..=self.num_frets.get() {
        let note = tuning.add_steps(fret_idx as usize);

        let state = Self::determine_fret_state(note, scale);

        if string_idx < self.num_strings && fret_idx <= self.num_frets.get() {
          let coord = FretCoord {
            string_idx,
            fret_idx,
          };
          self.set_fret_state(coord, state);
        }
      }
    }
  }
}
