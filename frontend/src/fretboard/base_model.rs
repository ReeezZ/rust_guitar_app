use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;

use crate::fretboard::components::{base::FretState, visual_config::FretboardVisualConfig};

pub(crate) type FretStateSignals = HashMap<FretCoord, RwSignal<FretState>>;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub coord: FretCoord,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FretboardBaseModel {
  /// First fret in the active/playable range
  pub start_fret: RwSignal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: RwSignal<usize>,
  /// Number of guitar strings (default: 6)
  pub num_strings: RwSignal<u8>,
  /// Visual configuration for fretboard display properties
  pub config: RwSignal<FretboardVisualConfig>,
  /// Optional callback for fret click events
  pub on_fret_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
  /// States for each fret
  pub fret_states: RwSignal<FretStateSignals>,
}

pub trait FretboardBaseModelTrait {
  fn clear_fret_states(&self);
}

impl FretboardBaseModel {
  pub fn new(
    start_fret: RwSignal<usize>,
    end_fret: RwSignal<usize>,
    num_strings: RwSignal<u8>,
    config: RwSignal<FretboardVisualConfig>,
    on_fret_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
    fret_states: RwSignal<FretStateSignals>,
  ) -> Self {
    assert!(
      start_fret.get() < end_fret.get(),
      "Start fret must be less than or equal to end fret"
    );
    assert!(
      end_fret.get() <= 24,
      "End fret must be within a reasonable range (0-24)"
    );
    assert!(num_strings.get() > 0, "Number of strings must be positive");

    Self {
      start_fret,
      end_fret,
      num_strings,
      config,
      on_fret_clicked,
      fret_states,
    }
  }

  pub fn from_defaults() -> Self {
    Self {
      start_fret: RwSignal::new(1),
      end_fret: RwSignal::new(9),
      num_strings: RwSignal::new(6),
      config: RwSignal::new(FretboardVisualConfig::default()),
      on_fret_clicked: RwSignal::new(None),
      fret_states: RwSignal::new(HashMap::new()),
    }
  }
}

impl FretboardBaseModelTrait for FretboardBaseModel {
  fn clear_fret_states(&self) {
    self.fret_states.set(HashMap::new());
  }
}
