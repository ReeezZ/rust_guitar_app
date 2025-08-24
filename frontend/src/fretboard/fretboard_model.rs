use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;
use shared::{Note, Scale, ScaleTrait};

use crate::fretboard::components::{
  base::{FretState, FretStateColor},
  visual_config::FretboardVisualConfig,
};

pub(crate) type FretStateSignals = HashMap<FretCoord, RwSignal<FretState>>;

pub fn get_preallocated_fret_states() -> FretStateSignals {
  let mut map = FretStateSignals::with_capacity(MAX_STRINGS * MAX_FRETS);
  for string_idx in 0..=MAX_STRINGS {
    for fret_idx in 0..=MAX_FRETS {
      let coord = FretCoord {
        string_idx: string_idx as u8,
        fret_idx: fret_idx as u8,
      };
      map.insert(coord, RwSignal::new(FretState::Hidden));
    }
  }
  map
}

// Upper bounds used for preallocation; keeps per-cell signals stable (never created inside Effects).
// Adjust if you need more strings/frets; existing UI sliders should clamp within these maxima.
pub const MAX_STRINGS: usize = 8; // supports up to 8-string instruments
pub const MAX_FRETS: usize = 25; // frets 0..=24

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub note: Note,
  pub coord: FretCoord,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FretboardModel {
  /// First fret in the active/playable range
  start_fret: RwSignal<usize>,
  /// Last fret in the active/playable range
  end_fret: RwSignal<usize>,
  /// Tuning of the guitar strings, first index is the lowest string (6th string)
  tuning: RwSignal<Vec<Note>>,
  /// Visual configuration for fretboard display properties
  config: RwSignal<FretboardVisualConfig>,
  /// States for each fret
  fret_states: RwSignal<FretStateSignals>,
  /// Optional callback for fret click events
  on_note_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
}
pub fn default_tuning() -> RwSignal<Vec<Note>> {
  RwSignal::new(vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E])
}

impl Default for FretboardModel {
  fn default() -> Self {
    let start_fret = 1;
    let end_fret = 9;
    // Preallocate all possible per-cell signals once in the model's construction scope.
    let fret_states = RwSignal::new(get_preallocated_fret_states());
    Self {
      start_fret: RwSignal::new(start_fret),
      end_fret: RwSignal::new(end_fret),
      tuning: default_tuning(),
      config: RwSignal::new(FretboardVisualConfig::default()),
      on_note_clicked: RwSignal::new(None).into(),
      fret_states,
    }
  }
}

impl FretboardModel {
  // pub fn get_num_frets(self) -> Signal<usize> {
  //   Signal::derive(move || self.end_fret.get() - self.start_fret.get() + 1)
  // }
  pub fn get_num_frets_untracked(&self) -> usize {
    self.end_fret.get_untracked() - self.start_fret.get_untracked() + 1
  }
  // pub fn get_tuning_untracked(&self) -> Vec<Note> {
  //   self.tuning.get_untracked().clone()
  // }

  pub fn get_tuning(&self) -> Signal<Vec<Note>> {
    self.tuning.into()
  }
  pub fn set_tuning(&self, new_tuning: Vec<Note>) {
    // todo handle fret states when new tuning has more strings
    self.tuning.set(new_tuning);
  }

  pub fn get_start_fret(&self) -> Signal<usize> {
    self.start_fret.into()
  }
  pub fn set_start_fret(&self, new_start_fret: usize) {
    self.start_fret.set(new_start_fret);
  }

  pub fn get_end_fret(&self) -> Signal<usize> {
    self.end_fret.into()
  }
  pub fn set_end_fret(&self, new_end_fret: usize) {
    self.end_fret.set(new_end_fret);
  }

  pub fn get_config(&self) -> Signal<FretboardVisualConfig> {
    self.config.into()
  }
  pub fn set_config(&self, new_config: FretboardVisualConfig) {
    self.config.set(new_config);
  }

  pub fn get_fret_states(&self) -> Signal<FretStateSignals> {
    self.fret_states.into()
  }

  pub fn get_on_note_clicked(&self) -> Signal<Option<Callback<FretClickEvent>>> {
    self.on_note_clicked.into()
  }
  pub fn set_on_note_clicked(&self, callback: Option<Callback<FretClickEvent>>) {
    self.on_note_clicked.set(callback);
  }

  pub fn update_visual_config(&self, update_fn: impl FnOnce(&mut FretboardVisualConfig)) {
    self.config.update(|config| {
      update_fn(config);
    });
  }

  pub fn set_fret_states(&self, new_states: FretStateSignals) {
    // Merge into existing preallocated signals (no new signal creation during reactive updates).
    self.fret_states.update(|existing| {
      for (coord, state_signal) in new_states.into_iter() {
        if let Some(dest) = existing.get(&coord) {
          dest.set(state_signal.get());
        }
      }
    });
  }

  fn get_min_fret(&self) -> usize {
    self
      .start_fret
      .get_untracked()
      .saturating_sub(self.config.get_untracked().extra_frets.get())
  }

  fn get_max_fret(&self) -> usize {
    self.end_fret.get() + self.config.get_untracked().extra_frets.get()
  }

  pub fn update_from_scale(&self, scale: Scale) {
    self
      .tuning
      .get_untracked()
      .iter()
      .enumerate()
      .for_each(|(string_idx, string_note)| {
        for fret_idx in self.get_min_fret()..=self.get_max_fret() {
          let coord = FretCoord {
            string_idx: string_idx as u8,
            fret_idx: fret_idx as u8,
          };
          let note_at_fret = string_note.add_steps(fret_idx);
          let state = if scale.root_note() == Some(note_at_fret) {
            FretState::Normal(FretStateColor::Green, note_at_fret.to_string())
          } else if scale.contains_note(note_at_fret) {
            FretState::Normal(FretStateColor::Blue, note_at_fret.to_string())
          } else {
            FretState::Hidden
          };
          self.fret_states.update(|fret_states| {
            if let Some(sig) = fret_states.get(&coord) {
              sig.set(state);
            }
          });
        }
      });
  }
}
