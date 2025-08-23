use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;
use shared::{Note, Scale, ScaleTrait};

use crate::fretboard::components::{
  base::{FretState, FretStateColor},
  visual_config::FretboardVisualConfig,
};

pub(crate) type FretStateSignals = HashMap<FretCoord, RwSignal<FretState>>;

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
    Self {
      start_fret: RwSignal::new(1),
      end_fret: RwSignal::new(9),
      tuning: default_tuning(),
      config: RwSignal::new(FretboardVisualConfig::default()),
      on_note_clicked: RwSignal::new(None).into(),
      fret_states: RwSignal::new(HashMap::new()),
    }
  }
}

impl FretboardModel {
  pub fn get_num_frets(self) -> Signal<usize> {
    Signal::derive(move || self.end_fret.get() - self.start_fret.get() + 1)
  }
  pub fn get_num_frets_untracked(&self) -> usize {
    self.end_fret.get_untracked() - self.start_fret.get_untracked() + 1
  }
  pub fn get_tuning_untracked(&self) -> Vec<Note> {
    self.tuning.get_untracked().clone()
  }

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
    Self::ensure_all_fret_states_exist(
      self.fret_states,
      self.tuning.get_untracked().len(),
      new_start_fret,
      self.end_fret.get_untracked(),
    );
    self.start_fret.set(new_start_fret);
  }

  pub fn get_end_fret(&self) -> Signal<usize> {
    self.end_fret.into()
  }
  pub fn set_end_fret(&self, new_end_fret: usize) {
    Self::ensure_all_fret_states_exist(
      self.fret_states,
      self.tuning.get_untracked().len(),
      self.start_fret.get_untracked(),
      new_end_fret,
    );
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
    // TODO the signal is not really required
    let signal = RwSignal::new(new_states);
    Self::ensure_all_fret_states_exist(
      signal,
      self.tuning.get_untracked().len(),
      self.start_fret.get_untracked(),
      self.end_fret.get_untracked(),
    );
    self.fret_states.set(signal.get());
  }

  pub fn update_from_scale(&self, scale: Scale) {
    self
      .tuning
      .get_untracked()
      .iter()
      .enumerate()
      .for_each(|(string_idx, string_note)| {
        for fret_idx in self.start_fret.get_untracked()..=self.end_fret.get_untracked() {
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
            match fret_states.get(&coord) {
              Some(existing_signal) => existing_signal.set(state),
              None => {
                fret_states.insert(coord, RwSignal::new(state));
              }
            };
          });
        }
      });
  }

  fn ensure_all_fret_states_exist(
    fret_states: RwSignal<FretStateSignals>,
    num_strings: usize,
    start_fret: usize,
    end_fret: usize,
  ) {
    fret_states.update(|fret_states| {
      for string_idx in 0..num_strings {
        for fret_idx in start_fret..=end_fret {
          let coord = FretCoord {
            string_idx: string_idx as u8,
            fret_idx: fret_idx as u8,
          };
          if !fret_states.contains_key(&coord) {
            fret_states.insert(coord, RwSignal::new(FretState::Hidden));
          }
        }
      }
    });
  }
}
