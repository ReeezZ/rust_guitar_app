use std::collections::HashMap;

use leptos::prelude::{Callback, Signal};

use crate::fretboard::{
  components::{base::FretState, visual_config::FretboardVisualConfig},
  with_notes_model::FretCoord,
};

pub(crate) type FretStateSignals = HashMap<FretCoord, Signal<FretState>>;

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub coord: FretCoord,
}

#[derive(Clone, Debug)]
pub struct FretboardBaseModel {
  /// First fret in the active/playable range
  pub start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: Signal<usize>,
  /// Visual configuration for fretboard display properties
  pub config: Signal<FretboardVisualConfig>,
  /// Optional callback for fret click events
  pub on_fret_clicked: Option<Callback<FretClickEvent>>,
  /// States for each fret
  pub fret_states: Signal<FretStateSignals>,
}
