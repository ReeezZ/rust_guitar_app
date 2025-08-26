use leptos::prelude::*;
use shared::Note;

use crate::components::fretboard::{FretCoord, FretState, FretStateSignals, FretboardVisualConfig};
use crate::models::fretboard::model_builder::FretboardModelBuilder;

#[derive(Clone, Debug, PartialEq)]
pub struct FretboardModel {
  /// First fret in the active/playable range
  pub start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: Signal<usize>,
  /// Tuning of the guitar strings, first index is the lowest string (6th string)
  pub tuning: Signal<Vec<Note>>,
  /// Visual configuration for fretboard display properties
  pub config: Signal<FretboardVisualConfig>,
  /// States for each fret
  pub fret_states: Signal<FretStateSignals>,
}

impl Default for FretboardModel {
  fn default() -> Self {
    FretboardModelBuilder::new().build()
  }
}

impl FretboardModel {
  pub fn new(
    start_fret: Signal<usize>,
    end_fret: Signal<usize>,
    tuning: Signal<Vec<Note>>,
    config: Signal<FretboardVisualConfig>,
    fret_states: Signal<FretStateSignals>,
  ) -> Self {
    Self {
      start_fret,
      end_fret,
      tuning,
      config,
      fret_states,
    }
  }

  pub fn get_num_frets_untracked(&self) -> usize {
    self.end_fret.get_untracked() - self.start_fret.get_untracked() + 1
  }

  pub fn get_tuning(&self) -> Signal<Vec<Note>> {
    self.tuning.into()
  }

  pub fn get_start_fret(&self) -> Signal<usize> {
    self.start_fret.into()
  }

  pub fn get_end_fret(&self) -> Signal<usize> {
    self.end_fret.into()
  }

  pub fn get_config(&self) -> Signal<FretboardVisualConfig> {
    self.config.into()
  }

  pub fn get_fret_states(&self) -> Signal<FretStateSignals> {
    self.fret_states.into()
  }

  /// Update the fret states by merging new states into existing preallocated signals
  pub fn update_fret_states(&self, new_states: FretStateSignals) {
    // Merge into existing preallocated signals (no new signal creation during reactive updates).
    self.fret_states.with(|existing| {
      for (coord, state_signal) in new_states.into_iter() {
        if let Some(dest) = existing.get(&coord) {
          if dest.get() != state_signal.get() {
            dest.set(state_signal.get());
          }
        }
      }
    });
  }

  /// Calculate the note at a specific fret position
  pub fn note_from_fret(&self, coord: FretCoord) -> Note {
    self.tuning.with_untracked(|tuning| {
      if let Some(string_note) = tuning.get(coord.string_idx as usize) {
        string_note.add_steps(coord.fret_idx as usize)
      } else {
        Note::C // Fallback for invalid string
      }
    })
  }

  pub fn set_fret_state(&self, coord: FretCoord, state: FretState) {
    self.fret_states.with_untracked(|fret_states| {
      if let Some(sig) = fret_states.get(&coord) {
        sig.set(state);
      }
    });
  }

  pub fn get_min_fret(&self) -> Signal<usize> {
    let start_fret = self.start_fret.clone();
    let config = self.config.clone();

    Signal::derive(move || {
      start_fret
        .get_untracked()
        .saturating_sub(config.get_untracked().extra_frets.get())
    })
  }

  pub fn get_max_visible_fret(&self) -> Signal<usize> {
    let end_fret = self.end_fret;
    let config = self.config;
    let extra_frets = Signal::derive(move || config.get().extra_frets.get());

    Signal::derive(move || end_fret.get() + extra_frets.get())
  }
}
