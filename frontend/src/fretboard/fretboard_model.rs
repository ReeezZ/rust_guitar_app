use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;
use shared::{Note, Scale};

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
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  end_fret: Signal<usize>,
  /// Tuning of the guitar strings, first index is the lowest string (6th string)
  tuning: Signal<Vec<Note>>,
  /// Visual configuration for fretboard display properties
  config: Signal<FretboardVisualConfig>,
  /// States for each fret
  fret_states: RwSignal<FretStateSignals>,
  /// Optional callback for fret click events
  on_note_clicked: Signal<Option<Callback<FretClickEvent>>>,
}
pub fn default_tuning() -> Signal<Vec<Note>> {
  Signal::derive(move || vec![Note::E, Note::B, Note::G, Note::D, Note::A, Note::E])
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
    on_note_clicked: Signal<Option<Callback<FretClickEvent>>>,
  ) -> Self {
    let fret_states = RwSignal::new(get_preallocated_fret_states());
    Self {
      start_fret,
      end_fret,
      tuning,
      config,
      on_note_clicked,
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

  pub fn get_on_note_clicked(&self) -> Signal<Option<Callback<FretClickEvent>>> {
    self.on_note_clicked.into()
  }

  pub fn set_fret_states(&self, new_states: FretStateSignals) {
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

  fn get_min_fret(&self) -> Signal<usize> {
    let start_fret = self.start_fret.clone();
    let config = self.config.clone();

    Signal::derive(move || {
      start_fret
        .get_untracked()
        .saturating_sub(config.get_untracked().extra_frets.get())
    })
  }

  fn get_max_fret(&self) -> Signal<usize> {
    let end_fret = self.end_fret.clone();
    let config = self.config.clone();
    let extra_frets = Signal::derive(move || config.get().extra_frets.get());

    Signal::derive(move || end_fret.get() + extra_frets.get())
  }

  pub fn update_from_scale(&self, scale: Scale) {
    self
      .tuning
      .get()
      .iter()
      .enumerate()
      .for_each(|(string_idx, string_note)| {
        for fret_idx in self.get_min_fret().get()..=self.get_max_fret().get() {
          let coord = FretCoord {
            string_idx: string_idx as u8,
            fret_idx: fret_idx as u8,
          };
          let state = if fret_idx >= self.start_fret.get() && fret_idx <= self.end_fret.get() {
            let note_at_fret = string_note.add_steps(fret_idx);
            let state = if scale.root_note() == Some(note_at_fret) {
              FretState::Normal(FretStateColor::Green, note_at_fret.to_string())
            } else if scale.contains_note(note_at_fret) {
              FretState::Normal(FretStateColor::Blue, note_at_fret.to_string())
            } else {
              FretState::Hidden
            };
            state
          } else {
            FretState::Hidden
          };
          self.fret_states.with(|fret_states| {
            if let Some(sig) = fret_states.get(&coord) {
              sig.set(state);
            }
          });
        }
      });
  }
}

pub struct FretboardModelBuilder {
  start_fret: Option<Signal<usize>>,
  end_fret: Option<Signal<usize>>,
  tuning: Option<Signal<Vec<Note>>>,
  config: Option<Signal<FretboardVisualConfig>>,
  on_note_clicked: Option<Signal<Option<Callback<FretClickEvent>>>>,
}

impl FretboardModelBuilder {
  pub fn new() -> Self {
    Self {
      start_fret: None,
      end_fret: None,
      tuning: None,
      config: None,
      on_note_clicked: None,
    }
  }

  pub fn start_fret(mut self, start_fret: Signal<usize>) -> Self {
    self.start_fret = Some(start_fret);
    self
  }

  pub fn end_fret(mut self, end_fret: Signal<usize>) -> Self {
    self.end_fret = Some(end_fret);
    self
  }

  pub fn tuning(mut self, tuning: Signal<Vec<Note>>) -> Self {
    self.tuning = Some(tuning);
    self
  }

  pub fn config(mut self, config: Signal<FretboardVisualConfig>) -> Self {
    self.config = Some(config);
    self
  }

  pub fn on_note_clicked(
    mut self,
    on_note_clicked: Signal<Option<Callback<FretClickEvent>>>,
  ) -> Self {
    self.on_note_clicked = Some(on_note_clicked);
    self
  }

  pub fn from_options(
    start_fret: Option<Signal<usize>>,
    end_fret: Option<Signal<usize>>,
    tuning: Option<Signal<Vec<Note>>>,
    config: Option<Signal<FretboardVisualConfig>>,
    on_note_clicked: Option<Signal<Option<Callback<FretClickEvent>>>>,
  ) -> FretboardModel {
    FretboardModel::new(
      start_fret.unwrap_or_else(|| Signal::derive(move || 0)),
      end_fret.unwrap_or_else(|| Signal::derive(move || 12)),
      tuning.unwrap_or_else(default_tuning),
      config.unwrap_or_else(|| Signal::derive(move || FretboardVisualConfig::default())),
      on_note_clicked.unwrap_or_else(|| Signal::derive(move || None)),
    )
  }

  pub fn build(self) -> FretboardModel {
    Self::from_options(
      self.start_fret,
      self.end_fret,
      self.tuning,
      self.config,
      self.on_note_clicked,
    )
  }
}
