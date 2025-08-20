use leptos::{logging::log, prelude::*};

use shared::music::{
  notes::Note,
  scales::{Scale, ScaleTrait},
};

use crate::fretboard::components::base::{FretState, FretStateColor};

pub type FretNoteSignal = RwSignal<FretState>;
pub type FretStringSignals = RwSignal<Vec<FretNoteSignal>>;
pub type FretboardSignals = Vec<FretStringSignals>;

#[derive(Clone, Debug)]
pub struct FretboardModel {
  frets: FretboardSignals,
  num_strings: u8,
  num_frets: RwSignal<u8>,
  tuning: Vec<Note>,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
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

    FretboardModel {
      frets,
      num_strings,
      num_frets: RwSignal::new(num_frets),
      tuning,
    }
  }

  fn generate_frets(num_strings: u8, num_frets: u8) -> FretboardSignals {
    let mut frets: FretboardSignals = Vec::with_capacity(num_strings as usize);

    for _ in 0..=num_strings {
      let mut string_frets = Vec::with_capacity(num_frets as usize);
      for _ in 0..=num_frets {
        string_frets.push(RwSignal::new(FretState::Hidden));
      }
      frets.push(RwSignal::new(string_frets));
    }

    frets
  }

  pub fn update_num_frets(&mut self, num_frets: u8) {
    let current = self.num_frets.get();

    // Only update if actually changing
    if num_frets == current {
      return;
    }

    log!("Updating number of frets from {} to {}", current, num_frets);
    // Set the new number of frets
    self.num_frets.set(num_frets);

    if num_frets > current {
      for string_idx in 0..self.num_strings as usize {
        let string_frets = &mut self.frets[string_idx];

        // Add additional fret signals
        for _ in (current + 1)..=num_frets {
          string_frets.update(|string_frets| string_frets.push(RwSignal::new(FretState::Hidden)));
        }
      }
    }
    // If decreasing, we can leave the extra frets in the array
    // as they won't be displayed or accessed
  }

  pub fn six_string_standard_tuning(num_frets: u8) -> Self {
    Self::new(6, num_frets, Self::standard_tuning())
  }

  fn get_fret_state(&self, coord: FretCoord) -> RwSignal<FretState> {
    self.frets[coord.string_idx as usize].get_untracked()[coord.fret_idx as usize]
  }

  pub fn set_fret_state(&self, coord: FretCoord, state: FretState) {
    self.get_fret_state(coord).set(state);
  }

  pub fn set_all(&self, state: FretState) {
    for string in &self.frets {
      for fret in string.get() {
        fret.set(state.clone());
      }
    }
  }

  pub fn get_num_strings(&self) -> u8 {
    self.num_strings
  }

  pub fn get_num_frets(&self) -> ReadSignal<u8> {
    self.num_frets.read_only()
  }

  pub fn get_tuning(&self) -> &[Note] {
    &self.tuning
  }

  pub fn standard_tuning() -> Vec<Note> {
    // Standard guitar tuning from thinnest to thickest string (top to bottom on fretboard display)
    // String 0 (top): High E (1st string)
    // String 1: B (2nd string)
    // String 2: G (3rd string)
    // String 3: D (4th string)
    // String 4: A (5th string)
    // String 5 (bottom): Low E (6th string)
    vec![Note::E, Note::B, Note::G, Note::D, Note::A, Note::E]
  }

  pub fn get_frets_of_string(&self, string_no: u8) -> FretStringSignals {
    self.frets[string_no as usize]
  }

  fn determine_fret_state(note: Note, scale: &Scale) -> FretState {
    if scale.contains_note(note) {
      match scale.root_note() {
        Some(root_note) if root_note == note => {
          FretState::Normal(FretStateColor::Red, note.to_string())
        }
        _ => FretState::Normal(FretStateColor::Blue, note.to_string()),
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
