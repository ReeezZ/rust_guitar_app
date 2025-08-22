use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;
use shared::Note;

use crate::fretboard::components::{base::FretState, visual_config::FretboardVisualConfig};

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
}
