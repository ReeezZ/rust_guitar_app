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
  pub start_fret: RwSignal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: RwSignal<usize>,
  /// Tuning of the guitar strings, first index is the lowest string (6th string)
  pub tuning: RwSignal<Vec<Note>>,
  /// Visual configuration for fretboard display properties
  pub config: RwSignal<FretboardVisualConfig>,
  /// States for each fret
  pub fret_states: RwSignal<FretStateSignals>,
  /// Optional callback for fret click events
  pub on_note_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
}

impl Default for FretboardModel {
  fn default() -> Self {
    Self {
      start_fret: RwSignal::new(1),
      end_fret: RwSignal::new(9),
      tuning: RwSignal::new(vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E]),
      config: RwSignal::new(FretboardVisualConfig::default()),
      on_note_clicked: RwSignal::new(None).into(),
      fret_states: RwSignal::new(HashMap::new()),
    }
  }
}

impl FretboardModel {
  pub fn get_num_frets(&self) -> usize {
    self.end_fret.get() - self.start_fret.get()
  }

  pub fn get_num_strings(&self) -> usize {
    self.tuning.get().len()
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
}
